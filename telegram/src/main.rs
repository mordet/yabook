use std::env;

use futures::StreamExt;
use telegram_bot::*;

static TOKEN_ENV: &str = "TELEGRAM_BOT_TOKEN";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let telegram_token = env::var(TOKEN_ENV)
        .expect("telegram bot token env not set");

    println!("Configuring");
    let api: Api = Api::new(telegram_token);
    let mut stream = api.stream();

    println!("Running");
    while let Some(update) = stream.next().await {
        let update = update?;

        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text {ref data, ..} = message.kind {
                println!("<{}>: {}", &message.from.first_name, data);

                api.send(message.text_reply(
                    format!("Привет, {}! Команда echo дала '{}'",
                    &message.from.first_name, data)
                )).await?;
            }
        }
    }

    Ok(())
}
