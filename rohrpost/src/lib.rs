#[macro_use]
extern crate futures;
#[macro_use]
extern crate derive_builder;

pub mod helpers;
pub mod http_stream;
pub mod receiver;
pub mod sender;
pub mod telegram_methods;
pub mod telegram_types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
