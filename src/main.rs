mod date;
mod euclid;
mod roman;

use anyhow::{Context, Result};
use date::*;
use roman::Roman;
use std::io::{Read, Write};

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

fn get_bulletin_a(issue: i32) -> Result<Vec<u8>> {
    let zero = i32::from(gregorian(2005, 1, 6));
    let date = Gregorian::from(zero + issue * 7);
    let year = date.year();
    let volume = Roman(year - 1987);
    let janus = i32::from(gregorian(year, 1, 1)) - 1;
    let week = (i32::from(date) - janus) / 7;
    let weeks = format!("{:03}", week);
    let prefix = "https://datacenter.iers.org/data/6/bulletina";
    let url = URL(format!("{}-{:?}-{}.txt", prefix, volume, weeks));
    let dir = format!("bula/{}", year);
    let file = format!("{}/{}", dir, weeks);
    std::fs::create_dir_all(dir)?;
    let mut data = Vec::new();
    if let Ok(mut f) = std::fs::File::open(&file) {
        println!("reading {}", &file);
        f.read_to_end(&mut data)
            .with_context(|| format!("failed to read {}", file))?;
    } else {
        println!("fetching {}", &url.0);
        url.read_to_end(&mut data)
            .with_context(|| format!("failed to get {}", url.0))?;
        std::fs::write(&file, &data)?;
    }
    Ok(data)
}

fn main() -> Result<()> {
    std::io::stdout().write_all(&get_bulletin_a(850)?)?;
    Ok(())
}
