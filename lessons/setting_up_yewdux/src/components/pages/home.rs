use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div>
        <h1>{"Home"}</h1>
        <div>
                <Link<Route> to={Route::Count}>{"Count"}</Link<Route>>
            </div>
            <div>
                <Link<Route> to={Route::Display}>{"Display"}</Link<Route>>
            </div>
      </div>
    }
}
