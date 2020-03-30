use crate::Error;
use chrono::{Date, Utc};

pub struct Schedule;

pub struct Client {
    http_client: reqwest::Client,
}

pub trait ChannelId {
    fn channel_id(&self) -> &str;
}

impl ChannelId for &str {
    fn channel_id(&self) -> &str {
        self.as_ref()
    }
}

impl ChannelId for crate::Channel {
    fn channel_id(&self) -> &str {
        self.id()
    }
}

impl Client {
    /// Creates a new `Client`
    pub fn new() -> Client {
        Client {
            http_client: reqwest::Client::new(),
        }
    }

    /// Returns the schedule for the given `channel` on the given `date`
    pub fn get_schedule(
        &self,
        channel: impl ChannelId,
        _date: Date<Utc>,
    ) -> Result<Option<Schedule>, ()> {
        unimplemented!()
    }

    /// Returns a list of schedules for the given `channels` on the given `date`
    pub fn get_schedules(
        &self,
        channels: Vec<impl ChannelId>,
        _date: Date<Utc>,
    ) -> Result<Option<Schedule>, ()> {
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
