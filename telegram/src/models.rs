use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum GameType {
    Kicker,
    Pool,
    PingPong
}

#[derive(Serialize, Deserialize)]
pub enum LobbyStatus {
    Draft,
    Approved,
    Declined
}
