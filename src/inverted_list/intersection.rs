use std::mem;

use super::{Posting, PostingIterator};

pub struct And<'a, L, R> {
    left: L,
    right: R,
    left_slot: Option<&'a Posting>,
    right_slot: Option<&'a Posting>,
}

impl<'a, L, R> And<'a, L, R>
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
    fn fill_left(&mut self) {
        self.left_slot = self.left.next();
    }

    #[inline]
    fn fill_right(&mut self) {
        self.right_slot = self.right.next();
    }
}

impl<'a, L, R> Iterator for And<'a, L, R>
where
    L: PostingIterator<'a>,
    R: PostingIterator<'a>,
{
    type Item = &'a Posting;

    fn next(&mut self) -> Option<Self::Item> {
        self.fill_left();
        self.fill_right();
        loop {
            match (self.left_slot, self.right_slot) {
                (Some(left), Some(right)) => {
                    if left.doc_id == right.doc_id {
                        return Some(left);
                    }

                    if left.doc_id < right.doc_id {
                        self.fill_left();
                        continue;
                    }

                    if left.doc_id > right.doc_id {
                        self.fill_right();
                        continue;
                    }
                }
                (_, None) => return None,
                (None, _) => return None,
            }
        }
    }
}

impl<'a, L, R> PostingIterator<'a> for And<'a, L, R>
where
    L: PostingIterator<'a>,
    R: PostingIterator<'a>,
{
}

pub struct AndNot<'a, L, R> {
    left: L,
    right: R,
    left_slot: Option<&'a Posting>,
    right_slot: Option<&'a Posting>,
}

impl<'a, L, R> AndNot<'a, L, R>
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

impl<'a, L, R> Iterator for AndNot<'a, L, R>
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

        loop {
            match (self.left_slot, self.right_slot) {
                (Some(left), Some(right)) => {
                    if left.doc_id == right.doc_id {
                        self.fill_left();
                        self.fill_right();
                        continue;
                    }

                    if left.doc_id < right.doc_id {
                        return self.fill_left();
                    }

                    if left.doc_id > right.doc_id {
                        self.fill_right();
                        continue;
                    }
                }
                (Some(_), None) => return self.fill_left(),
                (None, _) => return None,
            }
        }
    }
}

impl<'a, L, R> PostingIterator<'a> for AndNot<'a, L, R>
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
    fn and_single_posting_works() {
        let list_1 = InvertedList {
            list: vec![Posting { doc_id: 1 }],
        };

        let list_2 = InvertedList {
            list: vec![Posting { doc_id: 1 }],
        };

        let actual = list_1.iter().and(list_2.iter());
        let expected = list_1.iter();

        assert!(actual.eq(expected));
    }

    #[test]
    fn and_works() {
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

        let actual = list_1.iter().and(list_2.iter());
        let expected = vec![Posting { doc_id: 3 }, Posting { doc_id: 12 }];

        assert!(actual.eq(expected.iter()));
    }

    #[test]
    fn and_not_works() {
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

        let actual = list_1.iter().and_not(list_2.iter());
        let expected = vec![
            Posting { doc_id: 1 },
            Posting { doc_id: 7 },
            Posting { doc_id: 10 },
        ];

        assert!(actual.eq(expected.iter()));
    }
}
