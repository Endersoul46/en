use std::env;
use std::io::{self, BufReader, BufRead, Read};


use duct::cmd;

fn capture_out<R: Read>(stdout: R)-> io::Result<()> {
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        println!("> {}", line?);
    }

    Ok(())
}

pub fn update() -> Result<(), io::Error> {
    let flake = env::var("NH_FLAKE").expect("var wansn't found");
    let stdout = cmd!("nix", "flake", "update", "--flake", flake).reader()?;
    capture_out(stdout)?;
    let stdout = cmd!("nh", "home", "switch").reader()?;
    capture_out(stdout)?;
    let stdout = cmd!("nh", "os", "switch").reader()?;
    capture_out(stdout)?;
    Ok(())
}
