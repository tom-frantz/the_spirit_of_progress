use serde::Deserialize;

#[derive(Deserialize)]
struct Barrel {
    name: String,
    filename: String,
}
