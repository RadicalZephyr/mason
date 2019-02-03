use mason::Builder;

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Number {
    num: u8,
}

#[test]
fn works() {
    let actual = Number::builder().num(12).build();
    assert_eq!(Number { num: 12 }, actual);
}
