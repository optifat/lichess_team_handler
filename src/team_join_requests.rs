use serde::Deserialize;
use std::collections::{HashMap, HashSet};

// https://lichess.org/api#tag/Teams/operation/teamRequests

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Request {
    date: u64,
    message: String,
    team_id: String,
    user_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct User {
    created_at: u64,
    id: String,
    perfs: HashMap<String, Perf>,
    play_time: HashMap<String, u64>,
    profile: HashMap<String, String>,
    sent_at: u64,
    username: String,
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Perf {
    games: u32,
    prog: u32,
    prov: bool,
    rating: u32,
    rd: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TeamJoinResponse {
    request: Request,
    user: User,
}

pub async fn get_join_requests(
    team_id: &str,
    token: &str,
) -> anyhow::Result<Vec<TeamJoinResponse>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://lichess.org/api/team/{}/requests", team_id))
        .bearer_auth(token)
        .send()
        .await?;
    Ok(resp.json::<Vec<TeamJoinResponse>>().await?)
}

pub async fn handle_join_requests(
    team_id: &str,
    token: &str,
    requests: &Vec<TeamJoinResponse>,
    cheaters: &HashSet<String>,
) -> anyhow::Result<(u32, u32)> {
    let mut approved = 0;
    let mut declined = 0;
    futures::future::join_all(requests.iter().map(|user| {
        let user_id = &user.user.id;
        let client = reqwest::Client::new();
        match cheaters.get(user_id) {
            Some(_) => {
                declined += 1;
                #[cfg(feature = "full_info")]
                println!("{}: Declined ❌", user_id);
                client
                    .post(format!(
                        "https://lichess.org/api/team/{}/request/{}/decline",
                        team_id, user_id
                    ))
                    .bearer_auth(token)
                    .send()
            }
            None => {
                approved += 1;
                #[cfg(feature = "full_info")]
                println!("{}: Approved ✅", user_id);
                client
                    .post(format!(
                        "https://lichess.org/api/team/{}/request/{}/accept",
                        team_id, user_id
                    ))
                    .bearer_auth(token)
                    .send()
            }
        }
    }))
    .await;
    Ok((approved, declined))
}
