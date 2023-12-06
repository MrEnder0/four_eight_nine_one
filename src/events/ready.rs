use poise::serenity_prelude::{self as serenity, Ready};
use scorched::*;

use crate::utils::reply_list::download_reply_list;

pub async fn ready_event(data_about_bot: &Ready, ctx: &serenity::Context) {
    log_this(LogData {
        importance: LogImportance::Info,
        message: format!(
            "{} has started and connected to discord.",
            data_about_bot.user.name
        ),
    })
    .await;

    // Downloads reply list
    log_this(LogData {
        importance: LogImportance::Info,
        message: "Downloading reply list.".to_owned(),
    })
    .await;

    download_reply_list().await;

    log_this(LogData {
        importance: LogImportance::Info,
        message: "Reply list downloaded.".to_owned(),
    })
    .await;

    // Sets 4891 activity
    let bot_activity_ctx = ctx.clone();
    tokio::spawn(async move {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
            let guild_count = bot_activity_ctx.cache.guilds().len();
            let activity_msg = format!("with children in {} servers.", guild_count);
            bot_activity_ctx
                .set_activity(serenity::Activity::playing(&activity_msg))
                .await;
        }
    });
}
