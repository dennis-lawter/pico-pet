use core::fmt::Display;

#[derive(PartialEq)]
pub enum GardenAction {
    Till,
    Plant,
    Prune,
    Harvest,
    BuildScarecrow,
    BuildPlanter,
    PlaceMulch,
    PlaceBirdseed,
    Destroy,

    None,
}
impl Display for GardenAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GardenAction::Till => write!(f, "TILL"),
            GardenAction::Plant => write!(f, "PLANT"),
            GardenAction::Prune => write!(f, "PRUNE"),
            GardenAction::Harvest => write!(f, "HARVEST"),
            GardenAction::BuildScarecrow => write!(f, "SCARECROW"),
            GardenAction::BuildPlanter => write!(f, "PLANTER"),
            GardenAction::PlaceMulch => write!(f, "MULCH"),
            GardenAction::PlaceBirdseed => write!(f, "BIRDSEED"),
            GardenAction::Destroy => write!(f, "DESTROY"),
            GardenAction::None => Err(core::fmt::Error),
        }
    }
}
impl GardenAction {
    pub fn from_usize(i: usize) -> Result<Self, ()> {
        match i {
            0 => Ok(GardenAction::Till),
            1 => Ok(GardenAction::Plant),
            2 => Ok(GardenAction::Prune),
            3 => Ok(GardenAction::Harvest),
            4 => Ok(GardenAction::BuildScarecrow),
            5 => Ok(GardenAction::BuildPlanter),
            6 => Ok(GardenAction::PlaceMulch),
            7 => Ok(GardenAction::PlaceBirdseed),
            8 => Ok(GardenAction::Destroy),

            _ => Err(()),
        }
    }
}
