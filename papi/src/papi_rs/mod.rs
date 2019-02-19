use crate::ffi::*;
use libc::*;

pub struct EventSet{
    events: Vec<Counter>,
    counters: Vec<libc::c_longlong>
}

impl EventSet {

    pub fn new() -> EventSet {
        let num_counters = unsafe {PAPI_num_counters()};
        let mut counters: Vec<libc::c_longlong> = vec![];
        for _i in 0..num_counters {
            counters.push(0);
        };
        EventSet{ events: vec![], counters }
    }

    pub fn add_event(&mut self, c: Counter){
        self.counters.push(c);
    }

    pub fn is_initialized() -> bool {
        let status = unsafe {
            PAPI_is_initialized()
        };
        if status == PAPI_OK {true} else {false}
    }

    pub fn read_counters() {
    }
}