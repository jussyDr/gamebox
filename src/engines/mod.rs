//! All GameBox engines.

pub mod control {
    //! The control engine.

    pub(crate) mod effect_simi;

    #[doc(inline)]
    pub use effect_simi::EffectSimi;
}

pub mod game {
    //! The game engine.

    pub(crate) mod anchored_object;
    pub(crate) mod block;
    pub(crate) mod block_skin;
    pub(crate) mod challenge;
    pub(crate) mod challenge_parameters;
    pub(crate) mod collector_list;
    pub(crate) mod ghost;
    pub(crate) mod media_block;
    pub(crate) mod media_block_camera_custom;
    pub(crate) mod media_block_color_grading;
    pub(crate) mod media_block_fog;
    pub(crate) mod media_block_fx_colors;
    pub(crate) mod media_block_image;
    pub(crate) mod media_block_interface;
    pub(crate) mod media_block_manialink;
    pub(crate) mod media_block_sound;
    pub(crate) mod media_block_text;
    pub(crate) mod media_block_tone_mapping;
    pub(crate) mod media_block_transition_fade;
    pub(crate) mod media_block_triangles;
    pub(crate) mod media_block_triangles_2d;
    pub(crate) mod media_block_triangles_3d;
    pub(crate) mod media_clip;
    pub(crate) mod media_clip_group;
    pub(crate) mod media_track;
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
    pub use media_block::MediaBlock;
    #[doc(inline)]
    pub use media_block_camera_custom::MediaBlockCameraCustom;
    #[doc(inline)]
    pub use media_block_color_grading::MediaBlockColorGrading;
    #[doc(inline)]
    pub use media_block_fog::MediaBlockFog;
    #[doc(inline)]
    pub use media_block_fx_colors::MediaBlockFxColors;
    #[doc(inline)]
    pub use media_block_image::MediaBlockImage;
    #[doc(inline)]
    pub use media_block_interface::MediaBlockInterface;
    #[doc(inline)]
    pub use media_block_manialink::MediaBlockManialink;
    #[doc(inline)]
    pub use media_block_sound::MediaBlockSound;
    #[doc(inline)]
    pub use media_block_text::MediaBlockText;
    #[doc(inline)]
    pub use media_block_tone_mapping::MediaBlockToneMapping;
    #[doc(inline)]
    pub use media_block_transition_fade::MediaBlockTransitionFade;
    #[doc(inline)]
    pub use media_block_triangles::MediaBlockTriangles;
    #[doc(inline)]
    pub use media_block_triangles_2d::MediaBlockTriangles2D;
    #[doc(inline)]
    pub use media_block_triangles_3d::MediaBlockTriangles3D;
    #[doc(inline)]
    pub use media_clip::MediaClip;
    #[doc(inline)]
    pub use media_clip_group::MediaClipGroup;
    #[doc(inline)]
    pub use media_track::MediaTrack;
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
