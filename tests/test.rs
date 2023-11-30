use std::fs::File;

use gamebox::classes::item::Item;

#[test]
fn test() {
    let file = File::open("tests/PlatformBase.Item.Gbx").unwrap();
    let _item: Item = gamebox::read(file).unwrap();
}
