/// Marks type as "any."
///
/// Zero-sized.
pub struct EmAny;

impl EmAny {
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}
