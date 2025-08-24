use super::IntoEmObject;

#[derive(Debug, Clone)]
pub enum EmInt {
    I64(i64),
    U64(u64),
    F64(f64),
}

impl IntoEmObject for EmInt {}
