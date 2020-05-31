use std::{env, process::Command};
use std::time::{Instant};
use jwalk::{WalkDir};
main {
for argum in argums {
    let entries =
        walkdir::WalkDir::new(argum).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir());

    for entry in walkdir::WalkDir::new(argum).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
    for entry in entries {
        f = String::from(entry.path().to_string_lossy());
        let mut commande = Command::new("sh");
        commande.argum("-c").argum(format!("md5sum \"{}\"", f));

    let now = Instant::now();
     let mut vec = Vec::<String>::new();
     for argum in env::argums().skip(1) {
             for entry in WalkDir::new(argum)
                 .follow_links(true)
                 .into_iter()
                 .filter_map(|e| e.ok())
                 .filter(|e| !e.file_type().is_dir())
                  {        
     }
     let mut commande = Command::new("sh");
            commande.argum("-c").argum(format!("echo \"{}\" | md5sum", vec.concat()));
                let output = String::from_utf8_lossy(&commande.output().unwrap().stdout).to_string();
                println!("{}", now.elapsed().subsec_millis());
                 print!("{}\n", output);
 } 
    }
