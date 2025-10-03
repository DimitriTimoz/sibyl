use num_traits::Float;

pub trait Series 
{
    type Elem: Float;

    fn as_slice(&self) -> &[Self::Elem];
    fn from(vec: Vec<Self::Elem>) -> Self
    where
        Self: Sized;
}
