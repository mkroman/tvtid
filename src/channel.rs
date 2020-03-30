use serde::{Deserialize, Serialize};

/// Structure that contains information about a channel
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
    fn it_deserializes_channel(channel: Channel) {
        assert_eq!(channel.id(), "1");
        assert_eq!(channel.title(), "DR1");
        assert!(!channel.icon().is_empty());
        assert!(!channel.logo().is_empty());
        assert!(!channel.svg_logo().is_empty());
    }
}
