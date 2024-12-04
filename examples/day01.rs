use advent2024_rs::Advent;

/// DISTANCE = 3246517
fn main() {

    let advent = Advent::default();

    let distance = advent.get_distance();

    println!("DISTANCE = {distance}");

    let mappie = advent.simularity_map();

    for (k, v) in mappie.iter() {
        println!("{k} = {v}");
    }
}
