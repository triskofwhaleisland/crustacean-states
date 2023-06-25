//! Additional tools for making requests.

use reqwest::header::HeaderMap;
use reqwest::{IntoUrl, Response};
use std::num::ParseIntError;
use std::ops::Add;
use std::time::{Duration, Instant};
use thiserror::Error;

pub struct Client {
    inner: reqwest::Client,
    rate_limiter: Option<RateLimits>,
    last_sent: Option<Instant>,
    send_after: Option<Instant>,
}

impl Client {
    pub fn new(user_agent: String) -> Result<Self, ClientError> {
        Ok(Self {
            inner: reqwest::Client::builder().user_agent(user_agent).build()?,
            rate_limiter: None,
            last_sent: None,
            send_after: None,
        })
    }
    pub async fn get<U: IntoUrl>(&mut self, request: U) -> Result<Response, ClientError> {
        // If the client was told that it should not send until some time after now,
        if self.send_after.is_some_and(|t| t > Instant::now()) {
            // Raise an error detailing when the request should have been sent.
            Err(ClientError::RateLimitedError(self.send_after.unwrap()))
        } else {
            match self.inner.get(request).send().await {
                Ok(r) => {
                    self.rate_limiter = Some(RateLimits::new(r.headers())?);
                    self.last_sent = Some(Instant::now());
                    if let Some(t) = self.rate_limiter.as_ref().unwrap().retry_after {
                        self.send_after =
                            Some(self.last_sent.unwrap().add(Duration::from_secs(t as u64)));
                    }
                    Ok(r)
                }
                Err(e) => Err(ClientError::ReqwestError { source: e }),
            }
        }
    }
}

#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum ClientError {
    #[error("reqwest client failed")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },
    #[error("could not convert to string")]
    ToStrError {
        #[from]
        source: reqwest::header::ToStrError,
    },
    #[error("couldn't find RateLimit-{0} in headers")]
    NoRateLimitElementError(String),
    #[error("couldn't parse RateLimit-Policy")]
    RateLimitPolicyError,
    #[error("couldn't parse as integer")]
    IntegerParseError {
        #[from]
        source: ParseIntError,
    },
    #[error("rate limited until {0:?}")]
    RateLimitedError(Instant),
}

/// A simple tool to help with NationStates rate limits.
#[derive(Debug)]
pub struct RateLimits {
    /// The number of requests that can be sent within a timeframe,
    /// and how long that timeframe is in seconds.
    pub policy: (u8, u8),
    /// The number of requests that can be sent in this timeframe. Always equal to `policy.0`.
    pub limit: u8,
    /// The number of requests that can still be sent in this timeframe.
    pub remaining: u8,
    /// The number of seconds until the timeframe resets.
    pub reset: u8,
    /// The number of seconds until a request can be sent.
    /// (If a RateLimit-Retry-After header was not sent, `retry_after` will store `None`.)
    pub retry_after: Option<u8>,
}

impl RateLimits {
    pub fn new(headers: &HeaderMap) -> Result<Self, ClientError> {
        let raw_policy: Vec<u8> = headers
            .get("RateLimit-Policy")
            .ok_or_else(|| ClientError::NoRateLimitElementError("Policy".to_string()))?
            .to_str()?
            .split(";w=")
            .take(2)
            .filter_map(|x| x.parse().ok())
            .collect();
        let policy: (u8, u8) = (
            *raw_policy
                .first()
                .ok_or_else(|| ClientError::RateLimitPolicyError)?,
            *raw_policy
                .get(1)
                .ok_or_else(|| ClientError::RateLimitPolicyError)?,
        );
        let limit: u8 = headers
            .get("RateLimit-Limit")
            .ok_or_else(|| ClientError::NoRateLimitElementError("Limit".to_string()))?
            .to_str()?
            .parse()?;
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
            policy,
            limit,
            remaining,
            reset,
            retry_after,
        })
    }
    pub fn wait_until_next_request(&self) -> f64 {
        self.remaining as f64 / self.reset as f64
    }
}
