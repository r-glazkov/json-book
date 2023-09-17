use std::fs::File;
use std::io::BufReader;

use uuid::Uuid;

use json_book::Book;

#[test]
fn validate_fb2() {
    let fb2_file =
        File::open("examples/books/Макаренко Антон — Педагогическая поэма. Полная версия.fb2").unwrap();
    let fb2_book: fb2::FictionBook = quick_xml::de::from_reader(BufReader::new(fb2_file)).unwrap();
    let book_id = Uuid::new_v4();
    let binary_ids = fb2_book
        .binaries
        .iter()
        .map(|binary| (binary.id.clone(), Uuid::new_v4()))
        .collect();
    let actual = Book::from_fb2(fb2_book, book_id, &binary_ids);

    let json_file = File::open("examples/books/Макаренко Антон - Педагогическая поэма. Полная версия.json").unwrap();
    let mut expected: Book = serde_json::from_reader(BufReader::new(json_file)).unwrap();
    expected.id = actual.id.clone();

    assert_eq!(expected, actual);
}
