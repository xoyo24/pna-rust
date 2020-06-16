use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml)
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let mut kvs = kvs::KvStore::new();

    match matches.subcommand() {
        ("get", Some(sub_m)) => {
            if let Some(key) = sub_m.value_of("KEY") {
                println!("Value for get: {}", key);
                kvs.get(key.to_owned());
            }
        }
        ("set", Some(sub_m)) => {
            if let Some(key) = sub_m.value_of("KEY") {
                if let Some(value) = sub_m.value_of("VALUE") {
                    println!("Key:Value for set: {}:{}", key, value);
                    kvs.set(key.to_owned(), value.to_owned());
                }
            }
        }
        ("rm", Some(sub_m)) => {
            if let Some(key) = sub_m.value_of("KEY") {
                println!("Value for rm: {}", key);
                kvs.remove(key.to_owned());
            }
        }
        _ => unreachable!(),
    }
}
