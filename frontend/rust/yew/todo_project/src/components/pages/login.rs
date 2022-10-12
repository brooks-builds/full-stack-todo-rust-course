use crate::{
    api::auth::auth_service::AuthService,
    components::{
        atoms::{button::Button, text_input::TextInput},
        stores::auth_store::AuthStore,
    },
    router::Route,
    styles::{color::Color, styles::Styles}, SessionStore,
};
use gloo::console::log;
use lazy_static::__Deref;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Login)]
pub fn create_account() -> Html {
    let (store, dispatch) = use_store::<AuthStore>();

    let onchange = dispatch
        .clone()
        .reduce_mut_callback_with(|store, event: Event| {
            let target_element = event.target_unchecked_into::<HtmlInputElement>();
            match target_element.id().as_str() {
                "username" => store.username = target_element.value(),
                "password" => store.password = target_element.value(),
                _ => (),
            }
        });

    let (_, session_dispatch) = use_store::<SessionStore>();
    let history = use_history().unwrap();

    let onsubmit = {
        let session_dispatch = session_dispatch.clone();
        let store = store.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            let session_dispatch = session_dispatch.clone();
            let store = store.clone();
            let history = history.clone();
            spawn_local(async move {
                let response =
                    AuthService::login(store.username.clone(), store.password.clone()).await;
                match response {
                    Ok(auth) => {
                        session_dispatch.clone().reduce(|store| {
                            let mut store = store.deref().clone();
                            store.user = Some(auth);
                            store
                        });
                        history.push(Route::Home)
                    }
                    Err(error) => log!(format!("login failed, details: {}", error)),
                }
            });
        })
    };

    let style = Styles::get_form_style();

    html! {
        <form class={style} {onsubmit}>
            <h2 class={Color::Info.into_style("color")}>{"Login"}</h2>
            <TextInput id={"username"} onchange={onchange.clone()} label={"Your username"} placeholder={"enter username.."} data_test={"username"}/>
            <TextInput id={"password"} {onchange} label={"Your password"} input_type={"password"} placeholder={"enter password.."} data_test={"password"}/>
            <Button label={"Log in!"} data_test={"submit"}/>
        </form>
    }
}
