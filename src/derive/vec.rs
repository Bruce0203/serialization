use crate::{
    __private::{
        CompoundUnwrapper, CompoundWrapper, ConstifyPadding, Edge, End, Field, Len, PhantomEdge,
        Size, Sorted,
    },
    Encode, Encoder,
};

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        Ok(())
    }
}

impl<T> Edge for Vec<T>
where
    T: Edge,
{
    type First = End<Self>;

    type Second = PhantomEdge<Vec<T>, (Field<u64>, End<Self>)>;
}

impl<T> Len for Vec<T>
where
    T: Len,
{
    const SIZE: usize = size_of::<Self>();
}

impl<T> Size for Vec<T>
where
    T: Size,
{
    const SIZE: usize = size_of::<Self>();
}

impl<S, T> CompoundWrapper<S> for Vec<T>
where
    T: CompoundWrapper<S> + Edge,
{
    type Compound = <Self as CompoundUnwrapper<S>>::Output;
}
