mod about;

type Error = Box<dyn std::error::Error + Send + Sync>;

// Link all commands into vec
pub fn commands() -> Vec<poise::Command<super::Data, Error>> {
    let commands = vec![about::about()];

    commands
}
