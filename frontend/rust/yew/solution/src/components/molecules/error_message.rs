use crate::{
    components::atoms::bb_text::BBText,
    store::{self, StoreType},
};
use stylist::yew::styled_component;
use yew::prelude::*;
use yewdux_functional::use_store;

#[styled_component(ErrorMessage)]
pub fn error_message() -> Html {
    let stylesheet = css!(
        r#"
      text-align: center;
      background-color: red;
    "#
    );
    let message = use_store::<StoreType>()
        .state()
        .map(|state| state.error_message.clone())
        .unwrap_or_default();
    let timer_id = use_state(|| None);
    {
        let message = message.clone();
        let dispatch = use_store::<StoreType>().dispatch().clone();
        let timer_id = timer_id.clone();
        use_effect(move || {
            if !message.is_empty() && timer_id.is_none() {
                let id = {
                    let dispatch = dispatch.clone();
                    let timer_id = timer_id.clone();
                    let id = gloo::timers::callback::Timeout::new(10000, move || {
                        store::reset_error_message(dispatch);
                        timer_id.set(None);
                    })
                    .forget();
                    id
                };
                timer_id.set(Some(id));
            }
            || {}
        });
    }
    html! {
      <div class={stylesheet}>
        if !message.is_empty() {
          <BBText
            text={message}
            data_test="error"
          />
        }
      </div>
    }
}
