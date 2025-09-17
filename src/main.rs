// use std::{env::args, fs, io::Result, ops::Range, path::Path, time::Duration};
// mod app;
// mod key_controller;
// mod key_table_parser;
// mod text_buffer;
// mod tree;
//
// pub const DESIRED_FRAME_TIME: Duration = Duration::from_millis(10);

use serde_json::{Value, json, to_string_pretty};
use std::{
    io::{BufRead, BufReader, Read, Write},
    process::Command,
    thread,
};

fn main() -> std::io::Result<()> {
    let mut child = Command::new("rust-analyzer")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());

    thread::scope(|scope| {
        scope.spawn(|| {
            loop {
                match read_message(&mut stdout) {
                    Ok(yippie) => pretty_print_json(&yippie),
                    Err(naur) => println!("{naur}"),
                };
            }
        });

        let initialize_msg = serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": std::process::id(),
                "rootUri": "file:///home/merle/dev/rust/nex",
                "capabilities": {
                    "window": {
                        "workDoneProgress": true
                    },
                    "workspace": {
                        "workspaceFolders": true
                    },
                    "textDocument": {
                        "completion": {
                        "hover": {
                            "contentFormat": ["markdown", "plaintext"]
                        },
                        "completion": {
                            "completionItem": {
                                "snippetSupport": true,
                                "documentationFormat": ["markdown", "plaintext"]
                            },
                            "contextSupport": true
                        }}
                    }
                }
            }
        }))?;

        send_message(&mut stdin, &initialize_msg)?;

        let initialized_msg = serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {}
        }))?;

        send_message(&mut stdin, &initialized_msg)?;

        let did_open_msg = serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///home/merle/dev/rust/nex/src/main.rs",
                    "languageId": "rust",
                    "version": 1,
                    "text": std::fs::read_to_string("/home/merle/dev/rust/nex/src/main.rs").unwrap_or_default()
                }
            }
        }))?;

        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("sleeped 5s");

        send_message(&mut stdin, &did_open_msg)?;

        let hover_func = |l, c| {
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "textDocument/hover",
                "params": {
                    "textDocument": {
                        "uri": "file:///home/merle/dev/rust/nex/src/main.rs"
                    },
                    "position": {
                        "line": l,
                        "character": c
                    },
                }
            }))
            .unwrap_or_default()
        };

        send_message(&mut stdin, &hover_func(106, 10))?;

        let shutdown_msg = serde_json::to_string(&json!({
          "jsonrpc": "2.0",
          "id": 3,
          "method": "shutdown",
          "params": null
        }))?;

        send_message(&mut stdin, &shutdown_msg)?;

        Ok(())
    })
}

fn send_message(stdin: &mut std::process::ChildStdin, message: &str) -> std::io::Result<()> {
    let header = format!("Content-Length: {}\r\n\r\n", message.len());
    stdin.write_all(header.as_bytes())?;
    stdin.write_all(message.as_bytes())?;
    stdin.flush()?;
    Ok(())
}

fn read_message(stdout: &mut BufReader<std::process::ChildStdout>) -> std::io::Result<String> {
    // Read Content-Length header
    let mut header_line = String::new();
    stdout.read_line(&mut header_line)?;

    let content_length: usize = header_line
        .strip_prefix("Content-Length: ")
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);

    // Read empty line
    let mut empty_line = String::new();
    stdout.read_line(&mut empty_line)?;

    // Read message content
    let mut buffer = vec![0; content_length];
    stdout.read_exact(&mut buffer)?;

    Ok(String::from_utf8_lossy(&buffer).to_string())
}

fn pretty_print_json(json_str: &String) {
    match serde_json::from_str::<Value>(json_str) {
        Ok(value) => {
            if let Ok(pretty) = to_string_pretty(&value) {
                println!("{} \n\n--\n", pretty);
            }
        }
        Err(e) => println!("Invalid JSON: {}", e),
    }
}
