use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse {
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize() {
        let json = json!({"version": "0.6.2"});
        let response: VersionResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.version, "0.6.2");
    }

    #[test]
    fn round_trip() {
        let response = VersionResponse {
            version: "0.6.2".to_string(),
        };
        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json, json!({"version": "0.6.2"}));
    }
}
