use std::net::SocketAddr;

use serde_json;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    Message {
        name: String,
        address: SocketAddr,
        payload: MessagePayload,
        //sent_timestamp
    },
    Error
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessagePayload{
    Message {
        content: String,
    },
    Arrived,
    Request,
    Announce,
    WhosThere
}

impl Message {
    pub fn from_bytes(payload: &[u8]) -> Message {
        match serde_json::from_slice::<Message>(payload){
            Ok(msg) => msg,
            Err(e) => {
                error!("Cannot parse {} {:?}", e, payload);
                Message::Error
            }
        }
    }
}
