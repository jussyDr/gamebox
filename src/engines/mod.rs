//! All GameBox engines.

pub mod game {
    //! The game engine.

    pub(crate) mod anchored_object;
    pub(crate) mod block;
    pub(crate) mod block_skin;
    pub(crate) mod challenge;
    pub(crate) mod challenge_parameters;
    pub(crate) mod collector_list;
    pub(crate) mod ghost;
    pub(crate) mod zone_genealogy;

    #[doc(inline)]
    pub use anchored_object::AnchoredObject;
    #[doc(inline)]
    pub use block::Block;
    #[doc(inline)]
    pub use block_skin::BlockSkin;
    #[doc(inline)]
    pub use challenge::Challenge;
    #[doc(inline)]
    pub use challenge_parameters::ChallengeParameters;
    #[doc(inline)]
    pub use collector_list::CollectorList;
    #[doc(inline)]
    pub use ghost::Ghost;
    #[doc(inline)]
    pub use zone_genealogy::ZoneGenealogy;
}

pub mod game_data {
    //! The game data engine.

    pub(crate) mod waypoint_special_property;

    #[doc(inline)]
    pub use waypoint_special_property::WaypointSpecialProperty;
}

pub mod scene {
    //! The scene engine.

    pub(crate) mod vehicle_car_marks_samples;

    #[doc(inline)]
    pub use vehicle_car_marks_samples::VehicleCarMarksSamples;
}

pub mod script {
    //! The script engine.

    pub(crate) mod traits_metadata;

    #[doc(inline)]
    pub use traits_metadata::TraitsMetadata;
}
