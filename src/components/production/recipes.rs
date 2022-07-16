// use crate::components::production::{Goods, GoodsQuantity, Producible};
// use std::collections::HashMap;
//
// pub struct Recipe {
//     inputs: GoodsQuantity,
//     outputs: GoodsQuantity,
//     time: f32,
// }
//
// impl Recipe {
//     pub fn raw_good(output: Goods, time: f32) -> Recipe {
//         Recipe {
//             inputs: HashMap::new(),
//             outputs: HashMap::new(),
//             time,
//         }
//     }
//
//     pub fn new<T, U>(inputs: T, outputs: U, time: f32) -> Self
//     where
//         T: Iterator + FromIterator<GoodsQuantity>,
//         U: Iterator + FromIterator<GoodsQuantity>,
//     {
//         Recipe {
//             inputs: inputs.into_iter().collect(),
//             outputs: outputs.into_iter().collect(),
//             time,
//         }
//     }
//
//     pub fn inputs(&self) -> &GoodsQuantity {
//         &self.inputs
//     }
//
//     pub fn outputs(&self) -> &GoodsQuantity {
//         &self.outputs
//     }
//
//     pub fn time(&self) -> f32 {
//         self.time
//     }
// }
