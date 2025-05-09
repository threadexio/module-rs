use core::fmt::{self, Display};
use core::iter::FusedIterator;

use alloc::boxed::Box;
use alloc::collections::linked_list::{self, LinkedList};

type Item = dyn Display + Send + Sync + 'static;

pub struct Trace(LinkedList<Box<Item>>);

impl Trace {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }

    pub fn add<D>(&mut self, module: D)
    where
        D: Display + Send + Sync + 'static,
    {
        self.0.push_front(Box::new(module));
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }
}

impl fmt::Debug for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DisplayToDebug<T>(T);

        impl<T> fmt::Debug for DisplayToDebug<T>
        where
            T: Display,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        f.debug_list()
            .entries(self.iter().map(DisplayToDebug))
            .finish()
    }
}

pub struct Iter<'a>(linked_list::Iter<'a, Box<Item>>);

impl fmt::Debug for Iter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iter").finish_non_exhaustive()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| &**x)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len()))
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| &**x)
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Iter<'_> {}
