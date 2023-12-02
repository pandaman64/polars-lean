mod sys;

#[no_mangle]
pub extern "C" fn polars_lean_add(left: u32, right: u32) -> u32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = polars_lean_add(2, 2);
        assert_eq!(result, 4);
    }
}
