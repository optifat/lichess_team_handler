use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn read_blacklist_file(path: &str) -> anyhow::Result<HashSet<String>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let cheaters = buf
        .lines()
        .into_iter()
        .map(|cheater| cheater.map(|id| id.to_lowercase()))
        .collect::<Result<HashSet<String>, _>>()?;
    println!("{:?}", cheaters);
    Ok(cheaters)
}
