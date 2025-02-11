use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::Deref;

/// API Key type.
/// Newtype of String.
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Key (pub String);

impl Deref for Key {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// API Value type.
#[derive(Debug, Clone)]
pub enum Value {
    // Integer(i64),
    // Text(String),
    Bytes(Vec<u8>),
}



// static mut STATE: State = State::default();
// static mut STATE: Box<State> = Box::new(State::default());