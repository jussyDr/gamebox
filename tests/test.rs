use gamebox::{Challenge, read_file};
use proef::test::Tests;

fn main() {
    let mut tests = Tests::new();

    tests.add("read Test.Map.Gbx", || {
        if let Err(err) =
            read_file::<Challenge>("C:/Users/Justin/Documents/Trackmania/Maps/My Maps/Test.Map.Gbx")
        {
            panic!("{err}");
        }

        Ok(())
    });

    tests.add("read Test.Map.Gbx", || {
        if let Err(err) = read_file::<Challenge>("tests/files/Test.Map.Gbx") {
            panic!("{err}");
        }

        Ok(())
    });

    tests.add("read Alive.Map.Gbx", || {
        if let Err(err) = read_file::<Challenge>("tests/files/Alive.Map.Gbx") {
            panic!("{err}");
        }

        Ok(())
    });

    tests.run();
}
