//! Reader

mod id;
mod node;

pub use id::IdTable;
pub use node::{Downcast, NodeTable};

use zerocopy::{FromBytes, FromZeros, IntoBytes};

use std::{any::Any, io::Read, iter, sync::Arc};

use crate::{
    Box3D, ClassId, ExternalNodeRef, Iso4, NodeRef, Quat, SubExtensions, UVec3, Vec2, Vec3,
    read::{Error, ReadBody, byte_order::LeToNe, error_unknown_version, map_io_error},
    sub_extension,
};

fn repeat_n_with<T, U: FromIterator<T>>(n: usize, repeater: impl FnMut() -> T) -> U {
    iter::repeat_with(repeater).take(n).collect()
}

/// Reader.
pub trait Reader: Read {
    /// Read `n` bytes.
    fn bytes(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; n];
        self.read_exact(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    /// Read `N` bytes into an array.
    fn byte_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    /// Read a byte buffer.
    fn byte_buf(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.u32()?;
        Reader::bytes(self, size as usize)
    }

    /// Read a value using zerocopy.
    fn zerocopy<T: FromBytes + LeToNe>(&mut self) -> Result<T, Error> {
        let mut value = T::read_from_io(self).map_err(map_io_error)?;

        // GameBox files are serialized as little endian,
        // so it is necessary to convert to the target's endianness.
        value.le_to_ne();

        Ok(value)
    }

    /// Read an unsigned 8-bit integer.
    fn u8(&mut self) -> Result<u8, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 16-bit integer.
    fn u16(&mut self) -> Result<u16, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 32-bit integer.
    fn u32(&mut self) -> Result<u32, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 64-bit integer.
    fn u64(&mut self) -> Result<u64, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 128-bit integer.
    fn u128(&mut self) -> Result<u128, Error> {
        self.zerocopy()
    }

    /// Read a 32-bit floating point number.
    fn f32(&mut self) -> Result<f32, Error> {
        self.zerocopy()
    }

    /// Read an 8-bit boolean.
    fn bool8(&mut self) -> Result<bool, Error> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new("expected an 8-bit boolean")),
        }
    }

    /// Read a 32-bit boolean.
    fn bool32(&mut self) -> Result<bool, Error> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new("expected a 32-bit boolean")),
        }
    }

    /// Read a `Vec2`.
    fn vec2(&mut self) -> Result<Vec2, Error> {
        self.zerocopy()
    }

    /// Read a `Vec3`.
    fn vec3(&mut self) -> Result<Vec3, Error> {
        self.zerocopy()
    }

    /// Read a `UVec3`.
    fn uvec3(&mut self) -> Result<UVec3, Error> {
        self.zerocopy()
    }

    /// Read a `Quat`.
    fn quat(&mut self) -> Result<Quat, Error> {
        self.zerocopy()
    }

    /// Read a box.
    fn box3d(&mut self) -> Result<Box3D, Error> {
        self.zerocopy()
    }

    /// Read an `Iso4`.
    fn iso4(&mut self) -> Result<Iso4, Error> {
        self.zerocopy()
    }

    /// Read a string.
    fn string(&mut self) -> Result<String, Error> {
        let bytes = self.byte_buf()?;

        String::from_utf8(bytes).map_err(|_| Error::new("expected an UTF-8 encoded string"))
    }

    /// Read a list of elements.
    fn list<T>(
        &mut self,
        read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        self.repeat(len as usize, read_fn)
    }

    /// Read a list of elements using zerocopy.
    fn list_zerocopy<T: FromZeros + FromBytes + IntoBytes + LeToNe>(
        &mut self,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;
        self.repeat_zerocopy(len as usize)
    }

    /// Read a list of elements.
    fn list_with_version<T>(
        &mut self,
        read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let version = self.u32()?;

        if version != 10 {
            return Err(error_unknown_version("list", version));
        }

        self.list(read_fn)
    }

    /// Repeat the given `read_fn` a total of `n` times.
    fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        repeat_n_with(n, || read_fn(self))
    }

    /// Repeat the given `read_fn` a total of `n` times using zerocopy.
    fn repeat_zerocopy<T: FromZeros + FromBytes + IntoBytes + LeToNe>(
        &mut self,
        n: usize,
    ) -> Result<Vec<T>, Error> {
        let mut list = T::new_vec_zeroed(n).expect("memory allocation failed");
        let bytes = list.as_mut_slice().as_mut_bytes();
        self.read_exact(bytes).map_err(map_io_error)?;

        // GameBox files are serialized as little endian,
        // so it is necessary to convert to the target's endianness.
        list.le_to_ne();

        Ok(list)
    }

    /// Return an error if the reader is not at EOF.
    fn expect_eof(&mut self) -> Result<(), Error> {
        let mut buf = [0];
        let n = self.read(&mut buf).map_err(map_io_error)?;

        if n != 0 {
            return Err(Error::new("expected EOF"));
        }

        Ok(())
    }
}

