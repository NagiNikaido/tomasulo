use super::common_data_bus::*;
use super::instruction::*;
use super::reserve_station::*;

const REG_COUNT: usize = 33;
const RES_COUNT: usize = 12;
const INS_COUNT: usize = 6;
const COM_COUNT: usize = 3;

const upper_bound: [i32; COM_COUNT] = [3, 2, 2];

pub struct Platform {
    clock: usize,
    regs: [i32; REG_COUNT], // we will use f32 as pc.
    cdb: CommonDataBus,
    inst_type:[InstructionType; INS_COUNT],
    inst: Vec<Instruction>,
    stations: [ReserveStation; RES_COUNT],
    running_count: [i32; COM_COUNT]
}


impl Platform {
    pub fn new() -> Self {
        Platform {
            clock: 0,
            regs: [0; REG_COUNT],
            cdb: CommonDataBus::new(),
            inst: Vec::new(),
            inst_type: [
                InstructionType::new(
                    "ADD",
                    3, &[ParamType::REGISTER, ParamType::REGISTER, ParamType::REGISTER],
                    0, |p: &[i32]| -> Result<i32,i32> {Ok(p[1]+p[2])},
                    &[0, 1, 2, 3, 4, 5]
                ),
                InstructionType::new(
                    "SUB",
                    3, &[ParamType::REGISTER, ParamType::REGISTER, ParamType::REGISTER],
                    0, |p: &[i32]| -> Result<i32,i32> {Ok(p[1]-p[2])},
                    &[0, 1, 2, 3, 4, 5]
                ),
                InstructionType::new(
                    "JUMP",
                    1, &[ParamType::INSTANT, ParamType::REGISTER, ParamType::INSTANT, ParamType::REGISTER, ParamType::REGISTER],
                    4, |p: &[i32]| -> Result<i32,i32> {Ok(if p[1]==p[0] {p[3]+p[2]} else {p[3]+1})},
                    &[0, 1, 2, 3, 4, 5]
                ),
                InstructionType::new(
                    "MUL",
                    12, &[ParamType::REGISTER, ParamType::REGISTER, ParamType::REGISTER],
                    0, |p: &[i32]| -> Result<i32,i32> {Ok(p[1]*p[2])},
                    &[6, 7, 8]
                ),
                InstructionType::new(
                    "DIV",
                    40, &[ParamType::REGISTER, ParamType::REGISTER, ParamType::REGISTER],
                    0, |p: &[i32]| -> Result<i32,i32> {if p[2]!=0 {Ok(p[1]/p[2])} else {Err(p[1])}},
                    &[6, 7, 8]
                ),
                InstructionType::new(
                    "LD",
                    3, &[ParamType::REGISTER, ParamType::INSTANT],
                    0, |p: &[i32]| -> Result<i32,i32> {Ok(p[1])},
                    &[9, 10, 11]
                ),
            ],
            stations: [
                ReserveStation::new(0), ReserveStation::new(0), ReserveStation::new(0), ReserveStation::new(0),
                ReserveStation::new(0), ReserveStation::new(0), ReserveStation::new(1), ReserveStation::new(1),
                ReserveStation::new(1), ReserveStation::new(2), ReserveStation::new(2), ReserveStation::new(2),
            ],
            running_count: [0, 0, 0],
        }
    }
    pub fn write_back(&mut self) {
        for (i,station) in self.stations.iter_mut().enumerate() {
            if station.state == StationState::WRITE_BACK {
                let inst = &mut self.inst[station.inst];
                let dest = station.params[inst.inst_type.dest].unwrap() as usize;
                let result = station.result;
                println!("inst #{} write back.", station.inst);
                self.cdb.set_result(i, result);
                match self.cdb.get_busy(dest) {
                    Some(x) => {
                        if x == i {
                        self.regs[dest] = result;
                        self.cdb.clean_busy(dest);
                        }
                    }
                    None => ()
                };
                if inst.write_back_time == -1 {
                    inst.write_back_time = self.clock as i32;
                }
                self.running_count[station.belong] -= 1;
                station.clean();
            }
        }
    }
    pub fn exec(&mut self) {
        let mut ready_list = self.stations.iter().enumerate()
                            .map(|(i,station)|
                                    if station.state == StationState::READY {
                                        Some(i)
                                    } else {
                                        None
                                    })
                            .filter(|t| t.is_some())
                            .map(|t| t.unwrap())
                            .collect::<Vec<usize>>();
        ready_list.sort_by(|a, b| {
                                (self.stations[*a].ready_punch, self.stations[*a].inst).cmp(
                                    &(self.stations[*b].ready_punch, self.stations[*b].inst)
                                )
                            });
        for i in ready_list.into_iter() {
            let station = &mut self.stations[i];
            if self.running_count[station.belong] < upper_bound[station.belong] {
                println!("inst #{} exec.", station.inst);
                self.running_count[station.belong] += 1;
                station.state = StationState::EXEC;
            }
        }
        for (i,station) in self.stations.iter_mut().enumerate() {
            if station.state == StationState::EXEC {
                station.time_left -= 1;
                if station.time_left == 0 {
                    let inst = &mut self.inst[station.inst];
                    if inst.exec_time == -1 {
                        inst.exec_time = self.clock as i32;
                    }
                    station.state = StationState::WRITE_BACK;
                }
            }
        }
    }
    pub fn issue(&mut self) -> bool{
        let pc = self.regs[32] as usize;
        if self.cdb.get_busy(32) != None { // JUMPING!
            return true;
        }
        if pc >= self.inst.len() { // No more insts.
            return false;
        }
        let inst = &mut self.inst[pc];
        for i in inst.inst_type.stations.iter() {
            let station = &mut self.stations[*i];
            if station.state == StationState::IDLE {
                println!("inst #{} issued.", pc);
                station.state = StationState::ISSUE;
                station.inst = pc;
                if inst.issue_time == -1 {
                    inst.issue_time = self.clock as i32;
                }
                let mut params = Vec::<Option<i32>>::new();
                let mut tags = Vec::<Option<usize>>::new();
                for (i,param) in inst.param.iter().enumerate() {
                    if i == inst.inst_type.dest{
                        params.push(Some(*param));
                        tags.push(None);
                    } else {
                        match inst.inst_type.param_type[i] {
                            ParamType::INSTANT => {
                                params.push(Some(*param));
                                tags.push(None);
                            }
                            ParamType::REGISTER => {
                                let t = self.cdb.get_busy((*param) as usize);
                                params.push(if t.is_some() {None} else {Some(self.regs[(*param) as usize])});
                                tags.push(t);
                            }
                        }
                    }
                }
                station.params = params;
                station.tags = tags;
                self.cdb.set_busy(inst.param[inst.inst_type.dest] as usize, *i);
                self.regs[32] += 1;
                break;
            }
        }
        true
    }
    pub fn check_ready(&mut self) -> bool{
        let mut done = true;
        for (i,station) in self.stations.iter_mut().enumerate() {
            if station.state == StationState::ISSUE {
                for (j,tag) in station.tags.iter_mut().enumerate() {
                    if tag.is_some() {
                        let t = self.cdb.get_result(tag.unwrap() as usize);
                        if t.is_some() {
                            *tag = None;
                            station.params[j] = Some(t.unwrap() as i32);
                        }
                    }
                }
                if station.ready() {
                    station.state = StationState::READY;
                    let inst = &mut self.inst[station.inst];
                    println!("inst #{} ready.", station.inst);
                    station.ready_punch = self.clock as i32;
                    let result = (inst.inst_type.func)(
                                           station.params.iter()
                                                         .map(|m| {println!("{:#?}",m);m.unwrap()})
                                                         .collect::<Vec<i32>>()
                                                         .as_slice());
                    println!("{:#?}", result);
                    if result.is_err() {
                        station.result = result.unwrap_err();
                        station.time_left = 1;
                    } else {
                        station.result = result.unwrap();
                        station.time_left = inst.time_left;
                    }
                }
            }
            if station.state != StationState::IDLE {
                done = false;
            }
        }
        done
    }
    pub fn step(&mut self) -> bool{
        println!("Cycle #{}", self.clock);
        self.write_back();
        self.exec();
        self.issue();
        let done = self.check_ready();
        self.cdb.clean_result();
        print!("{}", self);
        self.clock += 1;
        done
    }

