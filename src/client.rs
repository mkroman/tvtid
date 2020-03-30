use crate::{Channel, Error};

use chrono::{Date, Utc};

pub struct Schedule;

pub struct Client {
    http_client: reqwest::Client,
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
    pub async fn get_channels(&self) -> Result<Option<Vec<Channel>>, Error> {
        unimplemented!()
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
}
