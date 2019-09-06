use std::env;

use futures::StreamExt;
use telegram_bot::*;
use crate::user_message::UserMessage;

mod user_message;

static TOKEN_ENV: &str = "TELEGRAM_BOT_TOKEN";

async fn process(api: Api, message: Message) -> Result<(), Error> {
    if let MessageKind::Text {ref data, ref entities} = message.kind {
        process_text_message(&api, &data, &entities, &message).await;
    }

    Ok(())
}

async fn process_text_message(
    api: &Api, data: &String, entities: &Vec<MessageEntity>, message: &Message) {
    let user_message = UserMessage::new(&data, &entities);

    if let Some(login) = message.from.username.as_ref() {
        println!("<{}>: {}", login, data);
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
        .expect(format!("{} env not set", TOKEN_ENV).as_str());

    let api: Api = Api::new(telegram_token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;

        if let UpdateKind::Message(message) = update.kind {
            process(api.clone(), message).await?
        }
    }

    Ok(())
}
