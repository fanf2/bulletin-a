use crate::euclid::Euclid;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Gregorian {
    pub y: Euclid,
    pub m: Euclid,
    pub d: Euclid,
}

impl Gregorian {
    pub fn year(self) -> i32 {
        self.y.into()
    }
    pub fn month(self) -> i32 {
        self.m.into()
    }
    pub fn day(self) -> i32 {
        self.d.into()
    }
}

pub fn gregorian(y: i32, m: i32, d: i32) -> Gregorian {
    Gregorian { y: y.into(), m: m.into(), d: d.into() }
}

impl std::fmt::Display for Gregorian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}",
            i32::from(self.y),
            i32::from(self.m),
            i32::from(self.d),
        )
    }
}

impl std::fmt::Debug for Gregorian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gregorian({})", self)
    }
}

fn days_in_years(y: Euclid) -> Euclid {
    y * 1461 / 4 - y / 100 + y / 400
}

impl From<Gregorian> for i32 {
    fn from(g: Gregorian) -> i32 {
        let (y, m, d) = if g.m > 2.into() {
            (g.y, g.m + 1, g.d)
        } else {
            (g.y - 1, g.m + 13, g.d)
        };
        i32::from(days_in_years(y) + m * 153 / 5 + d - 679004)
    }
}

impl From<i32> for Gregorian {
    fn from(mjd: i32) -> Gregorian {
        let mut d = Euclid::from(mjd + 678881);
        let mut y = d * 400 / 146097 + 1;
        if d < days_in_years(y) {
            y -= 1;
        }
        d -= days_in_years(y) - 31;
        let m = d * 17 / 520;
        d -= m * 520 / 17;
        if m > 10.into() {
            Gregorian { y: y + 1, m: m - 10, d }
        } else {
            Gregorian { y: y, m: m + 2, d: d }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        for (g, mjd) in &[
            (gregorian(-1, 12, 31), -678942),
            (gregorian(0, 1, 1), -678941),
            (gregorian(0, 12, 31), -678576),
            (gregorian(1, 1, 1), -678575),
            (gregorian(1858, 11, 16), -1),
            (gregorian(1858, 11, 17), 0),
            (gregorian(1970, 1, 1), 40587),
            (gregorian(2020, 2, 2), 58881),
        ] {
            assert_eq!(*g, Gregorian::from(*mjd));
            assert_eq!(*mjd, i32::from(*g));
        }
    }
}
