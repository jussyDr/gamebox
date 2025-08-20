use gamebox::{Challenge, read_file};
use proef::test::Tests;

fn main() {
    let mut tests = Tests::new();

    tests.add("read Alive.Map.Gbx", || {
        if let Err(err) = read_file::<Challenge>("tests/files/Alive.Map.Gbx") {
            panic!("{err}");
        }

        Ok(())
    });

    tests.run();
}
