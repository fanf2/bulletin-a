mod date;
mod roman;

use anyhow::{anyhow, Context, Result};
use date::*;
use roman::Roman;
use std::io::Read;
use std::str::FromStr;

fn usage() {
    eprint!(
            "usage: bulletin-a [N]\n\
             display leap second forecast from the last N issues of Bulletin A\n\
             default is to print the latest forecast\n"
        );
    std::process::exit(1);
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if (args.len() > 1 && (args[1] == "-h" || args[1] == "--help"))
        || args.len() > 3
    {
        usage();
    }
    let thresh = if args.len() > 2 { f64::from_str(&args[2])? } else { 0.60 };
    let count = if args.len() > 1 { i32::from_str(&args[1])? } else { 1 };
    let latest = latest_bulletin_a()?;
    let first = latest - count + 1;
    for issue in first..=latest {
        let param = bulletin_a(issue)?;
        let leap = param.leap(thresh);
        eprintln!("{} -> {}", param.date, leap);
    }
    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Prediction {
    dut1: f64,
    lod: f64,
    mjd: i32,
}

// https://gssc.esa.int/navipedia/index.php/Transformations_between_Time_Systems

impl Prediction {
    fn at(self, mjd: i32) -> f64 {
        let besselian_year = 2000.0 + (mjd as f64 - 51544.03) / 365.2422;
        let t = besselian_year * std::f64::consts::TAU;
        let tt = 2.0 * t;
        let s = 0.022 * t.sin() - 0.012 * t.cos();
        let ss = 0.006 * tt.sin() - 0.007 * tt.cos();
        let ut2_ut1 = s - ss;
        let days = (mjd - self.mjd) as f64;
        self.dut1 - days * self.lod - ut2_ut1
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Precision {
    base: f64,
    pow: f64,
    mjd: i32,
}

impl Precision {
    fn at(self, mjd: i32) -> f64 {
        let days = (mjd - self.mjd) as f64;
        self.base * days.powf(self.pow)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct BulletinA {
    date: Gregorian,
    pred: Prediction,
    prec: Precision,
}

impl BulletinA {
    fn dut1_at(self, date: Gregorian) -> UT1_UTC {
        let mjd = i32::from(date);
        let val = self.pred.at(mjd);
        let err = self.prec.at(mjd);
        let lod = self.pred.lod;
        UT1_UTC { val, err, lod }
    }
    fn leap_at(self, thresh: f64, date: Gregorian) -> Leap {
        if self.date > date {
            return Leap::Zero;
        }
        let dut1 = self.dut1_at(date);
        let lo = dut1.val - dut1.err;
        let hi = dut1.val + dut1.err;
        if dut1.lod < 0.0 && 0.0 < lo && thresh < hi {
            return Leap::Neg(date, dut1);
        }
        if dut1.lod > 0.0 && lo < -thresh && hi < 0.0 {
            return Leap::Pos(date, dut1);
        }
        Leap::Zero
    }
    fn leap(self, thresh: f64) -> Leap {
        for year in 2000..3000 {
            match self.leap_at(thresh, Gregorian(year, 6, 30)) {
                Leap::Zero => (),
                leap => return leap,
            };
            match self.leap_at(thresh, Gregorian(year, 12, 31)) {
                Leap::Zero => (),
                leap => return leap,
            };
        }
        Leap::Zero
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
struct UT1_UTC {
    val: f64,
    err: f64,
    lod: f64,
}

impl std::fmt::Display for UT1_UTC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UT1-UTC {:+.3} ± {:.3} s (lod {:+.0} µs)",
            self.val,
            self.err,
            self.lod * 1000000.0,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Leap {
    Neg(Gregorian, UT1_UTC),
    Pos(Gregorian, UT1_UTC),
    Zero,
}

impl std::fmt::Display for Leap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Leap::Zero => write!(f, "????-??-?? (?)"),
            Leap::Neg(date, dut1) => write!(f, "{} (-) {}", date, dut1),
            Leap::Pos(date, dut1) => write!(f, "{} (+) {}", date, dut1),
        }
    }
}

// silly thing so that fetching a URL looks like reading a file
struct URL(String);

impl URL {
    fn read_to_end(&self, data: &mut Vec<u8>) -> Result<()> {
        let mut ua = curl::easy::Easy::new();
        ua.useragent("fanf/1.0")?;
        ua.fail_on_error(true)?;
        ua.url(&self.0)?;
        let mut xfer = ua.transfer();
        xfer.write_function(|chunk| {
            data.extend_from_slice(chunk);
            Ok(chunk.len())
        })?;
        xfer.perform()?;
        Ok(())
    }
}

fn latest_bulletin_a() -> Result<i32> {
    use std::time::SystemTime;
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let unix_date = now.as_secs() / (24 * 60 * 60);
    let mjd = i32::from(Gregorian(1970, 1, 1)) + unix_date as i32;
    let zero = i32::from(Gregorian(2005, 1, 13));
    Ok((mjd - zero) / 7)
}

fn get_bulletin_a(issue: i32) -> Result<(Gregorian, String)> {
    // no bulletin issued on 2009-01-01 which would have been 208
    // first available issue
    let zero =
        i32::from(Gregorian(2005, 1, 6)) + if issue >= 208 { 7 } else { 0 };
    // published weekly
    let date = Gregorian::from(zero + issue * 7);
    let year = date.year();
    let volume = Roman(year - 1987);
    // volume number is week within year
    let janus = i32::from(Gregorian(year, 1, 1));
    let week = (i32::from(date) - janus) / 7 + if year == 2009 { 0 } else { 1 };
    let weeks = format!("{:03}", week);
    let prefix = "https://datacenter.iers.org/data/6/bulletina";
    let url = URL(format!("{}-{}-{}.txt", prefix, volume, weeks));
    // local cache
    let dir = format!("bula/{}", year);
    let file = format!("{}/{}", dir, weeks);
    std::fs::create_dir_all(dir)?;
    let mut data = Vec::new();
    if let Ok(mut fh) = std::fs::File::open(&file) {
        fh.read_to_end(&mut data)
            .with_context(|| format!("failed to read {}", file))?;
    } else {
        eprintln!("fetching {}", &url.0);
        url.read_to_end(&mut data)
            .with_context(|| format!("failed to get {}", url.0))?;
        std::fs::write(&file, &data)?;
    }
    // strings work more easily with nom
    Ok((date, String::from_utf8(data)?))
}

type ParseResult<'a> =
    nom::IResult<&'a str, BulletinA, nom::error::VerboseError<&'a str>>;

fn parse_bulletin_a<'a>(date: Gregorian, bula: &'a str) -> ParseResult<'a> {
    use nom::branch::*;
    use nom::bytes::complete::*;
    use nom::character::complete::*;
    use nom::combinator::*;
    use nom::multi::*;
    use nom::number::complete::*;
    use nom::sequence::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum State {
        Skip,
        Pred(Prediction),
        Prec(Precision),
        BulA(Prediction, Precision),
        Clash(&'static str),
    }
    use State::*;

    let tail = || tuple((space0, line_ending));

    // UT1-UTC = -0.NNNNN +- 0.000NNN (MJD - NNNNN) - (UT2-UT1)
    let pred = map(
        tuple((
            preceded(tuple((space0, tag("UT1-UTC ="), space0)), double),
            preceded(space0, one_of("+-")),
            preceded(space0, double),
            delimited(
                tuple((space0, tag("(MJD - "))),
                double,
                tuple((space0, tag(") - (UT2-UT1)"), tail())),
            ),
        )),
        |t| {
            Pred(Prediction {
                dut1: t.0,
                lod: match t.1 {
                    '-' => t.2,
                    _ => -t.2,
                },
                mjd: t.3 as i32,
            })
        },
    );

    // S x,y = 0.00068 (MJD-59326)**0.80   S t = 0.00025 (MJD-59326)**0.75
    let precision = |var| {
        map(
            tuple((
                preceded(
                    tuple((space0, tag("S "), tag(var), tag(" = "))),
                    double,
                ),
                preceded(tag(" (MJD-"), double),
                preceded(tag(")**"), double),
            )),
            |t| Prec(Precision { base: t.0, mjd: t.1 as i32, pow: t.2 }),
        )
    };
    let prec = delimited(precision("x,y"), precision("t"), tail());

    let skip = value(Skip, pair(not_line_ending, line_ending));

    let line = alt((pred, prec, skip));

    let mut parse = map_res(
        complete(fold_many0(line, Skip, |acc, item| match (acc, item) {
            (acc, Skip) => acc,
            (Skip, Pred(pred)) => Pred(pred),
            (Pred(pred), Prec(prec)) => BulA(pred, prec),
            _ => Clash("multiple equations"),
        })),
        |t| match t {
            BulA(pred, prec) => Ok(BulletinA { date, pred, prec }),
            Clash(err) => Err(err),
            _ => Err("missing equation"),
        },
    );

    parse(bula)
}

fn bulletin_a(issue: i32) -> Result<BulletinA> {
    let (date, bula) = get_bulletin_a(issue)?;
    let (_, param) =
        parse_bulletin_a(date, &bula).map_err(|e| anyhow!("{}", e))?;
    Ok(param)
}
