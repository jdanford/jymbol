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

impl Frame {
    pub fn locals(&mut self) -> &mut [Value] {
        match self {
            Frame::Compiled(compiled_frame) => &mut compiled_frame.locals,
            Frame::Native(native_frame) => &mut native_frame.locals,
        }
    }
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
