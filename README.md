# Lichess team handler

This program handles requests to join your lichess team.

## Building

General build:

`cargo build --release`

If you want more logs while running:

`cargo build --release --features full_info`

## Authorization token creation

This program needs authorization token, created by an admin or manager of your lichess team.

### Prefilled creation
Use [this link](https://lichess.org/account/oauth/token/create?scopes[]=team:read&scopes[]=team:lead&description=Team+handler+token) to open lichess auth token creation page with prefilled form.

### Manual creation
To create auth token, go to `Preferences -> API token access -> New access token`

You token will need these rights:
1. Read private team information
2. Manage teams you lead: send PMs, kick members

Although you can add other rights too, I highly recommend to limit yourself to those two only.

You can find more info on lichess auth tokens [here](https://lichess.org/api#section/Introduction/Authentication)

## Running

This program needs `config.json` file with the next fields:

1. `blacklistFile` -- path to the [blacklist file](#blacklist-file)
2. `team` -- your team id. You can find it in your club url address (`https://lichess.org/team/{team-id}`)
3. `authorizationToken` -- your authorization token. [How to create authorization token](#authorization-token-creation)

If you need an example, see `config-example.json`

### Blacklist file
Blacklist file -- file with one user per line, whose join request you want to decline.

Just fill it with one user non-grata per line (see `blacklist-example` file) 