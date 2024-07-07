#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub fn get_color(&self) -> String {
        return match self {
            Color::White => String::from("W"),
            Color::Black => String::from("B"),
        };
    }
}
