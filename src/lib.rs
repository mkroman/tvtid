use chrono::{serde::ts_nanoseconds, DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Structure that contains information for a scheduled program
#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    /// Unique id for the program
    id: String,
    /// The title of the program
    title: String,
    /// List of categories that applies to the program
    categories: Vec<String>,
    /// Returns whether this is available as a VOD
    #[serde(rename = "availableAsVod")]
    available_as_vod: bool,
    /// Returns whether this is a rerun
    rerun: bool,
    /// Returns whether this is a premiering episode
    premiere: bool,
    /// Returns whether this is a live program
    live: bool,
    /// The time the program starts, as UTC time.
    #[serde(rename = "start", with = "ts_nanoseconds")]
    starts_at: DateTime<Utc>,
    /// The time the program ends, as UTC time.
    #[serde(rename = "stop", with = "ts_nanoseconds")]
    ends_at: DateTime<Utc>,
}

impl Program {
    pub fn duration(&self) -> Duration {
        self.ends_at - self.starts_at
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The unique ID of the channel
    id: String,
    /// The name of the channel
    title: String,
    /// An url to an icon for the channel
    icon: String,
    /// An url to a logo for the channel
    logo: String,
    /// An url to a logo for the channel in SVG format
    #[serde(rename = "svgLogo")]
    svg_logo: String,
    /// Sorting index
    sort: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn program() -> Program {
        let json = r#"
            {
              "stop": 1585458000,
              "start": 1585454700,
              "categories": [
                "Kultur og Natur",
                "Programmer"
              ],
              "id": "20605495",
              "title": "Fantastiske floder: Mississippifloden",
              "availableAsVod": false,
              "rerun": true,
              "premiere": false,
              "live": false
            }
        "#;

        serde_json::from_str(&json).unwrap()
    }

    #[fixture]
    fn channel() -> Channel {
        let json = r#"
            {
              "id": "1",
              "title": "DR1",
              "icon": "https://epg-images.tv2.dk/channellogos/icon/1.png",
              "logo": "https://epg-images.tv2.dk/channellogos/logo/1.png",
              "svgLogo": "https://epg-images.tv2.dk/channellogos/svg/1.svg",
              "sort": 1
            }
        "#;

        serde_json::from_str(&json).unwrap()
    }

    #[rstest]
    fn it_deserializes_program(program: Program) {
        assert_eq!(program.title, "Fantastiske floder: Mississippifloden");
        assert_eq!(program.id, "20605495");
        assert_eq!(program.available_as_vod, false);
        assert!(program.rerun);
        assert_eq!(program.premiere, false);
        assert_eq!(program.live, false);
        assert!(program.categories.len() > 0);
    }

    #[rstest]
    fn it_deserializes_channel(channel: Channel) {
        assert_eq!(channel.id, "1");
        assert_eq!(channel.title, "DR1");
        assert!(!channel.icon.is_empty());
        assert!(!channel.logo.is_empty());
        assert!(!channel.svg_logo.is_empty());
    }

    #[rstest]
    fn program_has_duration(program: Program) {
        let duration = program.duration();

        assert!(duration > Duration::zero());
    }
}
