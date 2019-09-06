use std::env;

use futures::StreamExt;
use telegram_bot::*;
use crate::user_message::UserMessage;
use crate::client::*;

mod models;
mod user_message;
mod client;

static TOKEN_ENV: &str = "TELEGRAM_BOT_TOKEN";

async fn process(api: Api, message: Message) -> Result<(), Box<dyn std::error::Error>> {
    if let MessageKind::Text {ref data, ref entities} = message.kind {
        let reply = process_text_message(&data, &entities, &message).await;

        match reply {
            Ok(msg) => {
                api.send(message.text_reply(
                    format!("Привет, {}! Вот тебе ответ: '{}'",
                            &message.from.first_name, msg))).await?;
            },
            Err(e) => {
                api.send(message.text_reply(
                    format!("При обработке команды произошла ошибка: {}", e)
                )).await?;
            }
        }
    }

    Ok(())
}

async fn process_text_message(data: &String, entities: &Vec<MessageEntity>, message: &Message)
    -> Result<String, Box<dyn std::error::Error>> {
    let user_message = UserMessage::new(&data, &entities);
    if user_message.command.is_none() {
        return Ok("".to_string())
    }

    let login = message.from.username.clone().unwrap_or("<anonymous>".to_string());

    let command = user_message.command.unwrap();
    match command.as_ref() {
        "/help" => {
            Ok("Список доступных команд:\n\
            /list для получения списка бронирований\n\
            /create {@login1} {@login2} для создания брони и приглашения игроков".to_string())
        },
        "/list" => {
            println!("Вызов функции /list");
            let resp = get_bookings_list(
                "benua_kicker".to_string()
            )?;
            Ok(serde_json::to_string(&resp)?)
        },
        "/create" => {
            println!("Вызов функции /create");
            let resp = create_booking(
                "benua_kicker".to_string(), login, user_message.mentions)?;
            Ok(serde_json::to_string(&resp)?)
        },
/*        "/invite" => {
            println!("Вызов функции /invite");
            // let resp =
        }*/
        _ => Ok("Команда неизвестна".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
