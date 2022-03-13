use bounce::Atom;
use gloo::console::{__macro::JsValue, log};
use reqwasm::http::Request;
use serde::Serialize;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Default, PartialEq, Atom)]
pub struct TextInput {
    pub inner: String,
}

#[derive(Default, PartialEq, Atom, Clone, Serialize)]
pub struct User {
    pub username: String,
    pub token: String,
}
