use advent2024_rs::Advent;

/// DISTANCE = 3246517
fn main() {

    let (v1, v2) = Advent::get_vecs();

    let mut distance: isize = 0;

    for i in 0..v1.len() {

        let u1 = v1[i];
        let u2 = v2[i];

        let diff: isize = diff(u1, u2);
        println!("{u1} - {u2} = {diff}");

        distance = distance + diff
    }

    println!("DISTANCE = {distance}");
}

fn diff(u1: isize, u2: isize) -> isize {
    (u1 - u2).abs()
}