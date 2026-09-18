use crate::Static;
use anyhow::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub server_name: String,
    pub map: String,
    pub experiences: Vec<String>,
    pub lighting: String,
    pub score_tick: ScoreTick,
    pub players: Players,
    pub faction_scores: Vec<FactionScore>,
    pub alternator: String,
    pub rotation: Rotation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreTick {
    pub current: i64,
    pub min: i64,
    pub max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Players {
    pub current: i64,
    pub max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionScore {
    pub name: String,
    pub color_hex: String,
    pub score: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    pub now_index: i64,
    pub next_index: i64,
}

pub async fn get_status(statics: &Static) -> Result<Status> {
    let client = reqwest::Client::new();
    let url = Url::parse(&format!("http://{}:{}/v1/status", statics.ip, statics.port)[..]).unwrap();
    let res = client
        .get(url)
        .header("Authorization", &format!("Bearer {}", statics.password)[..])
        .send()
        .await?
        .json::<Status>()
        .await?;
    Ok(res)
}
