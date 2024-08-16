#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            let result = LJX8IF_Initialize();
            assert_eq!(result, 0);
            let result = LJX8IF_Finalize();
            assert_eq!(result, 0);
        }
    }
}