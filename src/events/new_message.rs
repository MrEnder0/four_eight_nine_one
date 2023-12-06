use poise::{serenity_prelude as serenity, serenity_prelude::ReactionType};
use rustrict::CensorStr;
use scorched::*;

use crate::utils::counter::CountStruct;

pub async fn new_message_event(ctx: &serenity::Context, new_message: &serenity::Message) {
    let ctx = ctx.clone();
    let say_prefix = format!("<@{}> say", ctx.cache.current_user_id());

    if new_message.author.bot {
        return;
    }

    CountStruct::increment_count();

    if new_message.content.starts_with(&say_prefix) {
        if new_message.content.contains("@everyone") || new_message.content.contains("@here") {
            log_this(LogData {
                importance: LogImportance::Info,
                message: "Canceled message copy due to mass ping".to_owned(),
            })
            .await;

            return;
        }

        if new_message.content.is_inappropriate() {
            log_this(LogData {
                importance: LogImportance::Info,
                message: "Canceled message copy due to inappropriateness".to_owned(),
            })
            .await;

            new_message
                .react(&ctx, ReactionType::Unicode("👎".to_owned()))
                .await
                .log_expect(LogImportance::Warning, "Unable to react to ping");

            new_message
                .reply(&ctx, "Your message was to means for me to say.")
                .await
                .log_expect(LogImportance::Warning, "Unable to reply to ping");

            return;
        }

        log_this(LogData {
            importance: LogImportance::Info,
            message: format!(
                "Saying message from {} in {}",
                new_message.author.name,
                new_message.channel_id.name(&ctx).await.unwrap()
            ),
        })
        .await;

        let new_message_reply = new_message.content.clone()[say_prefix.len()..].to_owned();

        new_message
            .channel_id
            .to_channel(&ctx)
            .await
            .unwrap()
            .id()
            .say(&ctx, new_message_reply)
            .await
            .log_expect(LogImportance::Warning, "Unable to say message");
    }

    if CountStruct::check_count() {
        log_this(LogData {
            importance: LogImportance::Info,
            message: "Target count reached, replying with words".to_owned(),
        })
        .await;

        let reply = crate::utils::reply_list::choose_reply_from_list().await;

        new_message
            .channel_id
            .say(&ctx, reply)
            .await
            .log_expect(LogImportance::Warning, "Unable to say message");
    }
}
