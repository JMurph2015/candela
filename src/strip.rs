use std::{
    ops::Index,
    ops::IndexMut,
    slice::{Iter, IterMut, SliceIndex},
};

use crate::{types, Pixel};

#[derive(Debug)]
pub struct CandelaStrip {
    id: u32,
    inner: Vec<Pixel>,
}

impl From<CandelaStrip> for types::LedStripState {
    fn from(c: CandelaStrip) -> types::LedStripState {
        let id = c.id;
        let pixels = c
            .inner
            .iter()
            .map(|c| -> u32 { u32::from_le_bytes(*c) })
            .collect::<Vec<u32>>();
        types::LedStripState { id, pixels }
    }
}

impl CandelaStrip {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<I> Index<I> for CandelaStrip
where
    I: SliceIndex<[Pixel]>,
{
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.inner[index]
    }
}

impl<I> IndexMut<I> for CandelaStrip
where
    I: SliceIndex<[Pixel]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<'a> IntoIterator for &'a CandelaStrip {
    type Item = &'a Pixel;
    type IntoIter = Iter<'a, Pixel>;

    fn into_iter(self) -> Iter<'a, Pixel> {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut CandelaStrip {
    type Item = &'a mut Pixel;
    type IntoIter = IterMut<'a, Pixel>;

    fn into_iter(self) -> IterMut<'a, Pixel> {
        self.inner.iter_mut()
    }
}
