use std::marker::PhantomData;

pub struct CollectorList<'a> {
    marker: PhantomData<&'a ()>,
}
