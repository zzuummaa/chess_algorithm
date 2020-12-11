#![allow(dead_code)]

use generic_array::{ArrayLength, GenericArray};

pub struct StaticList<E: Sized, L: ArrayLength<E>> {
    pub data: GenericArray<E, L>,

}