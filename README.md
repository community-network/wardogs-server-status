This is just a small bot that shows watchdog server status within the discord activity

<img width="190" height="40" alt="image" src="https://github.com/user-attachments/assets/6e50d9e2-2695-4a36-b6e6-38a6f523d566" />


### Environment items:

```yaml
token: discord bot token
ip: ip of the server
port: rcon port of the server
password: password for rcon
```


## Using the bot

You can run it with Docker (Docker Compose):

```docker
services:
  wardogs-server-bot:
    image: ghcr.io/community-network/wardogs-server-status/wardogs-server-status:latest
    restart: always
    environment:
      - token=TOKEN
      - ip=127.0.0.1
      - port=90000
      - password=PASS
    healthcheck:
      test: ["CMD", "curl", "-f", "http://127.0.0.1:3030/"]
      interval: "60s"
      timeout: "3s"
      start_period: "5s"
      retries: 3
```

Or use the executable available [here](https://github.com/community-network/wardogs-server-status/releases/latest)

And use that on windows via a bat file:

```bat
@ECHO OFF
SET token=DISCORDTOKEN
SET ip=127.0.0.1
SET port=9000
SET password=PASS
FILENAME.exe
```

Or on Linux/Mac with these commands:

```bash
export token=TOKEN
export ip=127.0.0.1
export port=9000
export password=PASS
./FILENAME
```

If you want to run it with your own changes in the code, install [rust](https://www.rust-lang.org/tools/install) and run with:

```bash
export token=TOKEN
export ip=127.0.0.1
export port=9000
export password=PASS
cargo run
```
