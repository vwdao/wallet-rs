use salvo::websocket::Message;

pub fn is_close(msg: &Message) -> bool {
    msg.is_close()
}

pub fn is_control(msg: &Message) -> bool {
    msg.is_ping() || msg.is_pong()
}

pub fn payload_bytes(msg: &Message) -> Vec<u8> {
    msg.as_bytes().to_vec()
}

pub fn to_text_or_binary(bytes: &[u8]) -> Message {
    match std::str::from_utf8(bytes) {
        Ok(s) => Message::text(s.to_string()),
        Err(_) => Message::binary(bytes.to_vec()),
    }
}
