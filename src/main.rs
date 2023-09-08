// Engine web-backend

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// function of rocket endpoint /get-id returns id
#[get("/get-id")]
fn index() -> &'static str {
    
    "Hello, world!"
}


extern crate websocket;
// use std::sync::Arc;

use websocket::client::ClientBuilder;
// use websocket::Message;
// use websocket::OwnedMessage;


struct Ctx {
    wss_url: String,
    wss: Option<websocket::sync::Client<std::net::T>>,
}

// this function input ctx and create websocket connection with polkadot.js
// and return websocket client
fn wss_connect(mut ctx: Ctx) -> bool {
    match ctx.wss {
        Some(wss) => {
            return true;
        },
        None => {
            // open websocket with polkadot.js
            let client = ClientBuilder::new(&ctx.wss_url)
                .unwrap()
                .connect_secure(None);                

            match client {
                Ok(c) => {
                    println!("Connection successful: {:?}",ctx.wss_url);
                    ctx.wss = Some(c);
                    return true;
                },
                Err(e) => {
                    println!("Error: {}", e);
                    return false;
                }
            }
        }
    }
}

fn main() {
    let mut ctx = Ctx {
        wss_url: String::from("wss://rpc.polkadot.io"),
        wss: None,
    };
    
    if wss_connect(ctx) {
        println!("Connection successful: {:?}",ctx.wss_url);
    } else {
        println!("Connection failed: {:?}",ctx.wss_url);
    }    

    // // get number version op polkadot.js
    // let version = client
    // .send_message(&Message::text(r#"{"id":1, "jsonrpc":"2.0", "method":"system_chain"}"#))
    // .unwrap()
    // .recv_message()
    // .unwrap();

    // // print version
    // println!("Received: {}", version);
    
    // start server
    rocket::ignite().mount("/", routes![index]).launch();

}

// rocket
