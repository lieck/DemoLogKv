use clap::{arg, Command};

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("get")
                .about("Set the value of a string key to a string")
                .arg(arg!([Key]).help("A string key").required(true)),
        )
        .subcommand(
            Command::new("set")
                .about("about")
                .arg(arg!([Key]).help("A string key").required(true))
                .arg(
                    arg!([Value])
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("about")
                .arg(arg!([Key]).help("A string key").required(true)),
        )
        .get_matches();

    let mut kvs = kvs::KvStore::new();
    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let key = sub_matches.get_one::<String>("Key").unwrap();
            if let Some(val) = kvs.get(key.clone()) {
                println!("{val}");
            } else {
                println!("err");
            }
        }
        Some(("set", sub_matches)) => {
            let key = sub_matches.get_one::<String>("Key").unwrap();
            let val = sub_matches.get_one::<String>("Value").unwrap();
            kvs.set(key.clone(), val.clone());
        }
        Some(("rm", sub_matches)) => {
            let key = sub_matches.get_one::<String>("Key").unwrap();
            kvs.remove(key.clone());
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
