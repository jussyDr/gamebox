use std::fs::File;

#[test]
fn test() {
    let file = File::open("tests/PlatformBase.Item.Gbx").unwrap();
    gamebox::test(file).unwrap();

    let file = File::create("Test.Item.Gbx").unwrap();
    gamebox::test2(file).unwrap();
}
