pub enum Protocol {
    Default,
    Http,
    Http2,
}
pub enum Transport {
    Default,
    Tcp,
    Tls,
}

pub struct Connection<'a> {
    pub protocol: Protocol,   
    pub transport: Transport, 
    pub host: &'a str,        
    pub port: u32,            
}

impl<'a> Connection<'a> {
    pub fn new(p: Protocol, t: Transport, host: &str, port: u32) -> Connection {
        Connection {
            protocol: p,
            transport: t,
            host: host,
            port: port,
        }
    }

    pub fn display(&self) {
        let p = match self.protocol {
            Protocol::Default => "http",
            Protocol::Http => "http",
            Protocol::Http2 => "http2",
        };

        let t = match self.transport {
            Transport::Default => "tcp",
            Transport::Tcp => "tcp",
            Transport::Tls => "tls",
        };

        println!(
            "protocol: {}, transport:{}, host: {}, port: {}",
            p, t, self.host, self.port
        );
    }
}
