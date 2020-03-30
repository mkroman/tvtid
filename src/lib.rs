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
    /// Indicates whether this is available as a VOD
    #[serde(rename = "availableAsVod")]
    available_as_vod: bool,
    /// Indicates whether this is a rerun
    rerun: bool,
    /// Indicates whether this is a premiering episode
    premiere: bool,
    /// Indicates whether this is a live program
    live: bool,
    /// The time the program starts, as UTC time.
    #[serde(rename = "start", with = "ts_nanoseconds")]
    starts_at: DateTime<Utc>,
    /// The time the program ends, as UTC time.
    #[serde(rename = "stop", with = "ts_nanoseconds")]
    ends_at: DateTime<Utc>,
}

impl Program {
    /// Returns the unique ID of the program
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the title of the program
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the list of categories that applies to the program
    pub fn categories(&self) -> &Vec<String> {
        &self.categories
    }

    /// Returns whether this program is available as VOD
    pub fn available_as_vod(&self) -> bool {
        self.available_as_vod
    }

    /// Returns whether this program is a rerun
    pub fn rerun(&self) -> bool {
        self.rerun
    }

    /// Returns whether this program is premiering episode
    pub fn premiere(&self) -> bool {
        self.premiere
    }

    /// Returns whether this is a live program
    pub fn live(&self) -> bool {
        self.live
    }

    /// Returns the duration of the program
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

impl Channel {
    /// Returns the unique ID of the channel
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the channel
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the url for the logo of the channel
    pub fn logo(&self) -> &str {
        &self.logo
    }

    /// Returns the url for the SVG logo of the channel
    pub fn svg_logo(&self) -> &str {
        &self.svg_logo
    }

    /// Returns the url for the icon of the channel
    pub fn icon(&self) -> &str {
        &self.icon
    }

    /// Returns the sorting index
    pub fn sort(&self) -> u32 {
        self.sort
    }
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
        assert_eq!(program.title(), "Fantastiske floder: Mississippifloden");
        assert_eq!(program.id(), "20605495");
        assert_eq!(program.available_as_vod(), false);
        assert!(program.rerun());
        assert_eq!(program.premiere(), false);
        assert_eq!(program.live(), false);
        assert!(program.categories().len() > 0);
    }

    #[rstest]
    fn it_deserializes_channel(channel: Channel) {
        assert_eq!(channel.id(), "1");
        assert_eq!(channel.title(), "DR1");
        assert!(!channel.icon().is_empty());
        assert!(!channel.logo().is_empty());
        assert!(!channel.svg_logo().is_empty());
    }

    #[rstest]
    fn program_has_duration(program: Program) {
        let duration = program.duration();

        assert!(duration > Duration::zero());
    }
}
