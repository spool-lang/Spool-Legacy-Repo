use crate::instance::Instance;
use crate::opcode::{Chunk, OpCode};
use std::rc::Rc;
use std::cell::RefCell;

type Mut<T> = Rc<RefCell<T>>;
type MutVec<T> = Vec<Mut<T>>;

pub struct NewVM {
    stack: Vec<Instance>,
    frame_stack: MutVec<NewCallFrame>
}

impl NewVM {
    pub fn new() -> NewVM {
        NewVM {
            stack: vec![],
            frame_stack: vec![]
        }
    }

    pub fn run(&mut self, chunk: Chunk) {
        let frame = NewCallFrame::new(chunk);
        self.execute()
    }

    fn push_call_frame(&mut self, frame: NewCallFrame) {
        &self.frame_stack.push(Rc::new(RefCell::new(frame)));
    }

    fn get_call_frame(&self) -> Mut<NewCallFrame> {
        let index = self.frame_stack.len();
        let option = self.frame_stack.get(index - 1);
        match option {
            None => panic!(),
            Some(frame) => return Rc::clone(frame),
        }
    }

    fn execute(&mut self) {
        loop {
            let chunk = self.get_call_frame().borrow_mut().get_chunk();
            let pc = self.get_call_frame().borrow_mut().pc;
            let option = chunk.get(pc);

            let result = match chunk.get(pc) {
                None => InstructionResult::Return,
                Some(instruction) => self.execute_instruction(instruction),
            };
        }
    }

    fn execute_instruction(&mut self, instruction: &OpCode) -> InstructionResult {
        match instruction {
            OpCode::Get(from_chunk, index) => panic!(),
            OpCode::Declare(val, _) => panic!(),
            OpCode::Set(index) => panic!(),
            _ => panic!("This instruction is unimplemented!")
        }
    }

    fn get(&mut self) {

    }

    fn declare(&mut self) {

    }

    fn set(&mut self) {

    }

    fn push_stack(&mut self) {

    }

    fn pop_stack(&mut self) {

    }
}

struct NewCallFrame {
    chunk: Rc<Chunk>,
    pc: usize,
    stack_top: usize,
    register_top: usize
}

impl NewCallFrame {
    fn new(chunk: Chunk) -> NewCallFrame {
        NewCallFrame {
            chunk: Rc::new(chunk),
            pc: 0,
            stack_top: 0,
            register_top: 0
        }
    }

    fn get_chunk(&self) -> Rc<Chunk> {
        return Rc::clone(&self.chunk)
    }
}

enum InstructionResult {
    Next,
    Return,
    Jump(u16)
}