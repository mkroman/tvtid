use chrono::{serde::ts_nanoseconds, DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Structure that contains information about a scheduled program
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
    fn program_has_duration(program: Program) {
        let duration = program.duration();

        assert!(duration > Duration::zero());
    }
}
