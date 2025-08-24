use dashmap::DashMap;
use ruff_python_ast::Identifier;

pub struct Scope {
    items: DashMap<Identifier, String>,
}
