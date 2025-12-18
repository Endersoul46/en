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

// fn capture_error<R: Read>(stdout: R)-> io::Result<()> {
//     let reader = BufReader::new(stdout);
//     let mut errors: Vec<String> = vec![]; 
//
//     for line in reader.lines() {
//         let line = line?;
//         println!("> {}", line);
//         if line.to_lowercase().contains("error") {
//             errors.push(line.clone());
//         }
//     }
//
//     for er in errors {
//         println!("{}", er);
//     }
//
//     Ok(())
// }

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

pub fn home_update() -> Result<(), io::Error> {
    let stdout = cmd!("nh", "home", "switch").reader()?;
    capture_out(stdout)?;
    Ok(())
}

pub fn nix_os_update() -> Result<(), io::Error> {
    let stdout = cmd!("nh", "os", "switch").reader()?;
    capture_out(stdout)?;
   Ok(())
}


pub fn flake_update() -> Result<(), io::Error> {
    let flake = env::var("NH_FLAKE").expect("var wansn't found");
    let stdout = cmd!("nix", "flake", "update", "--flake", flake).reader()?;
    capture_out(stdout)?;
    Ok(())
}