impl<T: Read> Reader for T {}

/// Try from id.
pub trait TryFromId {
    /// Try from id.
    fn try_from_id(id: Option<Arc<str>>) -> Result<Self, Error>
    where
        Self: Sized;
}

/// Header reader.
pub trait HeaderReader: Reader {
    /// Id table.
    fn id_table(&mut self) -> &mut IdTable;

    /// WIP.
    fn id_v2<T: TryFromId>(&mut self) -> Result<T, Error> {
        let id = self.id_or_null()?;

        T::try_from_id(id)
    }

    /// Read an identifier.
    fn id(&mut self) -> Result<Arc<str>, Error> {
        match self.id_or_null()? {
            None => Err(Error::new("expected a non-null identifier")),
            Some(id) => Ok(id),
        }
    }

    /// Read an identifier.
    fn id_or_null(&mut self) -> Result<Option<Arc<str>>, Error> {
        if !self.id_table().seen_id {
            let version = self.u32()?;

            if version != 3 {
                return Err(error_unknown_version("identifier", version));
            }

            self.id_table().seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index == 0x0000001a {
            // Not sure what this is yet.
            return Ok(Some(Arc::from("")));
        }

        if index & 0x40000000 == 0 {
            return Err(Error::new("expected an identifier"));
        }

        let index = index & 0x37ffffff;

        match index.checked_sub(1) {
            None => {
                let id = Arc::from(self.string()?);
                self.id_table().ids.push(Arc::clone(&id));

                Ok(Some(id))
            }
            Some(index) => {
                let id = self
                    .id_table()
                    .ids
                    .get(index as usize)
                    .ok_or_else(|| Error::new(""))?;

                Ok(Some(Arc::clone(id)))
            }
        }
    }
}

/// HR.
pub struct HR<R, I> {
    /// Reader.
    pub reader: R,
    /// Id table.
    pub id_table: I,
}

impl<R: Read, I> Read for HR<R, I> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read, I: AsMut<IdTable>> HeaderReader for HR<R, I> {
    fn id_table(&mut self) -> &mut IdTable {
        self.id_table.as_mut()
    }
}

/// Try from node ref.
pub trait TryFromNodeRef {
    /// Try from node ref.
    fn try_from_node_ref(node_ref: Option<NodeRef<()>>) -> Result<Self, Error>
    where
        Self: Sized;
}

/// Body reader.
pub trait BodyReader: HeaderReader {
    /// Node table.
    fn node_table(&mut self) -> &mut NodeTable;

    /// WIP.
    fn node_ref_v2<T: TryFromNodeRef>(&mut self) -> Result<T, Error> {
        todo!()
    }

    /// Read a node.
    fn node<T: Default + ClassId + ReadBody>(&mut self) -> Result<T, Error>
    where
        Self: Sized,
    {
        let node = self.node_generic(|r, class_id| {
            if class_id != T::CLASS_ID {
                todo!("{:08X?}", class_id);
            }

            let mut node = T::default();
            node.read_body(r)?;

            Ok(node)
        })?;

        Ok(node)
    }

    /// Read a reference to a node.
    fn node_ref<T: Default + Send + Sync + ClassId + ReadBody + 'static>(
        &mut self,
    ) -> Result<NodeRef<Arc<T>>, Error>
    where
        Self: Sized,
    {
        let node_ref = self.node_ref_or_null::<T>()?;

        match node_ref {
            None => Err(Error::new("node reference is null")),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read a reference to a node.
    fn node_ref_or_null<T: Default + Send + Sync + ClassId + ReadBody + 'static>(
        &mut self,
    ) -> Result<Option<NodeRef<Arc<T>>>, Error>
    where
        Self: Sized,
    {
        let node_ref = self.node_ref_generic_or_null(|r, class_id| {
            if class_id != T::CLASS_ID {
                todo!()
            }

            let mut node = T::default();
            node.read_body(r)?;

            Ok(Arc::new(node))
        })?;

        match node_ref {
            None => Ok(None),
            Some(node_ref) => Ok(Some(node_ref)),
        }
    }

    /// Read a reference to an internal node.
    fn internal_node_ref<T: Default + Send + Sync + ClassId + ReadBody + 'static>(
        &mut self,
    ) -> Result<Arc<T>, Error>
    where
        Self: Sized,
    {
        let node = self.internal_node_ref_or_null::<T>()?;

        match node {
            None => Err(Error::new("node reference is null")),
            Some(node) => Ok(node),
        }
    }

