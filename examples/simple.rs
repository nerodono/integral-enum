use integral_enum::integral_enum;

#[integral_enum(u16)]
pub enum Test {
    Hello,
    Error,
}

fn main() {
    // TryFrom & PartialEq / Eq
    assert_eq!(Test::try_from(0), Ok(Test::Hello));
    assert_eq!(Test::try_from(1), Ok(Test::Error));

    // Copy
    let item = Test::Error;
    let item2 = item;
    assert_eq!(item, item2);

    // Debug
    assert_eq!(format!("{:?}", Test::Hello), "Hello");
    assert_eq!(format!("{:?}", Test::Error), "Error");

    // Ordering
    assert!(Test::Error > Test::Hello);
}
