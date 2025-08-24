mod _any;

mod base;
mod int;

pub use self::_any::EmAny;
pub use self::base::{ EmObject, ReferencedObject, IntoEmObject };

pub use self::int::EmInt;
