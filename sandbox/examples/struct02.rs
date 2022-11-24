struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
}

fn main() {
    let u = Vector2::UNIT;
    let z = Vector2::ZERO;

    println!("{}", u.x);
    println!("{}", z.x);
}
