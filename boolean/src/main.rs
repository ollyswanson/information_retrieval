use information_retrieval::{InvertedIndex, Query};

fn main() {
    let document_1 = "new home sales top forecasts".to_owned();
    let document_2 = "home sales rise in july".to_owned();
    let document_3 = "increase in home sales in july".to_owned();
    let document_4 = "july new home sales rise".to_owned();

    let mut inverted_index =
        InvertedIndex::new(|doc| doc.split_whitespace().map(str::to_owned).collect());

    inverted_index.insert_doc(document_1);
    inverted_index.insert_doc(document_2);
    inverted_index.insert_doc(document_3);
    inverted_index.insert_doc(document_4);

    println!("{}", inverted_index);

    let docs = inverted_index.query(Query::Or("rise".to_owned(), "july".to_owned()));
    println!("{:?}", docs);
}
