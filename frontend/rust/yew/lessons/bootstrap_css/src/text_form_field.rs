use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: String,
    pub label: String,
    pub placeholder: Option<String>,
    pub description: String,
}

#[function_component(TextFormField)]
pub fn text_form_field(props: &Props) -> Html {
    let help_id = format!("{}-help", &props.id);

    html! {
        <>
            <label for={props.id.clone()} class="form-label">{ &props.label }</label>
            <input type="text" class="form-control" id={props.id.clone()} placeholder={props.placeholder.clone().unwrap_or_default()} aria-describedby={help_id.clone()} />
            <span class="input-group-text visually-hidden" id={help_id}>{ &props.description }</span>
        </>
    }
}
