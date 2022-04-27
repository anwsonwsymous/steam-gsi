use std::collections::HashMap;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use crate::server::GSIRequest;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Auth {
    token: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Provider {
    name: Option<String>,
    appid: Option<u32>,
    version: Option<u32>,
    timestamp: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Player {
    steamid: Option<String>,
    name: Option<String>,
    activity: Option<String>,
    kills: Option<u16>,
    deaths: Option<u16>,
    assists: Option<u16>,
    last_hits: Option<u16>,
    denies: Option<u32>,
    kill_streak: Option<u16>,
    commands_issued: Option<u32>,
    kill_list: Option<HashMap<String, Value>>,
    team_name: Option<String>,
    gold: Option<u32>,
    gold_reliable: Option<u32>,
    gold_unreliable: Option<u32>,
    gold_from_hero_kills: Option<u32>,
    gold_from_creep_kills: Option<u32>,
    gold_from_income: Option<u32>,
    gold_from_shared: Option<u32>,
    gpm: Option<u32>,
    xpm: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Map {
    name: Option<String>,
    matchid: Option<String>,
    game_time: Option<u32>,
    clock_time: Option<i32>,
    daytime: Option<bool>,
    nightstalker_night: Option<bool>,
    game_state: Option<String>,
    paused: Option<bool>,
    win_team: Option<String>,
    customgamename: Option<String>,
    ward_purchase_cooldown: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DotaRequest {
    provider: Option<Provider>,
    player: Option<Player>,
    map: Option<Map>,
    auth: Option<Auth>,
}

impl GSIRequest for DotaRequest {
    fn authenticate(&self, trusted_token: &str) -> bool {
        if let Some(untrusted_token) = &self.auth {
            trusted_token == untrusted_token.token
        } else {
            false
        }
    }
}