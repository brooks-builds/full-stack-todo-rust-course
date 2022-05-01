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
        .error-message {
          text-align: center;
          background-color: red;
          animation: 10s fadeinout 2s linear;
          opacity: 0;
        }

        .hide {
          display: none;
        }

        @keyframes fadeinout {
          from {
            opacity: 0;
          }

          5% {
            opacity: 100%;
          }

          95% {
            opacity: 100%;
          }

          to {
            opacity: 0;
          }
        }
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
        let timer_id = timer_id;
        use_effect(move || {
            if !message.is_empty() && timer_id.is_none() {
                let id = {
                    let dispatch = dispatch.clone();
                    let timer_id = timer_id.clone();
                    gloo::timers::callback::Timeout::new(10000, move || {
                        store::reset_error_message(dispatch);
                        timer_id.set(None);
                    })
                    .forget()
                };
                timer_id.set(Some(id));
            }
            || {}
        });
    }
    let hide_class = if message.is_empty() {
        Some("hide")
    } else {
        None
    };

    html! {
      <div class={stylesheet}>
        <div class="error-message">
          <BBText
            text={message}
            data_test="error"
            class={hide_class}
          />
        </div>
      </div>
    }
}
