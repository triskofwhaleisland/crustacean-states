//! Additional tools for making requests.

use crate::shards::NSRequest;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Response,
};
use std::{
    num::ParseIntError,
    ops::Add,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use thiserror::Error;

/// A client helper. Uses [`reqwest`] under the surface.
pub struct Client {
    client: reqwest::Client,
    state: Arc<Mutex<ClientState>>,
}

#[derive(Clone, Debug, Default)]
struct ClientState {
    rate_limiter: Option<RateLimits>,
    last_sent: Option<Instant>,
    send_after: Option<Instant>,
}

impl Client {
    /// Creates a new client.
    /// `user_agent` needs to be [`TryInto`]<[`HeaderValue`]>,
    /// which, as of [`reqwest`] 0.11.18, is implemented for `&[u8]`, `&String`, `&str`,
    /// `String`, and `Vec<u8>`.
    pub fn new<V>(user_agent: V) -> Self
    where
        V: TryInto<HeaderValue>,
        V::Error: Into<http::Error>,
    {
        Self {
            client: reqwest::Client::builder()
                .user_agent(user_agent)
                .build()
                .unwrap(),
            state: Arc::new(Mutex::new(ClientState::default())),
        }
    }

    /// Make a request of the API.
    ///
    /// If the last request was too recent, early-return [`ClientError::RateLimitedError`].
    ///
    /// If there was an error in the [`reqwest`] crate, return [`ClientError::ReqwestError`].
    // Note: this function cannot be tested because it is `async`.
    pub async fn get<U: NSRequest>(&self, request: U) -> Result<Response, ClientError> {
        // If the client was told that it should not send until some time after now,
        if let Some(t) = self
            .state
            .lock()
            .unwrap()
            .send_after
            .filter(|t| *t > Instant::now())
        {
            // Raise an error detailing when the request should have been sent.
            return Err(ClientError::RateLimitedError(t));
        }

        match self.client.get(request.as_url()).send().await {
            Ok(r) => {
                let mut state = self.state.lock().unwrap();
                state.rate_limiter = Some(RateLimits::new(r.headers())?);
                state.last_sent = Some(Instant::now());
                if let Some(ref r) = state.rate_limiter {
                    state.send_after = if r.remaining == 0 {
                        Some(r.reset)
                    } else {
                        r.retry_after
                    }
                    .map(|t| state.last_sent.unwrap().add(Duration::from_secs(t as u64)))
                }
                Ok(r)
            }
            Err(e) => Err(ClientError::ReqwestError { source: e }),
        }
    }

    /// Estimates the length of time to wait between each request to avoid a
    /// 429 Too Many Requests error.
    pub fn wait_duration(&self) -> Option<Duration> {
        self.state
            .lock()
            .unwrap()
            .rate_limiter
            .as_ref()
            .map(|r| Duration::from_secs_f64(r.remaining as f64 / r.reset as f64))
    }
}

/// Describes the various errors that may come about from using [`Client`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ClientError {
    /// An error relating to the internal [`reqwest::Client`] occurred.
    #[error("reqwest client failed")]
    ReqwestError {
        /// The parent error.
        #[from]
        source: reqwest::Error,
    },
    /// An error relating to converting raw [`HeaderValue`]s to `&str`s. This happens if a `HeaderValue`
    /// is not made solely of visible ASCII characters.
    ///
    /// If you get this,
    /// your response is probably malformed, and you shouldn't attempt to parse it further!
    #[error("could not convert to string")]
    ToStrError {
        /// The parent error.
        #[from]
        source: reqwest::header::ToStrError,
    },
    /// Every response should contain the `RateLimit-Policy`,
    /// `RateLimit-Limit`, `RateLimit-Remaining`, and `RateLimit-Reset` headers.
    /// If not, this error is raised.
    ///
    /// The response is probably not malformed if you have this error,
    /// as the RFC for standardization of these headers by the IETF is still an active Internet draft.
    /// [Here is the current draft.](https://datatracker.ietf.org/doc/draft-ietf-httpapi-ratelimit-headers/)
    #[error("couldn't find RateLimit-{0} in headers")]
    NoRateLimitElementError(String),
    /// The `RateLimit-Policy` header is unique as it should contain two values.
    /// If not, this error is raised.
    ///
    /// See the note for [`ClientError::NoRateLimitElementError`] for more details.
    #[error("couldn't parse RateLimit-Policy")]
    RateLimitPolicyError,
    /// Every `RateLimit` header should have an integer value associated with it.
    /// If it can't be parsed as an integer, this error is raised.
    ///
    /// The response is probably malformed if you have this error.
    #[error("couldn't parse as integer")]
    IntegerParseError {
        #[from]
        /// The parent error.
        source: ParseIntError,
    },
    /// If you shouldn't send a request until later, this error will rate-limit you.
    /// Your request is perfectly fine, wait until your timeout is over.
    #[error("rate limited until {0:?}")]
    RateLimitedError(Instant),
}

