use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum SoldiersSpriteSheet {
    SelectedSoldiersRed,
    SelectedSoldiersBlue,
    SelectedSoldiersGrey,
    SelectedSoldiersGreen,
    SoldiersRed,
    SoldiersBlue,
    SoldiersGrey,
    SoldiersGreen,
}

impl Display for SoldiersSpriteSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoldiersSpriteSheet::SelectedSoldiersRed => {
                write!(
                    f,
                    "images/soldiers/selected_red_faction_soldiers_tile_sheet.png"
                )
            }
            SoldiersSpriteSheet::SelectedSoldiersBlue => {
                write!(
                    f,
                    "images/soldiers/selected_blue_faction_soldiers_tile_sheet.png"
                )
            }
            SoldiersSpriteSheet::SelectedSoldiersGrey => {
                write!(
                    f,
                    "images/soldiers/selected_grey_faction_soldiers_tile_sheet.png"
                )
            }
            SoldiersSpriteSheet::SelectedSoldiersGreen => {
                write!(
                    f,
                    "images/soldiers/selected_green_faction_soldiers_tile_sheet.png"
                )
            }
            SoldiersSpriteSheet::SoldiersRed => {
                write!(f, "images/soldiers/red_faction_soldiers_tile_sheet.png")
            }
            SoldiersSpriteSheet::SoldiersBlue => {
                write!(f, "images/soldiers/blue_faction_soldiers_tile_sheet.png")
            }
            SoldiersSpriteSheet::SoldiersGrey => {
                write!(f, "images/soldiers/grey_faction_soldiers_tile_sheet.png")
            }
            SoldiersSpriteSheet::SoldiersGreen => {
                write!(f, "images/soldiers/green_faction_soldiers_tile_sheet.png")
            }
        }
    }
}
