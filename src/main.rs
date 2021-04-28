mod euclid;

use euclid::Euclid;
use std::fmt;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Gregorian {
    y: Euclid,
    m: Euclid,
    d: Euclid,
}

fn gregorian(y: i32, m: i32, d: i32) -> Gregorian {
    Gregorian { y: y.into(), m: m.into(), d: d.into() }
}

impl fmt::Debug for Gregorian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gregorian({:04}-{:02}-{:02})",
            i32::from(self.y),
            i32::from(self.m),
            i32::from(self.d)
        )
    }
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct MJD(Euclid);

impl fmt::Debug for MJD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MJD({})", i32::from(self.0))
    }
}

fn days_in_years(y: Euclid) -> Euclid {
    y * 1461 / 4 - y / 100 + y / 400
}

impl From<i32> for MJD {
    fn from(i: i32) -> MJD {
        MJD(i.into())
    }
}

impl From<Gregorian> for MJD {
    fn from(g: Gregorian) -> MJD {
        let (y, m, d) = if g.m > 2.into() {
            (g.y, g.m + 1, g.d)
        } else {
            (g.y - 1, g.m + 13, g.d)
        };
        MJD(days_in_years(y) + m * 153 / 5 + d - 679004)
    }
}

impl From<MJD> for Gregorian {
    fn from(MJD(mut d): MJD) -> Gregorian {
        d += 678881;
        let mut y = d * 400 / 146097 + 1;
        if d < days_in_years(y) {
            y -= 1
        }
        d -= days_in_years(y) - 31;
        let mut m = d * 17 / 520;
        d -= m * 520 / 17;
        if m < 11.into() {
            m += 2;
        } else {
            m -= 10;
            y += 1;
        }
        Gregorian { y, m, d }
    }
}

fn main() {}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        for (g, mjd) in &[
            (gregorian(-1, 12, 31), MJD::from(-678942)),
            (gregorian(0, 1, 1), MJD::from(-678941)),
            (gregorian(0, 12, 31), MJD::from(-678576)),
            (gregorian(1, 1, 1), MJD::from(-678575)),
            (gregorian(1858, 11, 16), MJD::from(-1)),
            (gregorian(1858, 11, 17), MJD::from(0)),
            (gregorian(1970, 1, 1), MJD::from(40587)),
            (gregorian(2020, 2, 2), MJD::from(58881)),
        ] {
            assert_eq!(*mjd, MJD::from(*g));
            assert_eq!(*g, Gregorian::from(*mjd));
        }
    }
}
