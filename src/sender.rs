use tokio::io::AsyncWriteExt;

use crate::response::Response;

pub async fn send_response(mut socket: tokio::net::TcpStream, response: Response) {
    let response = response.parse();
    let _ = socket.write_all(response.as_bytes()).await;
    let _ = socket.flush().await;
}
