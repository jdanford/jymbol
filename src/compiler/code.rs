use crate::Inst;

#[derive(Clone, PartialEq, Debug)]
pub struct Code(Vec<Inst>);

impl Code {
    pub fn new() -> Self {
        Code(Vec::new())
    }

    pub fn pc(&self) -> u32 {
        u32::try_from(self.0.len()).unwrap()
    }

    pub fn emit(&mut self, inst: Inst) -> u32 {
        let pc = self.pc();
        self.0.push(inst);
        pc
    }

    pub fn bookmark(&mut self) -> u32 {
        self.emit(Inst::Nop)
    }

    pub fn patch(&mut self, pc: u32, inst: Inst) {
        self.0[pc as usize] = inst;
    }

    pub fn extract(&mut self) -> Vec<Inst> {
        self.0.drain(..).collect()
    }
}
