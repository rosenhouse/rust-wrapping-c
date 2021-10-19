extern "C" {
    pub fn my_square(x: u32) -> u32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sys_crate() {
        let result = unsafe { super::my_square(3) };
        assert_eq!(result, 9);
    }
}
