use koopa::ir::entities::Function;
use koopa::ir::BasicBlock;

pub struct IrgenEnv {
    cur_func: Option<Function>,
    cur_bb: Option<BasicBlock>,
}

impl IrgenEnv {
    pub fn new() -> Self {
        Self { cur_func: None, cur_bb: None }
    }

    pub fn get_cur_func(&self) -> Option<&Function> {
        self.cur_func.as_ref()
    }

    pub fn set_cur_func(&mut self, func: Function) {
        self.cur_func = Some(func);
    }

    pub fn get_cur_bb(&self) -> Option<&BasicBlock> {
        self.cur_bb.as_ref()
    }

    pub fn set_cur_bb(&mut self, bb: BasicBlock) {
        self.cur_bb = Some(bb);
    }
}
