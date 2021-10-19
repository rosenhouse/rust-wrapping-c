pub fn safe_square(x: u32) -> u32 {
    unsafe { mylib_sys::my_square(x) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rs_crate() {
        assert_eq!(super::safe_square(4), 16);
    }
}
