use std::env;

use futures::StreamExt;
use telegram_bot::*;

static TOKEN_ENV: &str = "TELEGRAM_BOT_TOKEN";

fn substr(data: &String, offset: i64, length: i64) -> String {
  data.chars()
      .skip(offset as usize)
      .take(length as usize)
      .collect()
}

async fn process_text_message(
    api: &Api, data: &String, entities: &Vec<MessageEntity>, message: &Message) {
    if let Some(login) = message.from.username.as_ref() {
        println!("<{}>: {}", login, data);
    }

    for entity in entities {
        let value = substr(&data, entity.offset, entity.length);
        match entity.kind {
            MessageEntityKind::Mention => {
                println!("You mentioned {}", value)
            },
            MessageEntityKind::BotCommand => println!("Bot command found: {}", value),
            _ => {}
        }
    }

    match api.send(message.text_reply(
        format!("Привет, {}! Команда echo дала '{}'",
                &message.from.first_name, data))).await {
        Ok(_) => {},
        Err(err) => println!("Не удалось отправить сообщение получателю: {:?}", err),
    }
}

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
            if let MessageKind::Text {ref data, ref entities} = message.kind {
                process_text_message(&api, &data, &entities, &message).await;
            }
        }
    }

    Ok(())
}
