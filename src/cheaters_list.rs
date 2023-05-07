use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn read_cheaters_list_file(path: String) -> anyhow::Result<HashSet<String>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let cheaters = buf
        .lines()
        .into_iter()
        .collect::<Result<HashSet<String>, _>>()?;
    println!("{:?}", cheaters);
    Ok(cheaters)
}
