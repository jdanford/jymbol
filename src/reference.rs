use std::fmt::{self, Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

impl Display for Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "&{}", self.0)
    }
}
