use std::error::Error;

use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn handle(mut inbound: TcpStream, proxy_addr: &str) -> Result<(), Box<dyn Error>> {
    println!("handle proxy: {}", proxy_addr);
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
