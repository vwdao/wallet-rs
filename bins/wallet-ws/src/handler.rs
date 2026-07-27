use futures::{SinkExt, StreamExt};
use salvo::prelude::*;
use salvo::websocket::{Message, WebSocket, WebSocketUpgrade};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::hub::Hub;

#[derive(Deserialize)]
struct ClientMsg {
    op: String,
    topic: Option<String>,
}

#[handler]
pub async fn ws_upgrade(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), StatusError> {
    let hub = match depot.get_typed::<Arc<Hub>>() {
        Ok(h) => h.clone(),
        Err(_) => {
            tracing::error!("Hub not inserted in depot");
            return Err(StatusError::internal_server_error());
        }
    };
    WebSocketUpgrade::new()
        .upgrade(req, res, move |ws| handle(ws, hub))
        .await
}

async fn handle(ws: WebSocket, hub: Arc<Hub>) {
    let (mut sender, mut receiver) = ws.split();

    let (tx_cmd, mut rx_cmd) = mpsc::unbounded_channel::<String>();
    let (tx_pong, mut rx_pong) = mpsc::unbounded_channel::<()>();

    let mut topic_rx = hub.subscribe("wallet.tx.indexed");

    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                biased;
                Some(topic) = rx_cmd.recv() => {
                    topic_rx = hub.subscribe(&topic);
                }
                Some(()) = rx_pong.recv() => {
                    if sender.send(Message::text(r#"{"op":"pong"}"#.to_string())).await.is_err() {
                        break;
                    }
                }
                msg = topic_rx.recv() => {
                    if let Ok(payload) = msg {
                        if sender.send(Message::text(payload.to_string())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if !msg.is_text() {
                continue;
            }
            let text_str = match msg.as_str() {
                Ok(s) => s,
                Err(_) => continue,
            };
            if let Ok(client_msg) = serde_json::from_str::<ClientMsg>(text_str) {
                match client_msg.op.as_str() {
                    "subscribe" => {
                        if let Some(topic) = client_msg.topic {
                            let _ = tx_cmd.send(topic);
                        }
                    }
                    "ping" => {
                        let _ = tx_pong.send(());
                    }
                    _ => {}
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}
