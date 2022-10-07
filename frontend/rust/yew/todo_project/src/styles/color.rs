use stylist::Style;

#[derive(PartialEq, Clone)]
pub enum Color {
    Primary,
    Secondary,
    Info,
    Highlight,
    Error,
    Custom(String)
}

impl Color {
    pub fn into_style(&self, target: &str) -> Style {
        Style::new(format!("{}: {};", target, self.get_css_color()))
        .unwrap()
    }

    pub fn get_css_color(&self) -> String {
        match self {
            Color::Primary => "var(--primary)".to_owned(),
            Color::Secondary => "var(--secondary)".to_owned(),
            Color::Info => "var(--info)".to_owned(),
            Color::Highlight => "var(--highlight)".to_owned(),
            Color::Error => "var(--error)".to_owned(),
            Color::Custom(color) => color.to_owned(),
        }
    }
}