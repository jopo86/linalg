use linalg::prelude::*;

fn main() {
    let mut vec = Vec2::new(5.0, 5.0);
    vec -= Vec2::new(1.0, 2.0);
    println!("{:?}", vec);
}
