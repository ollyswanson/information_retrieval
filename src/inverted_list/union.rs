use std::cmp::Ordering;
use std::mem;

use super::{Posting, PostingIterator};

pub struct Or<'a, L, R> {
    left: L,
    right: R,
    left_slot: Option<&'a Posting>,
    right_slot: Option<&'a Posting>,
}

impl<'a, L, R> Or<'a, L, R>
where
    L: PostingIterator<'a>,
    R: PostingIterator<'a>,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            left_slot: None,
            right_slot: None,
        }
    }

    #[inline]
    fn fill_left(&mut self) -> Option<&'a Posting> {
        mem::replace(&mut self.left_slot, self.left.next())
    }

    #[inline]
    fn fill_right(&mut self) -> Option<&'a Posting> {
        mem::replace(&mut self.right_slot, self.right.next())
    }
}

impl<'a, L, R> Iterator for Or<'a, L, R>
where
    L: PostingIterator<'a>,
    R: PostingIterator<'a>,
{
    type Item = &'a Posting;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left_slot.is_none() {
            self.fill_left();
        }

        if self.right_slot.is_none() {
            self.fill_right();
        }

        match (self.left_slot, self.right_slot) {
            (Some(left), Some(right)) => match left.doc_id.cmp(&right.doc_id) {
                Ordering::Less => self.fill_left(),
                Ordering::Equal => {
                    self.fill_left();
                    self.fill_right()
                }
                Ordering::Greater => self.fill_right(),
            },
            (Some(_), None) => self.fill_left(),
            (None, Some(_)) => self.fill_right(),
            (None, None) => None,
        }
    }
}

impl<'a, L, R> PostingIterator<'a> for Or<'a, L, R>
where
    L: PostingIterator<'a>,
    R: PostingIterator<'a>,
{
}

#[cfg(test)]
mod tests {
    use crate::inverted_list::{InvertedList, SetMethods};

    use super::*;

    #[test]
    fn or_works() {
        let list_1 = InvertedList {
            list: vec![
                Posting { doc_id: 1 },
                Posting { doc_id: 3 },
                Posting { doc_id: 7 },
                Posting { doc_id: 10 },
                Posting { doc_id: 12 },
            ],
        };

        let list_2 = InvertedList {
            list: vec![
                Posting { doc_id: 2 },
                Posting { doc_id: 3 },
                Posting { doc_id: 4 },
                Posting { doc_id: 11 },
                Posting { doc_id: 12 },
            ],
        };

        let actual = list_1.iter().or(list_2.iter());
        let expected = vec![
            Posting { doc_id: 1 },
            Posting { doc_id: 2 },
            Posting { doc_id: 3 },
            Posting { doc_id: 4 },
            Posting { doc_id: 7 },
            Posting { doc_id: 10 },
            Posting { doc_id: 11 },
            Posting { doc_id: 12 },
        ];

        assert!(actual.eq(expected.iter()));
    }
}
