#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = Vec::new();
    grains.push(Cereal::Rye);
    // drop() moves its ownership.
    // drop(grains);

    println!("{:?}", grains);
}