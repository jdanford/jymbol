use crate::{FnId, Value};

pub struct Compiled {
    pub fn_id: FnId,
    pub locals: Vec<Value>,
    pub pc: u32,
}

pub struct Native {
    pub fn_id: FnId,
    pub locals: Vec<Value>,
}

pub enum Frame {
    Compiled(Compiled),
    Native(Native),
}

impl Frame {
    pub fn compiled(fn_id: FnId, locals: Vec<Value>) -> Self {
        Frame::Compiled(Compiled {
            fn_id,
            locals,
            pc: 0,
        })
    }

    pub fn native(fn_id: FnId, locals: Vec<Value>) -> Self {
        Frame::Native(Native { fn_id, locals })
    }

    pub fn locals(&self) -> &[Value] {
        match self {
            Frame::Compiled(compiled_frame) => &compiled_frame.locals,
            Frame::Native(native_frame) => &native_frame.locals,
        }
    }
    pub fn locals_mut(&mut self) -> &mut [Value] {
        match self {
            Frame::Compiled(compiled_frame) => &mut compiled_frame.locals,
            Frame::Native(native_frame) => &mut native_frame.locals,
        }
    }
}

impl From<Compiled> for Frame {
    fn from(frame: Compiled) -> Self {
        Frame::Compiled(frame)
    }
}

impl From<Native> for Frame {
    fn from(frame: Native) -> Self {
        Frame::Native(frame)
    }
}
