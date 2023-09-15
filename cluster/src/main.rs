use std::process::Command;
use std::env;
use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();

    let key = &args[2].as_str();
    let mut hashval:u32 = 0;
    let magic = 103;
    for letter in key.chars() {
        hashval += letter as u32;
        hashval *= magic;
        hashval %= 19260817;
    }
    hashval %= 10;
    let mut host = "http://127.0.0.1:700".to_string();
    host.push(char::from_u32(hashval + '0' as u32).unwrap());
    let mut command = "echo '../proxy/target/debug/proxy -L http://127.0.0.1:3030 -F ".to_string();
    command.push_str(&host);
    command.push_str(" &' > proxy.sh");
    Command::new("bash").arg("-c").arg(&command).status();
    Command::new("bash")
            .arg("proxy.sh")
            .spawn();

    let delay = time::Duration::from_millis(100);
    
    thread::sleep(delay);
            
    if args.len() > 3 {
        let _output = Command::new("../myredis/target/debug/client")
                        .arg("-p")
                        .arg("3030")
                        .arg(args[1].as_str())
                        .arg(args[2].as_str())
                        .arg(args[3].as_str())
                        .status()
                        .expect("failed to execute process");
    } else {
        let _output = Command::new("../myredis/target/debug/client")
                        .arg("-p")
                        .arg("3030")
                        .arg(args[1].as_str())
                        .arg(args[2].as_str())
                        .status()
                        .expect("failed to execute process");
    }

    Command::new("pkill")
            .arg("proxy")
            .status();
}