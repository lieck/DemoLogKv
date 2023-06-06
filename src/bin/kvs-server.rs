use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    vec,
};

use bytes::Buf;
use clap::Parser;

use kvs::KvsEngine;
use kvs::{RequestMsg, ReplyMsg, RequestType};
use log::LevelFilter;


#[derive(Parser, Debug)]
#[command(about = "Fun with load balancing", version = env!("CARGO_PKG_VERSION"))]
struct CmdOptions {
    #[arg(short, long, default_value = "127.0.0.1:4000")]
    pub addr: String,
    #[arg(short, long, default_value = "kvs")]
    pub engine: String,
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).init();
    
    
    let options = CmdOptions::parse();

    log::info!("Starting {} engine...", options.engine);

    let mut kvs;

    if options.engine == "kvs" {
        kvs = kvs::KvStore::new(None);
    } else {
        log::error!("Unknown engine");
        return;
    }

    let listener = match TcpListener::bind(&options.addr) {
        Ok(listener) => listener,
        Err(err) => {
            log::error!("Could not bind to {}: {}", options.addr, err);
            std::process::exit(1);
        }
    };

    log::info!("Listening for requests on {}", options.addr);
    while let Ok((stream, addr)) = listener.accept() {
        log::info!("accept {}", addr.ip().to_string());
        handle_connection(stream, &mut kvs);
    }
}


fn handle_connection(mut client_conn: TcpStream, kvs: &mut impl KvsEngine) {

    let mut buf = [0; 4];
    client_conn.read_exact(&mut buf).unwrap();

    let buf_len = (&buf[0..4]).get_u32() as usize;
    log::debug!("request len : {}", buf_len);
    if buf_len == 0 {
        log::error!("request len : 0");
        return;
    }

    let mut buf = vec![0; buf_len];
    client_conn.read_exact(&mut buf).unwrap();
    
    let msg = RequestMsg::parse(&buf).unwrap();
    log::debug!("request: {:?}", msg);

    let mut msg_send = ReplyMsg {
        reply_type : kvs::ReplyType::Ok,
        value : None,
    };


    match msg.request_type {
        RequestType::Put => {
            kvs.set(msg.key.clone(), msg.value.clone().unwrap()).unwrap();
            log::debug!("put {} ==> value {:?}", msg.key, kvs.get(msg.key.clone()).unwrap());
        },
        RequestType::Delete => {
            kvs.remove(msg.key).unwrap();
        },
        RequestType::Get => {
            msg_send.value = kvs.get(msg.key.clone()).unwrap();
            msg_send.reply_type = kvs::ReplyType::Msg;
            log::debug!("get {} ==> value {:?}", msg.key, msg_send.value);
        }
    }

    let buf = msg_send.build();
    let buf_len = buf.len() as u32;
    client_conn.write_all(&mut buf_len.to_be_bytes().to_vec()).unwrap();
    client_conn.write_all(&buf).unwrap();
}
