use anyhow::Result;
use chrono::Utc;
use serenity::{
    client::{Client, Context, EventHandler},
    gateway::ActivityData,
    model::gateway::Ready,
    prelude::GatewayIntents,
};
use std::{
    env,
    sync::{Arc, atomic},
    time,
};
use warp::Filter;

#[derive(Clone, Debug)]
pub struct Static {
    pub ip: String,
    pub port: i32,
    pub password: String,
    pub mins_between_avatar_change: i32,
}

mod server_info;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        let user = ctx.cache.current_user().clone();
        log::info!("Logged in as {:#?}", user.name);

        let last_update = Arc::new(atomic::AtomicI64::new(0));
        let last_update_clone = Arc::clone(&last_update);

        let statics = Static {
            ip: env::var("ip").expect("No ip address set"),
            port: env::var("port")
                .expect("No port given")
                .parse::<i32>()
                .expect("port wasn't given an integer!"),
            password: env::var("password").expect("No password set"),
            mins_between_avatar_change: env::var("mins_between_avatar_change")
                .unwrap_or_else(|_| "1".to_string())
                .parse::<i32>()
                .expect("mins_between_avatar_change wasn't given an integer!"),
        };

        tokio::spawn(async move {
            let hello = warp::any().map(move || {
                let last_update_i64 = last_update_clone.load(atomic::Ordering::Relaxed);
                let now_minutes = Utc::now().timestamp() / 60;
                if (now_minutes - last_update_i64) > 5 {
                    warp::reply::with_status(
                        format!("{}", now_minutes - last_update_i64),
                        warp::http::StatusCode::SERVICE_UNAVAILABLE,
                    )
                } else {
                    warp::reply::with_status(
                        format!("{}", now_minutes - last_update_i64),
                        warp::http::StatusCode::OK,
                    )
                }
            });
            warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
        });

        // loop in separate async
        tokio::spawn(async move {
            loop {
                match status(ctx.clone(), &statics).await {
                    Ok(()) => {}
                    Err(e) => {
                        log::error!("cant get new stats: {:#?}", e);
                        // return old if it cant find new details
                    }
                };
                last_update.store(Utc::now().timestamp() / 60, atomic::Ordering::Relaxed);
                // wait 2 minutes before redo
                tokio::time::sleep(time::Duration::from_secs(60)).await;
            }
        });
    }
}

async fn status(ctx: Context, statics: &Static) -> Result<()> {
    match server_info::get_status(statics).await {
        Ok(status) => {
            let status_msg = format!(
                "{}/{} - {}",
                status.players.current, status.players.max, status.map
            );
            ctx.set_activity(Some(ActivityData::playing(status_msg)));
        }
        Err(e) => {
            let server_info = "¯\\_(ツ)_/¯ server not found";
            ctx.set_activity(Some(ActivityData::playing(server_info)));

            anyhow::bail!(format!("Failed to get new serverinfo: {:#?}", e))
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::set_max_level(log::LevelFilter::Info);
    flexi_logger::Logger::try_with_str("warn,discord_bot=info")
        .unwrap_or_else(|e| panic!("Logger initialization failed with {:#?}", e))
        .start()?;

    // Login with a bot token from the environment
    let token = &env::var("token").expect("token wasn't given an argument!")[..];
    let intents = GatewayIntents::non_privileged();
    let mut client: Client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }
    Ok(())
}
