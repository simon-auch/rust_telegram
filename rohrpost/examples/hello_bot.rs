extern crate rohrpost;

use async_std::io;
use async_std::sync::Receiver;
use futures::channel::oneshot;
use futures::executor::{block_on, LocalPool};
use futures::task::LocalSpawn;
use futures::task::LocalSpawnExt;
use rohrpost::helpers::Or;
use rohrpost::receiver;
use rohrpost::sender;
use rohrpost::telegram_methods;
use rohrpost::telegram_types;
use std::net::SocketAddr;

const TOKEN: &'static str = "1066698743:AAEbuPawIIjn9merOarOkZE688qk0WOFfxc";

fn main() {
    let config = sender::Config::new(String::from(TOKEN));
    let sender = sender::Sender::new(config);
    let config = receiver::Config::new(
        SocketAddr::from(([0, 0, 0, 0], 8443)),
        String::from("fullchain.pem"),
        String::from("privkey.pem"),
        String::from("https://auch.dnshome.de:8443/somesecretpath"),
    );
    let (recv, stop, out) = receiver::Receiver::new(config);

    //register webhook so we receive updates
    println!("Registering webhook");
    block_on(register_web_hook(&sender, &recv)).unwrap();

    println!("Creating Thread pool");
    let mut local_pool = LocalPool::new();
    let local_spawn = local_pool.spawner();
    println!("Spawn stdin future");
    local_spawn.spawn_local(send_stop_on_stdin(stop)).unwrap();
    println!("Spawn receiver future");
    local_spawn
        .spawn_local(recv.run(local_spawn.clone()))
        .unwrap();
    println!("Spawn echo_bot future");
    local_spawn
        .spawn_local(echo_bot(local_spawn.clone(), out, sender))
        .unwrap();
    local_pool.run();
}

async fn send_stop_on_stdin(sender: oneshot::Sender<()>) {
    let stdin = io::stdin();
    let mut line = String::new();
    //we realy dont care why the call finished, we will stop either way.
    stdin.read_line(&mut line).await;
    sender.send(()).unwrap();
}

async fn register_web_hook(
    sender: &sender::Sender,
    receiver: &receiver::Receiver,
) -> io::Result<()> {
    sender.register_web_hook(receiver).await?;
    Ok(())
}

async fn echo_bot<S>(executor: S, receiver: Receiver<receiver::SendItem>, sender: sender::Sender)
where
    S: LocalSpawn,
{
    while let Some((mut http_stream, http_msg)) = receiver.recv().await {
        let body = http_msg.get_body();
        let json = String::from_utf8(body.clone());
        if json.is_err() {
            println!("Could not parse body into utf-8 string");
            continue;
        }
        let json = json.unwrap();
        let update = serde_json::from_str(&json);
        if update.is_err() {
            println!("Could not deserialize string into Update");
            continue;
        }
        let update: telegram_types::Update = update.unwrap();
        if let Some(message) = update.message {
            if message.from.is_none() || message.text.is_none() {
                println!("Cannot respond without sender or text");
                continue;
            }
            println!("Building method");
            let method = telegram_methods::sendMessageBuilder::default()
                .chat_id(Or::A(message.chat.id))
                .text(message.text.unwrap())
                .reply_to_message_id(message.message_id)
                .build()
                .unwrap();
            let result = sender.call(&method).await;
            if result.is_err() {
                println!("{}", result.err().unwrap());
            } else {
                let http_msg = result.unwrap();
                if http_msg.get_response().status().as_u16() != 200 {
                    println!("{:#?}", http_msg);
                } else {
                    let response = rohrpost::http_stream::HttpMsg::new_respone(200);
                    let result = http_stream.write(response).await;
                }
            }
        } else {
            println!("message is none");
        }
    }
}
