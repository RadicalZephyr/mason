use mason::Builder;

#[derive(Builder)]
pub struct Number {
    num: u8,
}

#[test]
fn works() {
    let num = Number::builder();
}
