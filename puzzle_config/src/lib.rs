mod config;
mod error;
mod json;

pub use config::area::AreaConfig;
pub use config::area::AreaValueFormatter;
pub use config::board::BoardConfig;
pub use config::collection::PuzzleConfigCollection;
pub use config::puzzle::PuzzleConfig;
pub use config::target::{Target, TargetIndex, TargetTemplate};
pub use config::tile::TileConfig;
pub use error::ReadError;
use regex::Regex;

/// Generates a regex pattern string to match the start of a JSON config with the given version.
fn start_regex(version: u32) -> String {
    format!(r#"^\s*\{{\s*"config_version"\s*:\s*{}\s*,"#, version)
}

/// Checks if the given JSON string matches the expected version.
fn matches_version(json_str: &str, version: u32) -> bool {
    Regex::new(&start_regex(version))
        .map(|re| re.is_match(json_str))
        .unwrap()
}

pub fn load_puzzle_collection_from_json(
    json_str: &str,
) -> Result<PuzzleConfigCollection, ReadError> {
    if !matches_version(json_str, 1) {
        return Err(ReadError::UnsupportedVersion);
    }

    json::load_puzzle_collection_from_json(json_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_regex() {
        let regex = start_regex(1);
        assert_eq!(regex, r#"^\s*\{\s*"config_version"\s*:\s*1\s*,"#);
    }

    #[test]
    fn test_matches_version() {
        let test_str = r#"{
            "config_version": 1,
            "other_field": "value"
        }"#;

        assert!(matches_version(test_str, 1));
    }

    #[test]
    fn test_matches_version_wrong_version() {
        let test_str = r#"{
            "config_version": 11,
            "other_field": "value"
        }"#;

        assert!(!matches_version(test_str, 1));
    }

    #[test]
    fn test_matches_version_wrong_format() {
        let test_str = r#"{
            "other_field": "value",
            "config_version": 1
        }"#;

        assert!(!matches_version(test_str, 1));
    }
}
