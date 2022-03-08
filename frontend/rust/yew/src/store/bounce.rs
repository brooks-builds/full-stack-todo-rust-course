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
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
}

impl User {
    pub async fn login_to_server(&mut self) {
        log!(self.username.clone());
        let response = Request::post("http://nodejs-express:3000/api/v1/users/login")
            .body(JsValue::from_serde(self).unwrap())
            .send()
            .await
            .unwrap();
        log!(response.as_raw());
        self.password = None;
    }
}
