use crate::game::weapons::component::WeaponComponent;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct SmallArmsProperty {
    #[serde(default)]
    recoil: f32,
    #[serde(default)]
    range: f32,
}

#[derive(Deserialize, Debug)]
pub struct SmallArmsComponents {
    #[serde(rename = "barrel")]
    barrels: Vec<WeaponComponent<SmallArmsProperty>>,

    #[serde(rename = "body")]
    bodies: Vec<WeaponComponent<SmallArmsProperty>>,

    #[serde(rename = "stock")]
    stocks: Vec<WeaponComponent<SmallArmsProperty>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_COMPONENTS_TOML_FILE: &'static str = r#"
[[barrel]]
name = "Barrel 1"
[barrel.statistics]
range = 20
recoil = 10
[barrel.costs]
wood = 1
metal = 2
[barrel.asset]
filename = "barrel_1.png"
offset = { x = 0, y = -1 }


[[barrel]]
name = "Barrel 2"
[barrel.statistics]
range = 10
recoil = 5
[barrel.costs]
metal = 1
[barrel.asset]
filename = "barrel_2.png"
offset = { x = 0, y = -2 }


[[body]]
name = "Body 1"
[body.asset]
filename = "body_1.png"
[body.statistics]
range = 10
recoil = 5
[body.costs]
metal = 1



[[stock]]
name = "Stock 1"
[stock.asset]
filename = "stock_1.png"
offset = { x = 0, y = -4 }
[stock.statistics]
range = 10
recoil = 5
[stock.costs]
metal = 1


[[stock]]
name = "Stock 2"
[stock.asset]
filename = "stock_2.png"
offset = { x = 0, y = -1 }
[stock.statistics]
range = 10
recoil = 5
[stock.costs]
metal = 1
"#;

    #[test]
    fn serde_deserialize() {
        let test_deserialize: SmallArmsComponents =
            toml::from_str(TEST_COMPONENTS_TOML_FILE).unwrap();

        println!("{test_deserialize:#?}")
    }
}
