use std::collections::HashMap;

use stylist::Style;

#[derive(PartialEq, Clone, Eq, Hash)]
pub enum Color {
    Primary,
    Secondary,
    Info,
    Highlight,
    Highlight2,
    Error,
    Error2,
    CustomStr(String),
    CustomCss(CssColor)
}

#[derive(PartialEq, Clone, Eq, Hash)]
pub struct CssColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl CssColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Result<Self, String> {
        if a > 100 {
            Err(format!("Value of aplha channel invalid. Expected value in range {{0 - 100}}, got: {}", &a))
        }
        else {
            Ok(Self { r, g, b, a})
        }
    }

    pub fn set_alpha(&mut self, a: u8) -> Self {
        if a > 100 {
            panic!("Value of aplha channel invalid. Expected value in range {{0 - 100}}, got: {}", &a);
        }
        else {
            self.a = a;
            self.clone()
        }
    }
}

impl Color {
    pub fn into_style(&self, target: &str) -> Style {
        Style::new(format!("{}: {};", target, self.get_css_color()))
        .unwrap()
    }

    pub fn get_css_color_struct(&self) -> CssColor {
        let color_values = Self::get_color_values();
        match self {
            Color::CustomStr(_) => todo!(),
            Color::CustomCss(css_color) => css_color.clone(),
            color => color_values.get(color).unwrap().clone()
        }
    }

    pub fn get_css_color(&self) -> String {
        let color_values = Self::get_color_values();
        match self {
            Color::CustomStr(color) => color.to_owned(),
            Color::CustomCss(css_color) => format!("rgba({}, {}, {}, {}%)", css_color.r, css_color.g, css_color.b, css_color.a),
            color => {
                let css_color = color_values.get(color).unwrap();
                format!("rgba({}, {}, {}, {}%)", css_color.r, css_color.g, css_color.b, css_color.a)
            }
        }
    }

    fn get_color_values() -> HashMap<Color, CssColor> {
        HashMap::from([
            (Color::Primary, CssColor::new(142, 202, 230, 100).unwrap()),
            (Color::Secondary, CssColor::new(2, 48, 71, 100).unwrap()),
            (Color::Info, CssColor::new(33, 156, 186, 100).unwrap()),
            (Color::Highlight, CssColor::new(255, 183, 3, 100).unwrap()), 
            (Color::Highlight2, CssColor::new(251, 133, 0, 100).unwrap()),
            (Color::Error, CssColor::new(158, 42, 43, 100).unwrap()),
            (Color::Error2, CssColor::new(213, 47, 49, 100).unwrap())
         ])
    }
}