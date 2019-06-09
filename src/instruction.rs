pub type InstFunc = fn(&[i32]) -> Result<i32, i32>;

#[derive(PartialEq)]
pub enum ParamType {
    INSTANT,
    REGISTER
}

pub struct InstructionType {
    pub name: &'static str,
    pub cycles: i32,
    pub param_type: &'static [ParamType],
    pub dest: usize,
    pub func: InstFunc,
    pub stations: &'static [usize]
}

impl Clone for InstructionType {
    fn clone(&self) -> Self{
        InstructionType {
            name: self.name,
            cycles: self.cycles,
            param_type: self.param_type,
            dest: self.dest,
            func: self.func,
            stations: self.stations,
        }
    }
}
impl Copy for InstructionType {}

impl InstructionType {
    pub fn new(name: &'static str,
               cycles: i32,
               param_type: &'static [ParamType],
               dest: usize,
               func: InstFunc,
               stations: &'static [usize]) -> Self{
        InstructionType {
            name: name,
            cycles: cycles,
            param_type: param_type,
            dest: dest,
            func: func,
            stations: stations
        }
    }
    
}

pub struct Instruction {
    pub inst_type: InstructionType,
    pub param: Vec<i32>,
    pub issue_time: i32,
    pub exec_time: i32,
    pub write_back_time: i32,
    pub time_left: i32
}

impl Instruction {
    pub fn new(inst_type: InstructionType, param: Vec<i32>) -> Self {
        Instruction {
            inst_type: inst_type,
            param: param,
            issue_time: -1,
            exec_time: -1,
            write_back_time: -1,
            time_left: inst_type.cycles,
        }
    }
}