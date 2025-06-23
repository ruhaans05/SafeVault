use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Deserialize)]
struct IncomingMessage {
    action: String,
}

#[derive(Serialize)]
struct OutgoingMessage {
    status: String,
}

fn main() {
    let mut len_buf = [0u8; 4];
    if io::stdin().read_exact(&mut len_buf).is_err() {
        return;
    }

    let len = u32::from_le_bytes(len_buf) as usize;
    let mut buffer = vec![0; len];
    if io::stdin().read_exact(&mut buffer).is_err() {
        return;
    }

    let msg: IncomingMessage = match serde_json::from_slice(&buffer) {
        Ok(m) => m,
        Err(_) => return,
    };

    let status = if msg.action == "trigger_auth" {
        "OK"
    } else {
        "FAIL"
    };

    let response = OutgoingMessage {
        status: status.to_string(),
    };

    let json = serde_json::to_vec(&response).unwrap();
    let len_bytes = (json.len() as u32).to_le_bytes();

    io::stdout().write_all(&len_bytes).unwrap();
    io::stdout().write_all(&json).unwrap();
    io::stdout().flush().unwrap();
}
