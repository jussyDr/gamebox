use std::fs::File;

use gamebox::item::Item;

#[test]
fn test() {
    let file = File::open("tests/PlatformBase.Item.Gbx").unwrap();
    let item: Item = gamebox::read(file).unwrap();
}
