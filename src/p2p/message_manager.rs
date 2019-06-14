extern crate serde;
extern crate semver;

use std::string::String;
use std::net::SocketAddr;
use self::serde::{Serialize, Deserialize};
use self::semver::Version;

const PROTOCOL_NAME: &'static str = "mincoin_protocol";
const PROTOCOL_VERSION: &'static str = "0.1.0";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MsgType {
    Add,
    Remove,
    CoreList,
    RequestCoreList,
    Ping,
    AddAsEdge,
    RemoveEdge,
    NewTransaction,
    NewBlock,
    RequestFullChain,
    RspFullChain,
    Enhanced,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub protocol: String,
    pub version: String,
    pub msg_type: MsgType,
    pub my_addr: SocketAddr,
    pub payload: Option<Vec<SocketAddr>>,
}

impl Message {
    pub fn new(msg_type: MsgType, my_addr: SocketAddr, payload: Option<Vec<SocketAddr>>) -> Message {
        Message {
            protocol: PROTOCOL_NAME.to_string(),
            version: PROTOCOL_VERSION.to_string(),
            msg_type,
            my_addr,
            payload,
        }
    }
}

pub fn build(msg_type: MsgType, my_addr: SocketAddr, payload: Option<Vec<SocketAddr>>) -> String {
    let msg = Message::new(msg_type, my_addr, payload);
    serde_json::to_string(&msg).unwrap()
}

pub fn parse(msg_str: &String) -> Result<Message, &'static str> {
    let msg: Message = serde_json::from_str(msg_str).unwrap();

    if msg.protocol != PROTOCOL_NAME.to_string() {
        Err("Protocol name is not matched")
    } else if Version::parse(&msg.version) > Version::parse(PROTOCOL_VERSION) {
        Err("Protocol version is not matched")
    } else {
        Ok(msg)
    }
}
