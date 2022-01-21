# solbot
Solana Discord Bot

## To Build and Run in Docker
```shell
docker build -t solbot/solbot-app:1.0.0 .
docker run -e DISCORD_TOKEN=the_token --rm -ti solbot/solbot-app:1.0.0
```