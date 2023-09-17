use boon::{Compiler, Schemas};

#[test]
fn validate_schema() {
    let mut schemas = Schemas::new();
    let mut compiler = Compiler::new();
    let valid = compiler.compile("schema.json", &mut schemas).is_ok();
    assert!(valid);
}
