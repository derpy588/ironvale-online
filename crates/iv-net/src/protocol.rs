use serde::{Serialize, Deserialize};

// Server --> Client Messages
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
     
}


// Client --> Server Messages
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {

}

