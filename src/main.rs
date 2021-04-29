mod date;
mod euclid;
mod roman;

use anyhow::{anyhow, Context, Result};
use date::*;
use roman::Roman;
use std::io::Read;

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

fn get_bulletin_a(issue: i32) -> Result<String> {
    // first available issue
    let zero = i32::from(gregorian(2005, 1, 6));
    // published weekly
    let date = Gregorian::from(zero + issue * 7);
    let year = date.year();
    let volume = Roman(year - 1987);
    // volume number is week within year
    let janus = i32::from(gregorian(year, 1, 1)) - 1;
    let week = (i32::from(date) - janus) / 7;
    let weeks = format!("{:03}", week);
    let prefix = "https://datacenter.iers.org/data/6/bulletina";
    let url = URL(format!("{}-{:?}-{}.txt", prefix, volume, weeks));
    // local cache
    let dir = format!("bula/{}", year);
    let file = format!("{}/{}", dir, weeks);
    std::fs::create_dir_all(dir)?;
    let mut data = Vec::new();
    if let Ok(mut fh) = std::fs::File::open(&file) {
        eprintln!("reading {}", &file);
        fh.read_to_end(&mut data)
            .with_context(|| format!("failed to read {}", file))?;
    } else {
        eprintln!("fetching {}", &url.0);
        url.read_to_end(&mut data)
            .with_context(|| format!("failed to get {}", url.0))?;
        std::fs::write(&file, &data)?;
    }
    // to work more easily with nom
    Ok(String::from_utf8(data)?)
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct BulletinA {
    dut1: Option<f64>,
    lod: Option<f64>,
    mjd_pred: Option<i32>,
    mjd_err: Option<i32>,
}

const NO_BUL_A: BulletinA =
    BulletinA { dut1: None, lod: None, mjd_pred: None, mjd_err: None };

type VerboseResult<'a, O> =
    nom::IResult<&'a str, O, nom::error::VerboseError<&'a str>>;

fn parse_bulletin_a<'a>(bula: &'a str) -> VerboseResult<'a, BulletinA> {
    use nom::branch::alt;
    use nom::bytes::complete::*;
    use nom::character::complete::*;
    use nom::combinator::*;
    use nom::multi::*;
    use nom::sequence::*;

    // zero-sized output value
    let skip = value((), pair(not_line_ending, line_ending));
    // assume there's no need to alloc a vec of zero-sized outputs
    let skip_till = |p| map(many_till(skip, p), |pair| pair.1);

    let eqn = value(
        BulletinA { dut1: Some(0.0), ..NO_BUL_A },
        tuple((space1, tag("UT1-UTC = "), not_line_ending, line_ending)),
    );

    let mut parse = skip_till(eqn);
    parse(bula)
}

fn main() -> Result<()> {
    let bula = get_bulletin_a(850)?;
    let (rest, value) =
        parse_bulletin_a(&bula).map_err(|e| anyhow!("{}", e))?;
    eprintln!("value: {:?}", value);
    eprint!("rest:\n>{}<", rest);
    Ok(())
}