    /// Read a reference to an internal node.
    fn internal_node_ref_or_null<T: Default + Send + Sync + ClassId + ReadBody + 'static>(
        &mut self,
    ) -> Result<Option<Arc<T>>, Error>
    where
        Self: Sized,
    {
        let node: Option<Arc<T>> = self.internal_node_ref_generic_or_null(|r, class_id| {
            if class_id != T::CLASS_ID {
                todo!()
            }

            let mut node = T::default();

            node.read_body(r)?;

            Ok(Arc::new(node))
        })?;

        match node {
            None => Ok(None),
            Some(node) => {
                let ptr = Arc::into_raw(node);
                unsafe { Ok(Some(Arc::from_raw(ptr.cast()))) }
            }
        }
    }

    /// Read a reference to an external node.
    fn external_node_ref<T: SubExtensions>(&mut self) -> Result<ExternalNodeRef, Error> {
        match self.external_node_ref_or_null::<T>()? {
            None => todo!(),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read a reference to an external node.
    fn external_node_ref_or_null<T: SubExtensions>(
        &mut self,
    ) -> Result<Option<ExternalNodeRef>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or_else(|| Error::new("node index is zero"))?;

        let slot = self
            .node_table()
            .nodes
            .get(index as usize)
            .ok_or_else(|| Error::new("node index exceeds number of nodes"))?;

        match slot {
            Some(NodeRef::External(node_ref)) => {
                let sub_extension = sub_extension(&node_ref.path).unwrap();

                if !T::has_sub_extension(sub_extension) {
                    todo!("{}", sub_extension);
                }

                Ok(Some(node_ref.clone()))
            }
            _ => todo!(),
        }
    }

    /// Read a node.
    fn node_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let node = self.node_or_null_generic(read_fn)?;

        match node {
            None => Err(Error::new("node is null")),
            Some(node) => Ok(node),
        }
    }

    /// Read a node.
    fn node_or_null_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<Option<T>, Error> {
        let class_id = self.u32()?;

        if class_id == 0xffffffff {
            return Ok(None);
        }

        let node = read_fn(self, class_id)?;

        Ok(Some(node))
    }

    /// Read a reference to a node.
    fn node_ref_generic<T: Clone + Downcast>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Any + Send + Sync>, Error>,
    ) -> Result<NodeRef<T>, Error> {
        let node_ref = self.node_ref_generic_or_null(read_fn)?;

        match node_ref {
            None => Err(Error::new("node reference is null")),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read a reference to a node.
    fn node_ref_generic_or_null<T: Clone + Downcast>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Any + Send + Sync>, Error>,
    ) -> Result<Option<NodeRef<T>>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or_else(|| Error::new("node index is zero"))?;

        let slot = self
            .node_table()
            .nodes
            .get_mut(index as usize)
            .ok_or_else(|| Error::new("node index exceeds number of nodes"))?;

        match slot {
            None => {
                let node = self.node_generic(read_fn)?;

                let slot = self.node_table().nodes.get_mut(index as usize).unwrap();

                *slot = Some(NodeRef::Internal(Arc::clone(&node)));

                Ok(Some(NodeRef::Internal(T::downcast(node).unwrap())))
            }
            Some(node_ref) => match node_ref {
                NodeRef::Internal(x) => {
                    Ok(Some(NodeRef::Internal(T::downcast(x.clone()).unwrap())))
                }
                NodeRef::External(x) => Ok(Some(NodeRef::External(x.clone()))),
            },
        }
    }

    /// Read a reference to an internal node.
    fn internal_node_ref_generic<T: Clone + Downcast>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Any + Send + Sync>, Error>,
    ) -> Result<T, Error> {
        let node_ref = self.node_ref_generic(read_fn)?;

        match node_ref {
            NodeRef::Internal(node) => Ok(node),
            NodeRef::External(_) => Err(Error::new("expected an internal node reference")),
        }
    }

    /// Read a reference to an internal node.
    fn internal_node_ref_generic_or_null<T: Clone + Downcast>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Any + Send + Sync>, Error>,
    ) -> Result<Option<T>, Error> {
        let node_ref = self.node_ref_generic_or_null(read_fn)?;

        match node_ref {
            None => Ok(None),
            Some(NodeRef::Internal(node)) => Ok(Some(node)),
            Some(NodeRef::External(_)) => Err(Error::new("expected an internal node reference")),
        }
    }
}

/// BR.
pub struct BR<R, I, N> {
    /// Reader.
    pub reader: R,
    /// Id table.
    pub id_table: I,
    /// Node table.
    pub node_table: N,
}

impl<R: Read, I, N> Read for BR<R, I, N> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read, I: AsMut<IdTable>, N> HeaderReader for BR<R, I, N> {
    fn id_table(&mut self) -> &mut IdTable {
        self.id_table.as_mut()
    }
}

impl<R: Read, I: AsMut<IdTable>, N: AsMut<NodeTable>> BodyReader for BR<R, I, N> {
    fn node_table(&mut self) -> &mut NodeTable {
        self.node_table.as_mut()
    }
}
