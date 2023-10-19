use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use url::Url;

pub struct WebSocketListener {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl WebSocketListener {
    pub fn new(url: &str) -> Self {
        let (socket, _response) = connect(Url::parse(url).unwrap()).expect("Can't connect");

        WebSocketListener { socket }
    }

    pub fn read(&mut self) -> Result<String, tungstenite::Error> {
        let ws_msg = self.socket.read()?;
        let text = ws_msg.to_text()?;

        Ok(text.to_string())
    }
}
