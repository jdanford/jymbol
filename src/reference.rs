#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ref(u32);

impl Ref {
    #[must_use]
    pub const fn from_u32(i: u32) -> Self {
        Ref(i)
    }

    #[must_use]
    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

impl From<u32> for Ref {
    fn from(i: u32) -> Self {
        Ref::from_u32(i)
    }
}

impl From<Ref> for u32 {
    fn from(ref_: Ref) -> Self {
        ref_.into_u32()
    }
}
