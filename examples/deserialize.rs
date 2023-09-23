use json_book::Book;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file =
        File::open("examples/books/Макаренко Антон - Педагогическая поэма. Полная версия.json")
            .unwrap();
    let reader = BufReader::new(file);
    let value: Book = serde_json::from_reader(reader).unwrap();
    println!("{}", value.short_title);
}
