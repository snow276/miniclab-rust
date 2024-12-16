use koopa::ir::entities::Function;
use koopa::ir::Program;

pub struct CodegenEnv<'p> {
    program: &'p Program,
    func: Option<Function>,
}

impl<'p, 'f> CodegenEnv<'p> {
    pub fn new(program: &'p Program) -> Self {
        Self { program, func: None }
    }

    pub fn get_program(&self) -> &'p Program {
        self.program
    }

    pub fn get_func(&self) -> Option<&Function> {
        self.func.as_ref()
    }

    pub fn set_func(&mut self, func: Function) {
        self.func = Some(func);
    }
}
