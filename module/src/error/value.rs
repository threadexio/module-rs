use core::fmt::{self, Display, Write};
use core::iter::FusedIterator;

use alloc::boxed::Box;
use alloc::collections::linked_list::{self, LinkedList};

pub struct Value(LinkedList<Box<dyn Display + Send + Sync + 'static>>);

impl Value {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }

    pub fn add<D>(&mut self, component: D)
    where
        D: Display + Send + Sync + 'static,
    {
        self.0.push_front(Box::new(component));
    }

    pub fn components(&self) -> Components<'_> {
        Components(self.0.iter())
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('"')?;

        let mut components = self.components();

        if let Some(first) = components.next() {
            write!(f, "{first}")?;

            components.try_for_each(|x| write!(f, ".{x}"))?;
        }

        f.write_char('"')?;
        Ok(())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut components = self.components();

        if let Some(first) = components.next() {
            write!(f, "{first}")?;

            components.try_for_each(|x| write!(f, ".{x}"))?;
        }

        Ok(())
    }
}

pub struct Components<'a>(linked_list::Iter<'a, Box<dyn Display + Send + Sync + 'static>>);

impl<'a> Iterator for Components<'a> {
    type Item = &'a (dyn Display + Send + Sync + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| &**x)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len()))
    }
}

impl DoubleEndedIterator for Components<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| &**x)
    }
}

impl ExactSizeIterator for Components<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Components<'_> {}
