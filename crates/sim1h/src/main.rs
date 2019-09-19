#[macro_use]
extern crate log;

#[macro_use]
extern crate detach;

extern crate env_logger;
extern crate futures;

use lib3h::engine::ghost_engine_wrapper::LegacyLib3h;
use crate::ghost_actor::SimGhostActor;
use lib3h_protocol::data_types::ConnectData;
use url::Url;
use lib3h_protocol::protocol_client::Lib3hClientProtocol;

pub mod agent;
pub mod dht;
pub mod ghost_actor;
pub mod protocol_map;
pub mod trace;
pub mod workflow;

fn main() {
    println!("Yeah");

    let ghost_engine = SimGhostActor::new(&"http://derp:8000".into());
    let mut net_engine = LegacyLib3h::new("core", ghost_engine);
    let connect_data = ConnectData {
        request_id: String::from("request-id-0"),
        peer_uri: Url::parse("http://bs").unwrap(),
        network_id: String::from("network-id"),
    };
    let message = Lib3hClientProtocol::Connect(connect_data);
    net_engine.post(message).expect("post to work");


    let mut i = 1;
    loop {
        let (did_something, output) = net_engine.process().expect("process to work without error");
        println!("PROCESS tick {}: {:?} {:?}", i, did_something, output);
        i = i+1;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

}
