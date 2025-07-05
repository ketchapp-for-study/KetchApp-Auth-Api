#[derive(Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}
impl Theme {
    pub fn to_string(&self) -> String {
        match self {
            Theme::Light => "light".to_string(),
            Theme::Dark => "dark".to_string(),
        }
    }
}