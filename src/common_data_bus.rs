use std::collections::HashMap;

pub struct CommonDataBus {
    busy: HashMap<usize, usize>,
    result: HashMap<usize, i32>,
}

impl CommonDataBus {
    pub fn new() -> Self{
        CommonDataBus {
            busy: HashMap::new(),
            result: HashMap::new(),
        }
    }
    pub fn get_busy(&self, reg: usize) -> Option<usize> {
        let t = self.busy.get(&reg);
        match t {
            Some(x) => Some(*x),
            None => None,
        }
    }
    pub fn set_busy(&mut self, reg: usize, station: usize) {
        self.busy.insert(reg, station);
    }
    pub fn clean_busy(&mut self, reg: usize) {
        self.busy.remove(&reg);
    }
    pub fn get_result(&self, inst: usize) -> Option<i32> {
        let t = self.result.get(&inst);
        match t {
            Some(x) => Some(*x),
            None => None,
        }
    }
    pub fn set_result(&mut self, inst: usize, res: i32) {
        self.result.insert(inst, res);
    }
    pub fn clean_result(&mut self) {
        self.result.clear();
    }
}