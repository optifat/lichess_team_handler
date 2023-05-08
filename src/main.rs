#[deny(warnings)]
mod cheaters_list;
mod config;
mod team_join_requests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::read_config_file()?;
    let team_id = &config.team;
    let authorization_token = &config.authorization_token;
    let blacklist_file = &config.blacklist_file;

    #[cfg(feature = "full_info")]
    {
        println!("Team id: {}", team_id);
        println!("Auth token: {}", authorization_token);
    }

    loop {
        println!("Getting join requests");
        let join_requests =
            team_join_requests::get_join_requests(authorization_token, team_id).await?;
        println!("Got all join requests: {} in total", join_requests.len());

        #[cfg(feature = "full_info")]
        println!("Requests: {:#?}", join_requests);

        let cheaters = cheaters_list::read_blacklist_file(blacklist_file)?;

        let (approved, declined) = team_join_requests::handle_join_requests(
            authorization_token,
            team_id,
            &join_requests,
            &cheaters,
        )
        .await?;

        println!(
            "Handled all join requests: {} approved, {} declined",
            approved, declined
        );

        let waiting_time = tokio::time::Duration::from_secs(60);
        println!("Sleeping for {} secs", waiting_time.as_secs());
        tokio::time::sleep(waiting_time).await;
    }
}
