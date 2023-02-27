use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;

#[tokio::main]
async fn main() {
    // Conecte-se ao servidor Telnet
    let stream = TcpStream::connect("127.0.0.1:23").await.unwrap();
    let (read, write) = WebSocketStream::from_raw_socket(stream).split();

    // Envie uma mensagem para o servidor
    let message = Message::text("hello");
    write.send(message).await.unwrap();

    // Receba uma resposta do servidor
    let message = read.next().await.unwrap().unwrap();
    println!("Mensagem recebida: {}", message.to_text().unwrap());
}
