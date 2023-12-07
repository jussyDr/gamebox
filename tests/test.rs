use std::fs::File;

use gamebox::classes::item::Item;

#[test]
fn read_item() {
    let file = File::open("tests/big_palm_tree_low.Item.Gbx").unwrap();
    let _item: Item = gamebox::read(file).unwrap();
}
