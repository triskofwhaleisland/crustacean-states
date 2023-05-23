use reqwest::header::HeaderMap;
use reqwest::Response;
use std::error::Error;

const BASE_URL: &str = "https://www.nationstates.net/cgi-bin/api.cgi?";

pub async fn client_request(
    client: &reqwest::Client,
    request: &str,
) -> Result<Response, reqwest::Error> {
    client
        .get(format!("{BASE_URL}{request}"))
        .header(
            reqwest::header::USER_AGENT,
            "Project Iron Oxide; NS nation: Aramos",
        )
        .send()
        .await
}

pub struct RateLimits {
    pub policy: (u8, u8),
    // requests / second
    pub limit: u8,
    // # requests
    pub remaining: u8,
    // # requests
    pub reset: u8,
    // seconds
    pub retry_after: Option<u8>, // seconds
}

impl RateLimits {
    pub fn new(headers: &HeaderMap) -> Result<Self, Box<dyn Error>> {
        let raw_policy: Vec<u8> = headers
            .get("RateLimit-Policy")
            .ok_or("Couldn't find RateLimit-Policy in headers")?
            .to_str()?
            .split(";w=")
            .take(2)
            .filter_map(|x| x.parse().ok())
            .collect();
        let policy: (u8, u8) = (
            *raw_policy
                .first()
                .ok_or("Could not parse RateLimit-Policy")?,
            *raw_policy
                .get(1)
                .ok_or("Could not parse RateLimit-Policy")?,
        );
        let limit: u8 = headers
            .get("RateLimit-Limit")
            .ok_or("Couldn't find RateLimit-Policy in headers")?
            .to_str()?
            .parse()?;
        let remaining: u8 = headers
            .get("RateLimit-Remaining")
            .ok_or("Couldn't find RateLimit-Remaining in headers")?
            .to_str()?
            .parse()?;
        let reset: u8 = headers
            .get("RateLimit-Reset")
            .ok_or("Couldn't find RateLimit-Reset in headers")?
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
