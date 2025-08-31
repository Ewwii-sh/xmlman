use rhai::Engine;
use std::error::Error;

pub fn format_rhai_code(code: &str) -> Result<String, Box<dyn Error>> {
    let engine = Engine::new();

    let ast = engine.compile(&code)?;

    for stmt in ast.statements() {
        println!("STATEMENT: {:#?}", stmt);
    }

    Ok(String::new())
}
