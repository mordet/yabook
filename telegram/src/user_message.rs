use telegram_bot::{MessageEntityKind, MessageEntity};

pub struct UserMessage {
    command: Option<String>,
    mentions: Vec<String>
}

fn substring(data: &String, offset: i64, length: i64) -> String {
    data.chars()
        .skip(offset as usize)
        .take(length as usize)
        .collect()
}

impl UserMessage {
    pub fn new(data: &String, entities: &Vec<MessageEntity>) -> UserMessage {
        let mut command: Option<String> = None;
        let mut mentions: Vec<String> = vec![];

        for entity in entities {
            let value = substring(&data, entity.offset, entity.length);
            match entity.kind {
                MessageEntityKind::Mention => {
                    mentions.push(value);
                },
                MessageEntityKind::BotCommand => match command {
                    None => command = Some(value),
                    Some(_) => println!("Secondary command found, rejecting"),
                },
                _ => {},
            }
        }

        UserMessage { command: command, mentions: mentions }
    }
}