    pub fn load_inst(&mut self, line: &String) -> Result<(),()>{
        let mut pieces: Vec<&str> = line.split(",").collect();
        let inst_type = self.inst_type.iter()
                                .filter(|it| {it.name == pieces[0]})
                                .collect::<Vec<&InstructionType>>();
        if inst_type.len() != 1 {
            println!("Not supported inst: {}", pieces[0]);
            return Err(());
        }

        let mut param = pieces[1..].iter()
                                .map(|piece| -> i32{
                                        if piece.chars().nth(0) == Some('F') { // it's a register
                                            u32::from_str_radix(piece.trim_start_matches("F"), 10).unwrap() as i32
                                        } else {
                                            u32::from_str_radix(piece.trim_start_matches("0x"), 16).unwrap() as i32
                                        }
                                    })
                                .collect::<Vec<i32>>();
        if inst_type[0].name == "JUMP" {
            param.push(32);
            param.push(32)
        }
        println!("Inst inserted: {} {:?}", inst_type[0].name, param);
        self.inst.push(Instruction::new(*inst_type[0], param));
        Ok(())
    }
    pub fn print_inst_state(&self) {
        println!("No.     Issue  Exec  WriteBack");
        for (i, inst) in self.inst.iter().enumerate() {
            println!("#{:3}{:^6}  {}  {}  {}", i,
                inst.inst_type.name, inst.issue_time, inst.exec_time, inst.write_back_time);
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Regs: [");
        for (i,reg) in self.regs.iter().enumerate() {
            if i%4 == 0 {
                write!(f, "\n    ");
            }
            write!(f, "F{}: {}, ", i, reg);
        }
        write!(f, "\n]\n");
        write!(f, "Stations: [\n");

        macro_rules! print_station {
            ($t:expr) => ({
                write!(f,"{:^5}", state_to_string(&self.stations[$t].state));
                if self.stations[$t].state != StationState::IDLE {
                    let inst = &self.inst[self.stations[$t].inst];
                    write!(f, " #{:<2}{:^7}", self.stations[$t].inst, inst.inst_type.name);
                    for (i,param) in self.stations[$t].params.iter().enumerate() {
                        if i != inst.inst_type.dest {
                            let tag = & self.stations[$t].tags[i];
                            match tag {
                                Some(x) => {
                                    write!(f, " Station {:2} ", x);
                                }
                                None => {
                                    write!(f, " 0x{:08x} ", param.unwrap());
                                }
                            };
                        }
                    }
                }
                write!(f, "\n");
            })
        }

        fn state_to_string(state : &StationState) -> &'static str {
            match state {
                    StationState::IDLE => "IDLE",
                    StationState::ISSUE => "ISSUE",
                    StationState::READY => "READY",
                    StationState::EXEC => "EXEC",
                    StationState::WRITE_BACK => "WB",
                }
        }

        for i in 0..6 {
            let t = i;
            write!(f, "    Station {:2} / Ars {}: ", t, i);
            print_station!(t);
            
        }
        for i in 0..3 {
            let t = i + 6;
            write!(f, "    Station {:2} / Mrs {}: ", t, i);
            print_station!(t);
        }
        write!(f, "]\n");
        write!(f, "Load Buffers: [\n");
        for i in 0..3 {
            let t = i + 9;
            write!(f, "    Station {:2} / LB  {}: ", t, i);
            print_station!(t);
        }
        write!(f, "]\n");
        write!(f, "\n")
    }

}