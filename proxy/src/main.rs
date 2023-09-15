use clap::Parser;
mod connector;
mod handler;
use connector::*;

mod listeners;
use listeners::http::HttpListener;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Listen host and port
    #[clap(short = 'L', value_parser)]
    listen: String,

    /// Forward host and port
    #[clap(short = 'F', value_parser)]
    forward: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let l_conn = parse_args(&args.listen);
    Connection::display(&l_conn);

    let f_conn = parse_args(&args.forward);
    Connection::display(&f_conn);

    let l = match l_conn.protocol {
        Protocol::Http => HttpListener::new(l_conn, f_conn),
        _ => HttpListener::new(l_conn, f_conn),
    };

    _ = l.listen().await;
}

fn parse_args(s: &str) -> Connection {
    let strs: Vec<&str> = s.split("://").collect();
    let hostport: Vec<&str> = strs[1].split(":").collect();

    let pt: Vec<&str> = strs[0].split("+").collect();
    let mut p = Protocol::Default;
    let mut t = Transport::Default;

    if pt.len() == 1 || pt.len() == 2 {
        p = match pt[0] {
            "http" => Protocol::Http,
            "http2" => Protocol::Http2,
            _ => Protocol::Default,
        };
    }

    if pt.len() == 2 {
        t = match pt[1] {
            "tcp" => Transport::Tcp,
            "tls" => Transport::Tls,
            _ => Transport::Default,
        };
    }

    let host = match hostport[0] {
        "" => "127.0.0.1",
        _ => hostport[0],
    };

    Connection::new(p, t, host, hostport[1].parse::<u32>().unwrap())
}
