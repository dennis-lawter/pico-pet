// pub struct Inventory {
//     pub tomatoes: u8,
//     pub raspberries: u8,
// }
// impl Inventory {
//     pub fn load_or_write_default() -> Self {
//         match Self::try_load() {
//             Some(inventory) => inventory,
//             None => Self {
//                 tomatoes: 42,
//                 raspberries: 255,
//             },
//         }
//     }

//     pub fn try_load() -> Option<Self> {
//         None // TODO: reserve NVM
//     }
// }
