//! Additional tools for making requests.

use crate::shards::NSRequest;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Response;
use std::num::ParseIntError;
use std::ops::Add;
use std::time::{Duration, Instant};
use thiserror::Error;

/// A client helper. Uses [`reqwest`] under the surface.
pub struct Client(reqwest::Client);

#[derive(Clone, Debug, Default)]
pub struct ClientState {
    rate_limiter: Option<RateLimits>,
    last_sent: Option<Instant>,
    send_after: Option<Instant>,
}

impl Client {
    /// Creates a new client.
    /// `user_agent` needs to be [`TryInto`]<[`HeaderValue`]>,
    /// which, as of [`reqwest`] 0.11.18, is implemented for `&[u8]`, `&String`, `&str`,
    /// `String`, and `Vec<u8>`.
    pub fn new<V>(user_agent: V) -> Result<Self, ClientError>
    where
        V: TryInto<HeaderValue>,
        V::Error: Into<http::Error>,
    {
        Ok(Self(
            reqwest::Client::builder().user_agent(user_agent).build()?,
        ))
    }

    pub fn with_default_state(self) -> (Self, ClientState) {
        (self, ClientState::default())
    }

    /// Make a request of the API.
    ///
    /// Note that this method requires the [`Client`] to be `mut`.
    /// This is because `Client`s store important information about the previous request in them.
    /// For example, it allows this function to return a [`ClientError::RateLimitedError`]
    /// if it knows the last request was too recent.
    ///
    /// If there was an error in the [`reqwest`] crate,
    /// [`ClientError::ReqwestError`] will be returned.
    pub async fn get<U: NSRequest>(
        &self,
        request: U,
        state: &mut ClientState,
    ) -> Result<Response, ClientError> {
        // If the client was told that it should not send until some time after now,
        if state.send_after.is_some_and(|t| t > Instant::now()) {
            // Raise an error detailing when the request should have been sent.
            Err(ClientError::RateLimitedError(state.send_after.unwrap()))
        } else {
            match self.0.get(request.as_url()).send().await {
                Ok(r) => {
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
    }
}

impl ClientState {
    pub fn new() -> Self {
        Self::default()
    }

    /// A guide on how long to wait between requests.
    pub fn wait_duration(&self) -> Option<Duration> {
        self.rate_limiter
            .as_ref()
            .map(|r| Duration::from_secs_f64(r.remaining as f64 / r.reset as f64))
    }
}

/// Describes the various errors that may come about from using [`Client`].
#[derive(Debug, Error)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum ClientError {
    /// An error relating to the internal [`reqwest::Client`] occurred.
    #[error("reqwest client failed")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },
    /// An error relating to converting raw [`HeaderValue`]s to `&str`s. This happens if a `HeaderValue`
    /// is not made solely of visible ASCII characters.
    ///
    /// If you get this,
    /// your response is probably malformed and you should not attempt to parse it further!
    #[error("could not convert to string")]
    ToStrError {
        #[from]
        source: reqwest::header::ToStrError,
    },
    /// Every response should contain the `RateLimit-Policy`,
    /// `RateLimit-Limit`, `RateLimit-Remaining`, and `RateLimit-Reset` headers.
    /// If not, this error is raised.
    ///
    /// The response is probably not malformed if you have this error,
    /// as the RFC for standardization of these headers by the IETF is still an active Internet draft.
    /// [This link should take you to the current draft.](https://datatracker.ietf.org/doc/draft-ietf-httpapi-ratelimit-headers/)
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
        source: ParseIntError,
    },
    /// If you shouldn't send a request until later, you will be rate-limited by this error.
    /// Your request is perfectly fine,
    #[error("rate limited until {0:?}")]
    RateLimitedError(Instant),
}

/// A simple tool to help with NationStates rate limits.
#[derive(Clone, Debug)]
pub struct RateLimits {
    // policy and limits are currently disabled
    // because this part of the program is private and implementation will probably change.

    // ---
    // /// The number of requests that can be sent within a timeframe,
    // /// and how long that timeframe is in seconds.
    // policy: (u8, u8),
    // /// The number of requests that can be sent in this timeframe.
    // /// Always equal to `policy.0`.
    // limit: u8,
    // ---
    /// The number of requests that can still be sent in this timeframe.
    remaining: u8,
    /// The number of seconds until the timeframe resets.
    reset: u8,
    /// The number of seconds until a request can be sent.
    /// (If a RateLimit-Retry-After header was not sent, `retry_after` will store `None`.)
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
}
