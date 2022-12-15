#[derive(Debug)]
struct S<'a> {
    r: &'a i32,
}
fn main() {
    let s;
    {
        let x = 10;
        s = S { r: &x };
        println!("{:?}", s);
    }
}
