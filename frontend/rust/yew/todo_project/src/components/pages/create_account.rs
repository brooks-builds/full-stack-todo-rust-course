use crate::{
    api::auth::auth_service::AuthService,
    components::{
        atoms::{button::Button, text_input::TextInput},
        stores::{auth_store::AuthStore, error_store::ErrorStore}, organisms::error_message::{ErrorMessage, DEFAULT_TIMEOUT_MS},
    },
    router::Route,
    styles::{color::Color, styles::Styles}, SessionStore,
};
use gloo::timers::callback::Timeout;
use lazy_static::__Deref;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(CreateAccount)]
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

    let dispatch = dispatch.clone();
    use_effect(move || {
        move || dispatch.reduce(|_| {AuthStore::default()})
    });

    let (error_store, error_dispatch) = use_store::<ErrorStore>();

    let (_, session_dispatch) = use_store::<SessionStore>();
    let history = use_history().unwrap();

    let onsubmit = {
        let error_dispatch = error_dispatch.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            let error_dispatch = error_dispatch.clone();
            let session_dispatch = session_dispatch.clone();
            let store = store.clone();
            let history = history.clone();
            spawn_local(async move {
                let response =
                    AuthService::register(store.username.clone(), store.password.clone()).await;
                match response {
                    Ok(auth) => {
                        session_dispatch.clone().reduce(|store| {
                            let mut store = store.deref().clone();
                            store.user = Some(auth);
                            store
                        });
                        history.push(Route::Home)
                    }
                    Err(error) => {
                        let error_uuid = Uuid::new_v4();
                        {
                            let error_dispatch = error_dispatch.clone();
                            Timeout::new(DEFAULT_TIMEOUT_MS, move || {
                                error_dispatch
                                    .clone()
                                    .reduce(|store| {
                                        if store.uuid == error_uuid {
                                            ErrorStore::new(String::new(), false, error_uuid).into()
                                        }
                                        else {
                                            store
                                        }
                                    })
                            }).forget();
                        }
                        error_dispatch
                            .clone()
                            .reduce(|_| ErrorStore::new(String::new(), false, error_uuid));

                        error_dispatch
                            .clone()
                            .reduce(|_| ErrorStore::new(error, true, error_uuid));
                    }
                }
            });
        })
    };

    let style = Styles::get_form_style();

    html! {
        <>
        if error_store.display {
            <ErrorMessage message={error_store.message.clone()}/>
        }
        <form class={style} {onsubmit}>
            <h2 class={Color::Info.into_style("color")}>{"Create account"}</h2>
            <TextInput id={"username"} onchange={onchange.clone()} label={"Your username"} placeholder={"enter username.."} data_test={"username"}/>
            <TextInput id={"password"} {onchange} label={"Your password"} input_type={"password"} placeholder={"enter password.."} data_test={"password"}/>
            <Button label={"Create account!"} data_test={"submit"}/>
        </form>
        </>
    }
}
