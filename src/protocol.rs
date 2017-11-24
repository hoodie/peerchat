use std::net::SocketAddr;

use serde_json;


#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Message {
        name: String,
        address: SocketAddr,
        payload: MessagePayload,
        //sent_timestamp
    },
    Error
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessagePayload{
    Message {
        content: String,
    },
    Arrived,
    Request,
    Announce,
    WhosThere
}

pub fn decode_msg(payload: &[u8]) -> Message {
    match serde_json::from_slice::<Message>(payload){
        Ok(msg) => msg,
        Err(e) => {
            error!("Cannot parse {} {:?}", e, payload);
            Message::Error
        }
    }
}