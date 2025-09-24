use crate::Arity;

use super::{code::Code, locals::Locals};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LoopContext {
    pub frame_offset: u16,
    pub locals_offset: u16,
    pub loop_body_pc: u32,
    pub loop_arity: Arity,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Context {
    pub locals: Locals,
    pub code: Code,
    #[allow(clippy::struct_field_names)]
    pub loop_context: Option<LoopContext>,
}

impl Context {
    pub fn blank() -> Self {
        Self {
            locals: Locals::new(),
            code: Code::new(),
            loop_context: None,
        }
    }

    pub fn fn_(&self) -> Self {
        Self {
            locals: Locals::new(),
            code: Code::new(),
            loop_context: self.loop_context.as_ref().map(|loop_context| LoopContext {
                frame_offset: loop_context.frame_offset + 1,
                ..*loop_context
            }),
        }
    }

    pub fn in_loop(&self) -> bool {
        self.loop_context.is_some()
    }
}
