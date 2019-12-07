#![recursion_limit = "256"]
#[macro_use]
extern crate futures;
#[macro_use]
extern crate derive_builder;

pub mod helpers;
pub mod http_stream;
pub mod telegram_methods;
pub mod telegram_receiver;
pub mod telegram_sender;
pub mod telegram_types;

pub use telegram_receiver::TelegramReceiver;
pub use telegram_sender::TelegramSender;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
