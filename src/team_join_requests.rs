use reqwest::Response;
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
    play_time: HashMap<String, u64>,
    profile: Option<HashMap<String, String>>,
    seen_at: u64,
    username: String,
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TeamJoinRequest {
    request: Request,
    user: User,
}

pub async fn get_join_requests(team_id: &str, token: &str) -> anyhow::Result<Vec<TeamJoinRequest>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://lichess.org/api/team/{}/requests", team_id))
        .bearer_auth(token)
        .send()
        .await?;
    let join_requests = resp.json::<Vec<TeamJoinRequest>>().await?;

    Ok(join_requests
        .into_iter()
        .filter(|request| request.request.team_id == team_id)
        .collect())
}

pub async fn handle_join_requests(
    team_id: &str,
    token: &str,
    requests: &Vec<TeamJoinRequest>,
    cheaters: &HashSet<String>,
) -> anyhow::Result<(u32, u32)> {
    let mut approved = 0;
    let mut declined = 0;
    let client = reqwest::Client::new();

    futures::future::join_all(requests.iter().map(|user| {
        let user_id = &user.user.id;
        let verdict = if cheaters.contains(user_id) {
            declined += 1;
            #[cfg(feature = "full_info")]
            println!("{}: Declined ❌", user_id);
            "decline"
        } else {
            approved += 1;
            #[cfg(feature = "full_info")]
            println!("{}: Approved ✅", user_id);
            "accept"
        };
        client
            .post(format!(
                "https://lichess.org/api/team/{}/request/{}/{}",
                team_id, user_id, verdict
            ))
            .bearer_auth(token)
            .send()
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<Response>, _>>()?;

    Ok((approved, declined))
}
