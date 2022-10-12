use stylist::yew::styled_component;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::SessionStore;
use crate::components::atoms::route_link::RouteLink;
use crate::router::Route;
use crate::styles::color::Color;
use crate::styles::styles::Styles;

#[derive(Properties, PartialEq)]
pub struct NavbarProperties {
    pub data_test: Option<String>,
    pub fore_color: Option<Color>,
    pub back_color: Option<Color>,
}

#[styled_component(Navbar)]
pub fn navbar(props: &NavbarProperties) -> Html {
    let (store, _) = use_store::<SessionStore>();
    let (style, div_style) = Styles::get_navbar_styles(props.fore_color.clone(), props.back_color.clone());

    html! {
        <section class={style}>
            <div class={div_style.clone()}>
                <RouteLink
                    text={"My TODO App"}
                    link={Route::Home}
                    data_test={"logo"}
                    fore_color={props.fore_color.clone()}
                    back_color={props.back_color.clone()}/>
            </div>
            if let None = store.user {
                <div class={div_style}>
                    <RouteLink
                        text={"Login"}
                        link={Route::Login}
                        data_test={"login"}
                        fore_color={props.fore_color.clone()}
                        back_color={props.back_color.clone()}/>
                    <RouteLink
                        text={"Create Account"}
                        link={Route::CreateAccount}
                        data_test={"create-account"}
                        fore_color={Color::Highlight}
                        back_color={props.back_color.clone()}
                        hover_color={Color::Highlight2}/>
                </div>
            }
        </section>
    }
}