/// A simple tool to help with NationStates rate limits.
#[derive(Clone, Debug)]
pub struct RateLimits {
    // Policy and limits are currently disabled
    // because this part of the program is private and implementation will probably change.
    // ---
    // /// The number of requests that can be sent within a timeframe,
    // /// and how long that timeframe is in seconds.
    // - `policy`: (u8, u8),
    // /// The number of requests that can be sent in this timeframe.
    // /// Always equal to `policy.0`.
    // - `limit`: u8,
    // ---
    remaining: u8,
    reset: u8,
    retry_after: Option<u8>,
}

impl RateLimits {
    /// Creates new RateLimits.
    fn new(headers: &HeaderMap) -> Result<Self, ClientError> {
        // let raw_policy: Vec<u8> = headers
        //     .get("RateLimit-Policy")
        //     .ok_or_else(|| ClientError::NoRateLimitElementError("Policy".to_string()))?
        //     .to_str()?
        //     .split(";w=")
        //     .take(2)
        //     .filter_map(|x| x.parse().ok())
        //     .collect();
        // let policy: (u8, u8) = (
        //     *raw_policy
        //         .first()
        //         .ok_or_else(|| ClientError::RateLimitPolicyError)?,
        //     *raw_policy
        //         .get(1)
        //         .ok_or_else(|| ClientError::RateLimitPolicyError)?,
        // );
        // let limit: u8 = headers
        //     .get("RateLimit-Limit")
        //     .ok_or_else(|| ClientError::NoRateLimitElementError("Limit".to_string()))?
        //     .to_str()?
        //     .parse()?;
        let remaining: u8 = headers
            .get("RateLimit-Remaining")
            .ok_or_else(|| ClientError::NoRateLimitElementError("Remaining".to_string()))?
            .to_str()?
            .parse()?;
        let reset: u8 = headers
            .get("RateLimit-Reset")
            .ok_or_else(|| ClientError::NoRateLimitElementError("Reset".to_string()))?
            .to_str()?
            .parse()?;
        let retry_after: Option<u8> = match headers.get("Retry-After") {
            Some(value) => Some(value.to_str()?.parse()?),
            None => None,
        };

        Ok(RateLimits {
            // policy,
            // limit,
            remaining,
            reset,
            retry_after,
        })
    }

    /// The number of requests that can still be sent in this timeframe.
    pub fn remaining(&self) -> u8 {
        self.remaining
    }

    /// The number of seconds until the timeframe resets.
    pub fn reset(&self) -> u8 {
        self.reset
    }

    /// The number of seconds until a request can be sent.
    /// If a RateLimit-Retry-After header was not sent, returns `None`.
    pub fn retry_after(&self) -> Option<u8> {
        self.retry_after
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_rate_limits() {
        use crate::client::RateLimits;
        use reqwest::header::{HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();
        headers.insert("RateLimit-Remaining", HeaderValue::from(11));
        headers.insert("RateLimit-Reset", HeaderValue::from(25));

        let limits = RateLimits::new(&headers).unwrap();
        assert_eq!(limits.remaining(), 11);
        assert_eq!(limits.reset(), 25);
        assert_eq!(limits.retry_after(), None);
    }

    #[test]
    fn rate_limits_with_retry_after() {
        use crate::client::RateLimits;
        use reqwest::header::{HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();
        headers.insert("RateLimit-Remaining", HeaderValue::from(11));
        headers.insert("RateLimit-Reset", HeaderValue::from(25));
        headers.insert("Retry-After", HeaderValue::from(7));

        let limits = RateLimits::new(&headers).unwrap();
        assert_eq!(limits.remaining(), 11);
        assert_eq!(limits.reset(), 25);
        assert_eq!(limits.retry_after(), Some(7));
    }
}
