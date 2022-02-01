#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Image {
    height: u64,
    width: u64,
    rgb: Vec<Vec<Color>>,
}
