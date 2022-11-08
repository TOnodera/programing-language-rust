#[derive(Debug)]
struct S {
    r: &i32,
}
fn main() {
    let s;
    {
        let x = 10;
        s = S { r: &x };
        println!("{:?}", s);
    }
}
