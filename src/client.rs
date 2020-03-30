use crate::{Channel, Error};

use chrono::{Date, Utc};
use serde::Deserialize;

pub struct Schedule;

pub struct Client {
    http_client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
struct ChannelListResponse {
    channels: Vec<Channel>,
}

impl Client {
    /// Creates a new `Client`
    pub fn new() -> Client {
        Client {
            http_client: reqwest::Client::new(),
        }
    }

    /// Returns the schedule for the given `channel` on the given `date`
    pub async fn get_schedule(
        &self,
        _channel_id: &str,
        _date: Date<Utc>,
    ) -> Result<Option<Schedule>, Error> {
        unimplemented!()
    }

    pub async fn get_schedules(
        &self,
        _channel_ids: &[&str],
        _date: Date<Utc>,
    ) -> Result<Option<Schedule>, Error> {
        unimplemented!()
    }

    /// Returns the list of available channels
    pub async fn get_channels(&self) -> Result<Vec<Channel>, Error> {
        let response = self
            .http_client
            .get("https://tvtid-api.api.tv2.dk/api/tvtid/v1/schedules/channels")
            .send()
            .await?
            .json::<ChannelListResponse>()
            .await?;

        Ok(response.channels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn it_instantiates() {
        let _client = Client::new();
    }

    #[tokio::test]
    async fn it_gets_channels() {
        let client = Client::new();
        let channels = client.get_channels().await.expect("channel list");

        assert!(channels.len() > 0);
    }
}
