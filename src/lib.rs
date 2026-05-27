pub fn fac(x: u64) -> u64 {
    (1..=x).into_iter().product()
}

#[cfg(test)]
mod tests {
    use crate::fac;

    #[test]
    fn test_fac_5() {
        assert_eq!(fac(5), 120);
    }
}
