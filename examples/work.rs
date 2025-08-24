use ruff_python_parser::parse_module;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"x = 100
def sus():
    print('yeah i am sus')

print(x)
sus()"#;

    let module = parse_module(source)?;
    let suite = module.into_suite();

    println!("{suite:#?}");

    Ok(())
}
