extern crate rohrpost;

use async_std::io;
use async_std::sync;
use futures::executor::{block_on, LocalPool};
use futures::task::LocalSpawnExt;
use rohrpost::helpers::Or;
use rohrpost::telegram_methods;
use rohrpost::telegram_receiver;
use rohrpost::telegram_sender;
use rohrpost::telegram_types;
use rohrpost::TelegramReceiver;
use rohrpost::TelegramSender;
use std::net::SocketAddr;

const TOKEN: &'static str = "1066698743:AAEbuPawIIjn9merOarOkZE688qk0WOFfxc";

fn main() {
    let config = telegram_sender::Config::new(String::from(TOKEN));
    let sender = TelegramSender::new(config);
    let config = telegram_receiver::Config::new(
        SocketAddr::from(([0, 0, 0, 0], 8443)),
        String::from("fullchain.pem"),
        String::from("privkey.pem"),
        String::from("https://auch.dnshome.de:8443/somesecretpath"),
    );
    let (recv, stop_send, http_recv) = TelegramReceiver::new(config);

    //register webhook so we receive updates
    println!("Registering webhook");
    block_on(register_web_hook(&sender, &recv)).unwrap();

    println!("Creating Thread pool");
    let mut local_pool = LocalPool::new();
    let local_spawn = local_pool.spawner();
    println!("Spawn stdin future");
    local_spawn
        .spawn_local(send_stop_on_stdin_enter(stop_send))
        .unwrap();
    println!("Spawn receiver future");
    local_spawn
        .spawn_local(recv.run(local_spawn.clone()))
        .unwrap();
    println!("Spawn echo_bot future");
    local_spawn
        .spawn_local(echo_bot(http_recv, sender))
        .unwrap();
    local_pool.run();
}

async fn send_stop_on_stdin_enter(sender: sync::Sender<()>) {
    let stdin = io::stdin();
    let mut line = String::new();
    //we realy dont care why the call finished, we will stop either way.
    stdin.read_line(&mut line).await;
    //we dont even have to send something, dropping it is more than enough
    //but this way we force the compiler into not dropping it earlier (could that even happen?)
    sender.send(());
}

async fn register_web_hook(sender: &TelegramSender, receiver: &TelegramReceiver) -> io::Result<()> {
    sender.register_web_hook(receiver).await?;
    Ok(())
}

fn parse_into_telegram_update(bytes: Vec<u8>) -> Option<telegram_types::Update> {
    let json = String::from_utf8(bytes);
    match json {
        Err(err) => {
            println!("Could not parse body into utf-8 string");
            println!("{}", err);
            None
        }
        Ok(json) => {
            let update = serde_json::from_str(&json);
            match update {
                Err(err) => {
                    println!("Could not deserialize string into Update");
                    println!("{}", err);
                    None
                }
                Ok(update) => Some(update),
            }
        }
    }
}

async fn echo_bot(http_recv: sync::Receiver<telegram_receiver::SendItem>, sender: TelegramSender) {
    while let Some((http_msg, response_channel)) = http_recv.recv().await {
        let body = http_msg.get_body();
        let update = parse_into_telegram_update(body.clone());
        match update {
            Some(update) => {
                if let Some(message) = update.message {
                    if message.from.is_some() && message.text.is_some() {
                        println!("Building method");
                        let method = telegram_methods::sendMessageBuilder::default()
                            .chat_id(Or::A(message.chat.id))
                            .text(message.text.unwrap())
                            .reply_to_message_id(message.message_id)
                            .build()
                            .unwrap();
                        let result = sender.call(&method).await;
                        if result.is_err() {
                            println!("IO Error while sending method {}", result.err().unwrap());
                        } else {
                            let http_msg = result.unwrap();
                            if http_msg.get_response().status().as_u16() != 200 {
                                println!("{:#?}", http_msg);
                            }
                        }
                    } else {
                        println!("Dont know whom or what to answer");
                    }
                } else {
                    println!("message is none");
                }
            }
            None => {}
        }
        //regardless of what the update contained, tell the server we received it (so it doesnt repeat itself).
        let response = rohrpost::http_stream::HttpMsg::new_respone(200);
        response_channel.send(response).await;
        println!("Finished echoing");
    }
    println!("Finished echo bot");
}
