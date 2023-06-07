use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum MechsSpriteSheet {
    Scout,
    SelectedScout,
}

impl Display for MechsSpriteSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MechsSpriteSheet::Scout => {
                write!(f, "images/mechs/scout_tile_sheet.png")
            }
            MechsSpriteSheet::SelectedScout => {
                write!(f, "images/mechs/selected_scout_tile_sheet.png")
            }
        }
    }
}
