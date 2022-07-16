// use crate::components::production::recipes::Recipe;
// use crate::components::production::Goods::*;
// use bevy::prelude::*;
// use std::collections::HashMap;
//
// pub mod recipes;
//
// #[derive(Clone, Copy, Debug)]
// pub enum Goods {
//     Iron,
//     Wood,
//     Coal,
//     Copper,
//
//     Steel,
//     MachineParts,
//
//     Guns,
//     Artillery,
// }
//
// type GoodsQuantity = HashMap<Goods, f32>;
//
// impl Goods {
//     fn recipe(&self) -> Recipe {
//         match self {
//             Iron => Recipe::raw_good(Iron, 6.0),
//             Wood => Recipe::raw_good(Wood, 1.0),
//             Coal => Recipe::raw_good(Coal, 3.0),
//             Copper => Recipe::raw_good(Copper, 5.0),
//             Steel => Recipe::new(vec![(Iron, 10.0), (Coal, 1.0)], vec![(Steel, 8.0)], 10.0),
//             MachineParts => Recipe::new(
//                 vec![(Steel, 5.0), (Copper, 1.0)],
//                 vec![(MachineParts, 1.0)],
//                 15.0,
//             ),
//             Guns => Recipe::new(vec![(Iron, 1.0), (Wood, 1.0)], vec![(Guns, 1.0)], 3.0),
//             Artillery => Recipe::new(
//                 vec![(Steel, 10.0), (MachineParts, 3.0)],
//                 vec![(Artillery, 1.0)],
//                 10.0,
//             ),
//         }
//     }
// }
//
// trait Producible {
//     fn recipe(&self) -> Recipe;
//
//     fn consumed_goods(&self) -> GoodsQuantity;
//     fn consume_good(&mut self, goods: GoodsQuantity);
//
//     fn create_goods(&mut self, store: GoodsQuantity) -> GoodsQuantity {
//         // let recipe;
//
//         unimplemented!()
//     }
// }
//
// #[derive(Component)]
// pub struct Producer {
//     produces: Vec<Recipe>,
// }
