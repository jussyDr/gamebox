#![warn(clippy::todo, clippy::unwrap_used)]

pub mod read;

pub use read::{read, read_file};

pub mod game {
    pub mod ctn {
        mod challenge;
        mod challenge_parameters;
        mod collector_list;
        mod ghost;

        pub use challenge::Challenge;
        pub use challenge_parameters::ChallengeParameters;
        pub use collector_list::CollectorList;
        pub use ghost::Ghost;
    }
}

pub use game::ctn::Challenge;
