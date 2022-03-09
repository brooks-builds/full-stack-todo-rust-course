use std::ops::Deref;

use crate::components::atom::button::{ButtonColor, ButtonWrapper};
use crate::components::atom::text_input_wrapper::{InputType, TextInputWrapper};
use crate::store::bounce::User;
use bounce::use_atom;
use gloo::console::__macro::JsValue;
use gloo::console::log;
use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use stylist::yew::styled_component;
use yew::prelude::*;

pub enum ComponentMessage {
  Username(String),
  Password(String),
  FormSubmitted
}

#[derive(Clone, PartialEq)]
pub struct Login {
  username: Option<String>,
  password: Option<String>,
}

impl Component for Login {
    type Message = ComponentMessage;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
          username: None,
          password: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
    

    html! {
      <section>
        <h1>{"Login"}</h1>
        <form onsubmit={ctx.link().callback(|event: FocusEvent| {
          event.prevent_default();
          ComponentMessage::FormSubmitted
        })
        }>
          <div>
            <TextInputWrapper label="Username" on_change={ctx.link().callback(|username| ComponentMessage::Username(username))} input_type={InputType::Text} />
          </div>
          <div>
            <TextInputWrapper label="Password" on_change={ctx.link().callback(|password| ComponentMessage::Password(password))} input_type={InputType::Password} />
          </div>
          <div>
            <ButtonWrapper label="Login" color={ButtonColor::Success} />
          </div>
        </form>
      </section>
    }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ComponentMessage::Username(username) => self.username = Some(username),
            ComponentMessage::Password(password) => self.password = Some(password),
            ComponentMessage::FormSubmitted => {
              let user_data = UserData{username: self.username.clone().unwrap(), password: self.password.clone().unwrap()};
              wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("http://localhost:3000/api/v1/users/login")
                .body(JsValue::from_serde(&user_data).unwrap())
            .send()
            .await
            .unwrap();
  log!(response.as_raw());  
              })
              // ctx.link().callback_future(|thing| login_to_server(UserData {username: self.username.unwrap(), password: self.password.unwrap()}));
            },
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
// #[styled_component(Login)]
// pub fn login() -> Html {
//     let context = use_context::<yew::Context<Login>>();
//     let user_store = use_atom::<User>();
//     let user_store_clone = user_store.clone();
//     let username_onchange = Callback::from(move |username_value: String| {
//         let mut user = user_store_clone.deref().clone();
//         user.username = Some(username_value);
//         user_store_clone.set(user);
//     });
//     let user_store_clone = user_store.clone();
//     let password_onchange = Callback::from(move |password: String| {
//         let mut user = user_store_clone.deref().clone();
//         user.password = Some(password);
//         user_store_clone.set(user);
//     });
//     let form_onsubmit = Callback::from(move |event: FocusEvent| {
//         event.prevent_default();
//         log!("about to submit form");
//         user_store.deref().clone().login_to_server();
//     });

//     html! {
//       <section>
//         <h1>{"Login"}</h1>
//         <form onsubmit={form_onsubmit}>
//           <div>
//             <TextInputWrapper label="Username" on_change={username_onchange} input_type={InputType::Text} />
//           </div>
//           <div>
//             <TextInputWrapper label="Password" on_change={password_onchange} input_type={InputType::Password} />
//           </div>
//           <div>
//             <ButtonWrapper label="Login" color={ButtonColor::Success} />
//           </div>
//         </form>
//       </section>
//     }
// }

#[derive(Serialize, Deserialize)]
struct UserData {
  username: String,
  password: String
}

async fn login_to_server(user: UserData) {
  let response = Request::post("http://localhost:3000/api/v1/users/login")
            .body(JsValue::from_serde(&user).unwrap())
            .send()
            .await
            .unwrap();
  log!(response.as_raw());          
}