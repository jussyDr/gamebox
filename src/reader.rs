use std::{
    borrow::BorrowMut,
    io::{self, Read, Seek, Take},
    iter,
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Self::Io(io_error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct IdState {
    seen_id: bool,
    ids: Vec<String>,
}

impl IdState {
    pub fn new() -> Self {
        Self {
            seen_id: false,
            ids: vec![],
        }
    }
}

pub trait IdStateMut: BorrowMut<IdState> {}

impl<T: BorrowMut<IdState>> IdStateMut for T {}

pub struct NodeState {
    num_nodes: u32,
}

impl NodeState {
    pub fn new(num_nodes: u32) -> Self {
        Self { num_nodes }
    }
}

pub trait NodeStateMut: BorrowMut<NodeState> {}

impl<T: BorrowMut<NodeState>> NodeStateMut for T {}

pub struct Reader<R, I, N> {
    inner: R,
    id_state: I,
    node_state: N,
}

impl<R, I, N> Reader<R, I, N> {
    pub fn new(inner: R, id_state: I, node_state: N) -> Self {
        Self {
            inner,
            id_state,
            node_state,
        }
    }
}

impl<R: Read, I, N> Reader<R, I, N> {
    pub fn u8(&mut self) -> Result<u8> {
        let bytes = self.byte_array()?;
        Ok(u8::from_le_bytes(bytes))
    }

    pub fn u16(&mut self) -> Result<u16> {
        let bytes = self.byte_array()?;
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn u32(&mut self) -> Result<u32> {
        let bytes = self.byte_array()?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn f32(&mut self) -> Result<f32> {
        let bytes = self.byte_array()?;
        Ok(f32::from_le_bytes(bytes))
    }

    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; n];
        self.inner.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn byte_array<const L: usize>(&mut self) -> Result<[u8; L]> {
        let mut array = [0; L];
        self.inner.read_exact(&mut array)?;
        Ok(array)
    }

    pub fn string(&mut self) -> Result<String> {
        let len = self.u32()?;
        let bytes = self.bytes(len as usize)?;
        let string = String::from_utf8(bytes).unwrap();
        Ok(string)
    }

    pub fn list<T>(&mut self, read_fn: impl Fn(&mut Self) -> Result<T>) -> Result<Vec<T>> {
        let len = self.u32()?;
        self.repeat(len as usize, read_fn)
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        read_fn: impl Fn(&mut Self) -> Result<T>,
    ) -> Result<Vec<T>> {
        iter::repeat_with(|| read_fn(self)).take(n).collect()
    }

    pub fn take<IS, NS>(
        &mut self,
        limit: u64,
        id_state: IS,
        node_state: NS,
    ) -> Reader<Take<&mut R>, IS, NS> {
        let inner = (&mut self.inner).take(limit);
        Reader::new(inner, id_state, node_state)
    }

    pub fn end(&mut self) -> Result<()> {
        let mut buf = [0];

        match self.inner.read(&mut buf) {
            Ok(0) => Ok(()),
            _ => todo!(),
        }
    }
}

impl<R: Read + Seek, I, N> Reader<R, I, N> {
    pub fn peek_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let bytes = self.bytes(n)?;
        self.inner.seek(io::SeekFrom::Current(-(n as i64)))?;
        Ok(bytes)
    }
}

impl<R: Read, I: IdStateMut, N> Reader<R, I, N> {
    pub fn id(&mut self) -> Result<String> {
        match self.id_or_null()? {
            None => todo!(),
            Some(id) => Ok(id),
        }
    }

    pub fn id_or_null(&mut self) -> Result<Option<String>> {
        if !self.id_state.borrow().seen_id {
            if self.u32()? != 3 {
                todo!()
            }

            self.id_state.borrow_mut().seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index == 0x40000000 {
            let id = self.string()?;
            self.id_state.borrow_mut().ids.push(id.clone());
            return Ok(Some(id));
        }

        todo!()
    }
}

impl<R: Read, I, N: NodeStateMut> Reader<R, I, N> {
    pub fn node(
        &mut self,
        class_id: u32,
        read_fn: impl FnOnce(&mut Self) -> Result<()>,
    ) -> Result<()> {
        let index = self.u32()?;

        if index == 0 || index > self.node_state.borrow().num_nodes {
            todo!()
        }

        if self.u32()? != class_id {
            todo!()
        }

        read_fn(self)?;

        Ok(())
    }
}
