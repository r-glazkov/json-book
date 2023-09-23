use boon::{Compiler, Schemas};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

#[test]
fn validate_example() {
    let mut schemas = Schemas::new();
    let mut compiler = Compiler::new();
    let sch_index = compiler.compile("schema.json", &mut schemas).unwrap();
    let file =
        File::open("examples/books/Макаренко Антон - Педагогическая поэма. Полная версия.json")
            .unwrap();
    let reader = BufReader::new(file);
    let instance: Value = serde_json::from_reader(reader).unwrap();
    let valid = schemas.validate(&instance, sch_index).is_ok();
    assert!(valid);
}
