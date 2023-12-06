use scorched::*;

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Explains what 4891 is and who made it
#[poise::command(prefix_command, slash_command, channel_cooldown = 60)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("About 4891")
                .description("4891 was originally developed by 1984 in the Python programming language, 4891 is now has been rewritten in Rust by Mr.Ender and is maintained on Github.")
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
