use stylist::{Style, style};

use super::color::Color;

pub struct Styles;

impl Styles {
    pub fn get_home_style() -> Style {
        style!(
            r#"
            padding: 10px;
            margin: auto;
            display: flex;
            flex-direction: column;
            width: 650px;
            h2 {
                margin-bottom: 20px;
            }
            "#
        )
        .unwrap()
    }

    pub fn get_form_style() -> Style {
        style!(
            r#"
            padding: 10px;
            margin: auto;
            display: flex;
            flex-direction: column;
            width: 650px;
            div {
                padding: 10px;
            }
            button {
                width: 40%;
                min-width: 150px;
                margin: auto;
                height: 36px;
                font-size: 24px;
            }
            "#
        )
        .unwrap()
    }

    pub fn get_navbar_styles(fore_color: Option<Color>, back_color: Option<Color>) -> (Style, Style) {
        
    let fore_color = match fore_color.clone() {
        Some(color) => color.get_css_color(),
        None => Color::Primary.get_css_color(),
    };

    let mut style_string = format!(
        r#"
        padding: 15px 25px;
        border-bottom: 1px solid {fore_color};
        color: {fore_color};
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    "#,
        fore_color = fore_color
    );

    if let Some(back_color) = back_color.clone() {
        style_string = format!("{} background-color: {}", style_string, back_color.get_css_color());
    }

    let style = Style::new(style_string).unwrap();

    let div_style_string = format!(
        r#"
            padding: 0 15px;
        "#
    );

    let div_style = Style::new(div_style_string).unwrap();

    (style, div_style)
    }
}