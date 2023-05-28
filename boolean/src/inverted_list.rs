mod intersection;
mod union;

use core::slice::Iter;

use intersection::{And, AndNot};

use self::union::Or;

pub type DocId = usize;

#[derive(Debug, PartialEq)]
pub struct Posting {
    pub doc_id: usize,
}

pub struct InvertedList {
    pub list: Vec<Posting>,
}

impl InvertedList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn insert(&mut self, doc_id: DocId) {
        // Each inserted doc_id must be higher than the last.
        let prev = self.list.last().map(|p| p.doc_id).unwrap_or(0);
        assert!(doc_id >= prev);

        self.list.push(Posting { doc_id });
    }

    pub fn iter(&self) -> impl PostingIterator<'_> {
        InvertedListIterator(self.list.iter())
    }
}

pub struct InvertedListIterator<'a>(Iter<'a, Posting>);

impl<'a> Iterator for InvertedListIterator<'a> {
    type Item = &'a Posting;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> PostingIterator<'a> for InvertedListIterator<'a> {}

/// Marker trait for iterators over posting. All types that implement `PostingsIterator` must
/// return `Posting` in ascending order of their document IDs.
pub trait PostingIterator<'a>: Iterator<Item = &'a Posting> {}

pub trait SetMethods<'a>
where
    Self: PostingIterator<'a> + Sized,
{
    fn and<I>(self, other: I) -> And<'a, Self, I>
    where
        I: PostingIterator<'a>,
    {
        And::new(self, other)
    }

    fn and_not<I>(self, other: I) -> AndNot<'a, Self, I>
    where
        I: PostingIterator<'a>,
    {
        AndNot::new(self, other)
    }

    fn or<I>(self, other: I) -> Or<'a, Self, I>
    where
        I: PostingIterator<'a>,
    {
        Or::new(self, other)
    }
}

impl<'a, I> SetMethods<'a> for I where I: PostingIterator<'a> {}
