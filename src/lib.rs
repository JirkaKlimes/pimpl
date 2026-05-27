pub fn fac(x: u64) -> u64 {
    if x <= 1 {
        return x;
    }
    x * papul::fac(x - 1)
}

#[cfg(test)]
mod tests {
    use crate::fac;

    #[test]
    fn test_fac_5() {
        assert_eq!(fac(5), 120);
    }
}
