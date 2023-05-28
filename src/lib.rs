mod inverted_list;

use std::collections::BTreeMap;
use std::fmt::Display;

use inverted_list::{DocId, InvertedList, SetMethods};

type Doc = String;
type Term = String;

pub struct InvertedIndex<T> {
    docs: Vec<Doc>,
    term_map: BTreeMap<Term, usize>,
    index: Vec<InvertedList>,
    tokenizer: T,
}

pub enum Query {
    Or(String, String),
}

impl<T> Display for InvertedIndex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (term, &pos) in self.term_map.iter() {
            write!(f, "{term}: ")?;
            let mut posting_list = self.index[pos].list.iter();

            if let Some(posting) = posting_list.next() {
                write!(f, "{}", posting.doc_id + 1)?;
            } else {
                return Ok(());
            }

            for posting in posting_list {
                write!(f, ", {}", posting.doc_id + 1)?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl<T> InvertedIndex<T>
where
    T: Fn(&Doc) -> Vec<Term>,
{
    pub fn new(tokenizer: T) -> Self {
        tokenizer(&String::new());
        Self {
            docs: Vec::new(),
            term_map: BTreeMap::new(),
            index: Vec::new(),
            tokenizer,
        }
    }

    pub fn insert_doc(&mut self, doc: Doc) {
        let tokenizer = &self.tokenizer;
        let terms = tokenizer(&doc);

        let doc_id = self.docs.len();
        self.docs.push(doc);

        for term in terms {
            let len = self.term_map.len();
            let entry = self.term_map.entry(term).or_insert(len);
            let term_num = *entry;

            if term_num == len {
                self.index.push(InvertedList::new());
            }
            self.index[term_num].insert(doc_id);
        }
    }

    #[inline]
    fn term_pos(&self, term: &Term) -> Option<usize> {
        self.term_map.get(term).copied()
    }

    pub fn query(&self, query: Query) -> Vec<DocId> {
        match query {
            Query::Or(left, right) => {
                let left = self.term_pos(&left).unwrap();
                let right = self.term_pos(&right).unwrap();

                self.index[left]
                    .iter()
                    .and(self.index[right].iter())
                    .map(|posting| posting.doc_id + 1)
                    .collect()
            }
        }
    }
}
