use core::fmt::Display;

#[derive(PartialEq)]
pub enum GardenAction {
    Till,
    Plant,
    Harvest,
    BuildScarecrow,
    BuildPlanter,
    PlaceMulch,
    PlaceBirdseed,
    Remove,

    None,
}
impl Display for GardenAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GardenAction::Till => write!(f, "TILL"),
            GardenAction::Plant => write!(f, "PLANT"),
            GardenAction::Harvest => write!(f, "HARVEST"),
            GardenAction::BuildScarecrow => write!(f, "SCARECROW"),
            GardenAction::BuildPlanter => write!(f, "PLANTER"),
            GardenAction::PlaceMulch => write!(f, "MULCH"),
            GardenAction::PlaceBirdseed => write!(f, "BIRDSEED"),
            GardenAction::Remove => write!(f, "REMOVE"),
            GardenAction::None => Err(core::fmt::Error),
        }
    }
}
impl GardenAction {
    pub fn from_usize(i: usize) -> Result<Self, ()> {
        match i {
            0 => Ok(GardenAction::Till),
            1 => Ok(GardenAction::Plant),
            2 => Ok(GardenAction::Harvest),
            3 => Ok(GardenAction::BuildScarecrow),
            4 => Ok(GardenAction::BuildPlanter),
            5 => Ok(GardenAction::PlaceMulch),
            6 => Ok(GardenAction::PlaceBirdseed),
            7 => Ok(GardenAction::Remove),

            _ => Err(()),
        }
    }
}
