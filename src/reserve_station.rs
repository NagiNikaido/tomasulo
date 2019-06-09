#[derive(PartialEq)]
pub enum StationState {
    IDLE,
    ISSUE,
    READY,
    EXEC,
    WRITE_BACK,
}

pub struct ReserveStation {
    pub params: Vec<Option<i32>>,
    pub tags: Vec<Option<usize>>,
    pub state: StationState,
    pub inst: usize,
    pub result: i32,
    pub time_left: i32,
    pub ready_punch: i32,
    pub belong: usize
}

impl ReserveStation {
    pub fn new(belong: usize) -> Self {
        ReserveStation{
            params: Vec::new(),
            tags: Vec::new(),
            state: StationState::IDLE,
            inst: 0,
            result: 0,
            time_left: 0,
            ready_punch: 0,
            belong: belong
        }
    }
    pub fn ready(&self) -> bool{
        for tag in self.tags.iter() {
            if tag.is_some() {
                return false;
            }
        }
        return true;
    }
    pub fn clean(&mut self) {
        self.state = StationState::IDLE;
        self.inst = 0;
        self.result = 0;
        self.ready_punch = 0;
    }
}
