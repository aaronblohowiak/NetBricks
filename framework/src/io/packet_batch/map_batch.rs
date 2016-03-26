use super::iterator::{BatchIterator, PacketBatchIterator};
use super::act::Act;
use super::Batch;
use super::HeaderOperations;
use super::super::interface::EndOffset;
use super::super::interface::Result;
use super::super::pmd::*;
use std::any::Any;

pub struct MapBatch<T, V>
    where T: EndOffset,
          V: Batch + BatchIterator + Act
{
    parent: V,
    transformer: Box<FnMut(&T, Option<&mut Any>)>,
}

batch!{MapBatch, [parent: V, transformer: Box<FnMut(&T, Option<&mut Any>)>], []}

impl<T, V> Act for MapBatch<T, V>
    where T: EndOffset,
          V: Batch + BatchIterator + Act
{
    #[inline]
    fn act(&mut self) {
        self.parent.act();
        {
            let ref mut f = self.transformer;
            let iter = PacketBatchIterator::<T>::new(&mut self.parent);
            while let Some((packet, ctx)) = iter.next(&mut self.parent) {
                f(packet, ctx);
            }
        }
    }

    #[inline]
    fn done(&mut self) {
        self.parent.done();
    }

    #[inline]
    fn send_queue(&mut self, port: &mut PmdPort, queue: i32) -> Result<u32> {
        self.parent.send_queue(port, queue)
    }

    #[inline]
    fn capacity(&self) -> i32 {
        self.parent.capacity()
    }

    #[inline]
    fn drop_packets(&mut self, idxes: Vec<usize>) -> Option<usize> {
        self.parent.drop_packets(idxes)
    }
}

impl<T, V> BatchIterator for MapBatch<T, V>
    where T: EndOffset,
          V: Batch + BatchIterator + Act
{
    #[inline]
    fn start(&mut self) -> usize {
        self.parent.start()
    }

    #[inline]
    unsafe fn next_address(&mut self, idx: usize) -> Option<(*mut u8, Option<&mut Any>, usize)> {
        self.parent.next_address(idx)
    }

    #[inline]
    unsafe fn next_payload(&mut self, idx: usize) -> Option<(*mut u8, Option<&mut Any>, usize)> {
        self.parent.next_payload(idx)
    }

    #[inline]
    unsafe fn next_base_address(&mut self, idx: usize) -> Option<(*mut u8, Option<&mut Any>, usize)> {
        self.parent.next_base_address(idx)
    }

    #[inline]
    unsafe fn next_base_payload(&mut self, idx: usize) -> Option<(*mut u8, Option<&mut Any>, usize)> {
        self.parent.next_base_payload(idx)
    }
}