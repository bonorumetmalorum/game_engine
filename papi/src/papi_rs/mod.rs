use crate::ffi::*;
use libc::*;

pub struct EventSet{
    events: Vec<Counter>,
    counters: Vec<libc::c_longlong>
}

impl EventSet {

    pub fn new() -> EventSet {
        let num_counters = unsafe {PAPI_num_counters()};
        
    }

    pub fn is_initialized() -> bool {
        let status = unsafe {
            PAPI_is_initialized()
        };
        if status == PAPI_OK {true} else {false}
    }

    pub fn read_counters()
}