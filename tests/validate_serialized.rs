use boon::{Compiler, Schemas};
use json_book::Book;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

#[test]
fn validate_serialized() {
    let file =
        File::open("examples/books/Макаренко Антон - Педагогическая поэма. Полная версия.json")
            .unwrap();
    let reader = BufReader::new(file);
    let book: Book = serde_json::from_reader(reader).unwrap();
    let serialized = serde_json::to_string(&book).unwrap();

    let mut schemas = Schemas::new();
    let mut compiler = Compiler::new();
    let sch_index = compiler.compile("schema.json", &mut schemas).unwrap();
    let instance: Value = serde_json::from_str(&serialized).unwrap();
    let valid = schemas.validate(&instance, sch_index).is_ok();
    assert!(valid);
}
