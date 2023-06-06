use kvs::{RequestMsg, ReplyMsg, ReplyType};
use log::LevelFilter;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use clap::{arg, Command};

fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("get")
                .about("Set the value of a string key to a string")
                .arg(arg!([Key]).help("A string key").required(true))
                .subcommand(
                    Command::new("--ipaddr").arg(arg!([Ipaddr]).help("Ipaddr").required(true)),
                ),
        )
        .subcommand(
            Command::new("set")
                .about("about")
                .arg(arg!([Key]).help("A string key").required(true))
                .arg(
                    arg!([Value])
                        .help("The string value of the key")
                        .required(true),
                )
                .subcommand(
                    Command::new("--ipaddr").arg(arg!([Ipaddr]).help("Ipaddr").required(true)),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("about")
                .arg(arg!([Key]).help("A string key").required(true))
                .subcommand(
                    Command::new("--ipaddr").arg(arg!([Ipaddr]).help("Ipaddr").required(true)),
                ),
        )
        .get_matches();


    let mut ipaddr = String::from("127.0.0.1:4000");
    
    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let key = sub_matches.get_one::<String>("Key").unwrap();

            match sub_matches.subcommand() {
                Some(("--ipaddr", sub_matches)) => {
                    let cmd = sub_matches.get_one::<String>("Ipaddr").unwrap();
                    ipaddr = cmd.to_string();
                }
                _ => {}
            }

            get(ipaddr, key);
            
        }

        Some(("set", sub_matches)) => {
            let key = sub_matches.get_one::<String>("Key").unwrap();
            let value = sub_matches.get_one::<String>("Value").unwrap();

            match sub_matches.subcommand() {
                Some(("--ipaddr", sub_matches)) => {
                    let cmd = sub_matches.get_one::<String>("Ipaddr").unwrap();
                    ipaddr = cmd.to_string();
                }
                _ => {}
            }

            set(ipaddr, key, value);
        }

        Some(("rm", sub_matches)) => {
            let key = sub_matches.get_one::<String>("Key").unwrap();

            match sub_matches.subcommand() {
                Some(("--ipaddr", sub_matches)) => {
                    let cmd = sub_matches.get_one::<String>("Ipaddr").unwrap();
                    ipaddr = cmd.to_string();
                }
                _ => {}
            }

            rm(ipaddr, key);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

fn set(ipaddr: String, key : &String, value : &String) {
    let buf = RequestMsg::build(kvs::RequestType::Put, key.clone(), Some(value.clone()));
    let buf = send_request(ipaddr, &buf);

    let msg = ReplyMsg::parse(&buf).unwrap();
    reply(msg);
}

fn get(ipaddr: String, key : &String) {
    let buf = RequestMsg::build(kvs::RequestType::Get, key.clone(), None);
    let buf = send_request(ipaddr, &buf);

    let msg = ReplyMsg::parse(&buf).unwrap();
    reply(msg);
}

fn rm(ipaddr: String, key : &String) {
    let buf = RequestMsg::build(kvs::RequestType::Delete, key.clone(), None);
    let buf = send_request(ipaddr, &buf);

    let msg = ReplyMsg::parse(&buf).unwrap();
    reply(msg);
}

fn reply(msg : ReplyMsg) {
    log::debug!("reply: {:?}", msg);
    match msg.reply_type {
        ReplyType::Error => println!("err"),
        ReplyType::Ok => println!("Ok"),
        ReplyType::Msg => println!("msg : {}", msg.value.unwrap()),
    }
}

fn send_request(ipaddr: String, send_buf: &[u8]) -> Vec<u8> {
    let mut conn = TcpStream::connect(ipaddr).unwrap();
    
    let mut len = (send_buf.len() as u32).to_be_bytes().to_vec();
    log::debug!("send request len: {}, vec: {:?}", send_buf.len(), len);

    conn.write_all(&mut len).unwrap();
    conn.write_all(send_buf).unwrap();

    let mut buf = [0; 4];
    conn.read_exact(&mut buf).unwrap();
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as usize];
    conn.read_exact(&mut buf).unwrap();
    buf
}
