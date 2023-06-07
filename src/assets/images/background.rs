use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum BackgroundSpriteSheet {
    Background,
}

impl Display for BackgroundSpriteSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackgroundSpriteSheet::Background => {
                write!(f, "images/background/background_tile_sheet.png")
            }
        }
    }
}
