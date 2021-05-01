#[derive(Copy, Clone, Debug)]
pub struct Roman(pub i32);

impl std::fmt::Display for Roman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // we'll just panic if the number is negative or too large
        let n = self.0 as usize;
        let m = n / 1000;
        let c = (n / 100) % 10;
        let x = (n / 10) % 10;
        let i = n % 10;
        write!(
            f,
            "{}{}{}{}",
            ["", "m", "mm", "mmm"][m],
            ["", "c", "cc", "ccc", "cd", "d", "dc", "dcc", "dccc", "cm"][c],
            ["", "x", "xx", "xxx", "xl", "l", "lx", "lxx", "lxxx", "xc"][x],
            ["", "i", "ii", "iii", "iv", "v", "vi", "vii", "viii", "ix"][i],
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!("mmcdlxviii", format!("{}", super::Roman(2468)));
    }
}
