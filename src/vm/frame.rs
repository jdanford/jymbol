use crate::{FnId, Value};

#[allow(clippy::module_name_repetitions)]
pub struct CompiledFrame {
    pub fn_id: FnId,
    pub locals: Vec<Value>,
    pub pc: u32,
}

#[allow(clippy::module_name_repetitions)]
pub struct NativeFrame {
    pub fn_id: FnId,
    pub locals: Vec<Value>,
}

pub enum Frame {
    Compiled(CompiledFrame),
    Native(NativeFrame),
}

impl From<CompiledFrame> for Frame {
    fn from(frame: CompiledFrame) -> Self {
        Frame::Compiled(frame)
    }
}

impl From<NativeFrame> for Frame {
    fn from(frame: NativeFrame) -> Self {
        Frame::Native(frame)
    }
}
