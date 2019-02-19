extern crate libc;

pub mod ffi;
pub mod papi_rs;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
