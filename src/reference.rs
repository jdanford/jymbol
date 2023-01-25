#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ref(u32);

impl From<u32> for Ref {
    fn from(i: u32) -> Self {
        Ref(i)
    }
}

impl From<Ref> for u32 {
    fn from(ref_: Ref) -> Self {
        ref_.0
    }
}
