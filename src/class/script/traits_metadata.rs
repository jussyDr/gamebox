//! Traits metadata.

use crate::ClassId;

/// Traits metadata.
#[derive(Default)]
pub struct TraitsMetadata;

impl ClassId for TraitsMetadata {
    const CLASS_ID: u32 = 0x11002000;
}

enum ScriptType {
    Void,
    Integer,
    Array {
        key: Box<ScriptType>,
        value: Box<ScriptType>,
    },
}

mod read {
    use crate::{
        class::script::traits_metadata::{ScriptType, TraitsMetadata},
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version,
            error_unknown_enum_variant, read_body_chunks, reader::BodyReader,
        },
    };

    impl ReadBody for TraitsMetadata {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for TraitsMetadata {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl TraitsMetadata {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(error_unknown_chunk_version(version));
            }

            let num_types = read_packed_u32(r)?;
            let types = r.repeat(num_types as usize, |r| read_type(r))?;
            let num_traits = read_packed_u32(r)?;
            r.repeat(num_traits as usize, |r| {
                let name_len = read_packed_u32(r)?;
                let _name = r.bytes(name_len as usize)?;
                let type_index = read_packed_u32(r)?;
                read_value(r, &types[type_index as usize])?;

                Ok(())
            })?;

            Ok(())
        }
    }

    fn read_packed_u32(r: &mut impl BodyReader) -> Result<u32, Error> {
        let x = r.u8()?;
        let y = if x >= 0x80 { r.u16()? } else { 0 };

        Ok((x & 0x7f) as u32 | (y as u32) << 7)
    }

    fn read_type(r: &mut impl BodyReader) -> Result<ScriptType, Error> {
        match r.u8()? {
            0 => Ok(ScriptType::Void),
            2 => Ok(ScriptType::Integer),
            7 => {
                let key = Box::new(read_type(r)?);
                let value = Box::new(read_type(r)?);

                Ok(ScriptType::Array { key, value })
            }
            value => Err(error_unknown_enum_variant("script type", value as u32)),
        }
    }

    fn read_value(r: &mut impl BodyReader, ty: &ScriptType) -> Result<(), Error> {
        match ty {
            ScriptType::Void => {}
            ScriptType::Integer => {
                r.u32()?;
            }
            ScriptType::Array { key, value } => {
                let len = read_packed_u32(r)?;
                r.repeat(len as usize, |r| {
                    read_value(r, key)?;
                    read_value(r, value)?;

                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}
