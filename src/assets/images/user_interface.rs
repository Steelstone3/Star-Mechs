use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum UserInterfaceSpriteSheet {
    UserInterface,
}

impl Display for UserInterfaceSpriteSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserInterfaceSpriteSheet::UserInterface => {
                write!(f, "images/user_interface/user_interface_tile_sheet.png")
            }
        }
    }
}
