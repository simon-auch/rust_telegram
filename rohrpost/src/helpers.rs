//use core::marker::PhantomData;
//use serde::de::{Deserializer, Visitor};
//use serde::ser::{Serialize, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Or<T, U> {
    A(T),
    B(U),
}
