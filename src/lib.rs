#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use super::*;

    const scalar_b: [u8; 16] = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128];
    const scalar_a: [u8; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128];

    #[test]
    fn xsb233_add_looks_reasonable() {
        unsafe {
            let mut point_a: xsb233_point = xsb233_neutral.clone();
            let mut point_b: xsb233_point = xsb233_neutral.clone();
            let mut point_ab: xsb233_point = xsb233_neutral.clone();
            let mut point_ba: xsb233_point = xsb233_neutral.clone();

            xsb233_mulgen(&mut point_a, scalar_a.as_ptr() as *const c_void, 16);
            xsb233_mulgen(&mut point_b, scalar_b.as_ptr() as *const c_void, 16);

            xsb233_add(&mut point_ab, &point_a, &point_b);
            xsb233_add(&mut point_ba, &point_b, &point_a);

            assert_eq!(xsb233_equals(&point_a, &point_b), 0);
            assert_eq!(xsb233_equals(&point_a, &point_ba), 0);
            assert_eq!(xsb233_equals(&point_b, &point_ba), 0);
            assert_eq!(xsb233_equals(&point_a, &point_ab), 0);
            assert_eq!(xsb233_equals(&point_b, &point_ab), 0);
            assert_eq!(xsb233_equals(&point_ab, &point_ba), 0xFFFFFFFF);
        }
    }

    #[test]
    fn xsk233_add_looks_reasonable() {
        unsafe {
            let mut point_a: xsk233_point = xsk233_neutral.clone();
            let mut point_b: xsk233_point = xsk233_neutral.clone();
            let mut point_ab: xsk233_point = xsk233_neutral.clone();
            let mut point_ba: xsk233_point = xsk233_neutral.clone();

            xsk233_mulgen(&mut point_a, scalar_a.as_ptr() as *const c_void, 16);
            xsk233_mulgen(&mut point_b, scalar_b.as_ptr() as *const c_void, 16);

            xsk233_add(&mut point_ab, &point_a, &point_b);
            xsk233_add(&mut point_ba, &point_b, &point_a);

            assert_eq!(xsk233_equals(&point_a, &point_b), 0);
            assert_eq!(xsk233_equals(&point_a, &point_ba), 0);
            assert_eq!(xsk233_equals(&point_b, &point_ba), 0);
            assert_eq!(xsk233_equals(&point_a, &point_ab), 0);
            assert_eq!(xsk233_equals(&point_b, &point_ab), 0);
            assert_eq!(xsk233_equals(&point_ab, &point_ba), 0xFFFFFFFF);
        }
    }
}
