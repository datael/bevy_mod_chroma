use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SessionInfo {
    #[serde(rename(deserialize = "sessionid"))]
    _session_id: u32,
    uri: String,
}
