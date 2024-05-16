# holou

<img src="https://raw.githubusercontent.com/eniraa/holou/master/holou.png" width="300" />

## Description
This Discord bot is a 4X game based on the Great War from [No Game No Life](http://ngnl.jp/).

## Usage

Invite the bot [here](https://discord.com/oauth2/authorize?client_id=1240348980362350672). See below for self-hosting.

### Kubernetes

TODO

### Docker Compose

1. Create a `docker-compose.yml`:
    ```yml
    name: holou
    services:
        bot:
            image: ghcr.io/eniraa/holou:latest
            environment:
                - DISCORD_TOKEN=${DISCORD_TOKEN}
    ```
2. Fill in the required environment variables in `.env`:
   - `DISCORD_TOKEN`: the bot's token
3. Start the bot: `docker-compose up`
