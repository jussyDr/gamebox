use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    game::ctn::{ChallengeParameters, CollectorList},
    read::{BodyChunksReader, BodyReader, Error, Readable},
};

pub struct Challenge(Inner);

#[self_referencing]
struct Inner {
    header_data: Box<[u8]>,
    body_data: Arc<[u8]>,
    node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    #[borrows(header_data, body_data, mut node_refs)]
    #[covariant]
    chunks: Chunks<'this>,
}

struct Chunks<'a> {
    delme: PhantomData<&'a ()>,
    chunk_2: Chunk2,
    chunk_3: Chunk3,
    chunk_4: Chunk4,
    chunk_5: Chunk5,
    chunk_7: Chunk7,
    chunk_8: Chunk8,
    chunk_13: Chunk13,
    chunk_17: Chunk17,
    chunk_24: Chunk24,
    chunk_25: Chunk25,
    chunk_31: Chunk31,
    chunk_34: Chunk34,
    chunk_36: Chunk36,
    chunk_37: Chunk37,
    chunk_38: Chunk38,
    chunk_40: Chunk40,
    chunk_41: Chunk41,
    chunk_42: Chunk42,
    chunk_52: Chunk52,
    chunk_54: Chunk54,
    chunk_56: Chunk56,
    chunk_62: Chunk62,
    chunk_64: Chunk64,
    chunk_66: Chunk66,
    chunk_67: Chunk67,
    chunk_68: Chunk68,
    chunk_72: Chunk72,
    chunk_73: Chunk73,
    chunk_75: Chunk75,
    chunk_79: Chunk79,
    chunk_80: Chunk80,
    chunk_81: Chunk81,
    chunk_82: Chunk82,
    chunk_83: Chunk83,
    chunk_84: Chunk84,
    chunk_85: Chunk85,
    chunk_86: Chunk86,
    chunk_87: Chunk87,
    chunk_88: Chunk88,
    chunk_89: Chunk89,
    chunk_90: Chunk90,
    chunk_91: Chunk91,
    chunk_92: Chunk92,
    chunk_93: Chunk93,
    chunk_94: Chunk94,
    chunk_95: Chunk95,
    chunk_96: Chunk96,
}

struct Chunk2;

struct Chunk3;

struct Chunk4;

struct Chunk5;

struct Chunk7;

struct Chunk8;

struct Chunk13;

struct Chunk17;

struct Chunk24;

struct Chunk25;

struct Chunk31;

struct Chunk34;

struct Chunk36;

struct Chunk37;

struct Chunk38;

struct Chunk40;

struct Chunk41;

struct Chunk42;

struct Chunk52;

struct Chunk54;

struct Chunk56;

struct Chunk62;

struct Chunk64;

struct Chunk66;

struct Chunk67;

struct Chunk68;

struct Chunk72;

struct Chunk73;

struct Chunk75;

struct Chunk79;

struct Chunk80;

struct Chunk81;

struct Chunk82;

struct Chunk83;

struct Chunk84;

struct Chunk85;

struct Chunk86;

struct Chunk87;

struct Chunk88;

struct Chunk89;

struct Chunk90;

struct Chunk91;

struct Chunk92;

struct Chunk93;

struct Chunk94;

struct Chunk95;

struct Chunk96;

impl Readable for Challenge {
    fn read_from_header_and_body(
        header_data: Box<[u8]>,
        body_data: Arc<[u8]>,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    ) -> Result<Self, Error> {
        let builder = InnerTryBuilder {
            header_data,
            body_data,
            node_refs,
            chunks_builder: |header_data, body_data, node_refs| {
                let mut body_data_offset = 0;

                let mut r = BodyChunksReader(BodyReader::new(
                    Arc::clone(body_data),
                    Arc::clone(node_refs),
                    body_data,
                    node_refs,
                    &mut body_data_offset,
                ));

                let chunk_13 = r.chunk(0x0304300d, |r| {
                    let _player_model = r.id_or_null()?;
                    let _player_model_collection = r.id_or_null()?;
                    let _player_model_author = r.id_or_null()?;

                    Ok(Chunk13)
                })?;

                let chunk_17 = r.chunk(0x03043011, |r| {
                    let _block_stock = r.node_ref::<CollectorList>()?;
                    let _challenge_parameters = r.node_ref::<ChallengeParameters>()?;
                    let _kind = r.u32()?;

                    Ok(Chunk17)
                })?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_2: Chunk2,
                    chunk_3: Chunk3,
                    chunk_4: Chunk4,
                    chunk_5: Chunk5,
                    chunk_7: Chunk7,
                    chunk_8: Chunk8,
                    chunk_13,
                    chunk_17,
                    chunk_24: Chunk24,
                    chunk_25: Chunk25,
                    chunk_31: Chunk31,
                    chunk_34: Chunk34,
                    chunk_36: Chunk36,
                    chunk_37: Chunk37,
                    chunk_38: Chunk38,
                    chunk_40: Chunk40,
                    chunk_41: Chunk41,
                    chunk_42: Chunk42,
                    chunk_52: Chunk52,
                    chunk_54: Chunk54,
                    chunk_56: Chunk56,
                    chunk_62: Chunk62,
                    chunk_64: Chunk64,
                    chunk_66: Chunk66,
                    chunk_67: Chunk67,
                    chunk_68: Chunk68,
                    chunk_72: Chunk72,
                    chunk_73: Chunk73,
                    chunk_75: Chunk75,
                    chunk_79: Chunk79,
                    chunk_80: Chunk80,
                    chunk_81: Chunk81,
                    chunk_82: Chunk82,
                    chunk_83: Chunk83,
                    chunk_84: Chunk84,
                    chunk_85: Chunk85,
                    chunk_86: Chunk86,
                    chunk_87: Chunk87,
                    chunk_88: Chunk88,
                    chunk_89: Chunk89,
                    chunk_90: Chunk90,
                    chunk_91: Chunk91,
                    chunk_92: Chunk92,
                    chunk_93: Chunk93,
                    chunk_94: Chunk94,
                    chunk_95: Chunk95,
                    chunk_96: Chunk96,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
