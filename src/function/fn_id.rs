use std::{num::NonZeroU32, sync::{LazyLock, Mutex}};

use gc::{unsafe_empty_trace, Finalize, Trace};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Finalize)]
pub struct FnId(NonZeroU32);

unsafe impl Trace for FnId {
    unsafe_empty_trace!();
}

impl From<NonZeroU32> for FnId {
    fn from(i: NonZeroU32) -> Self {
        FnId(i)
    }
}

impl From<FnId> for NonZeroU32 {
    fn from(sym: FnId) -> Self {
        sym.0
    }
}

impl From<FnId> for u32 {
    fn from(sym: FnId) -> Self {
        sym.0.into()
    }
}

static NEXT_ID: LazyLock<Mutex<FnId>> = LazyLock::new(|| {
    let id = NonZeroU32::new(1).unwrap();
    Mutex::new(FnId::from(id))
});

impl FnId {
    #[allow(clippy::missing_panics_doc)]
    pub fn next() -> Self {
        let mut next_id = NEXT_ID.lock().unwrap();
        let id = next_id.0;
        next_id.0 = NonZeroU32::new(u32::from(id) + 1).unwrap();
        id.into()
    }
}
