use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let current_path: std::path::PathBuf = env::current_dir()?;
    println!("Current directory : {}", current_path.display());
    let target: &str = "./";
    let mut files: Vec<String> = Vec::new();

    match fs::read_dir(target) {
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(dir_entry) => files.push(
                        dir_entry
                            .path()
                            .display()
                            .to_string()
                            .replacen(target, "", 1),
                    ),
                    Err(e) => println!("Error reading directory entry : {}", e),
                }
            }
        }
        Err(e) => println!("Error reading directory : {}", e),
    }

    files.sort();

    let strings: String = files
        .iter()
        .fold(String::new(), |joined: String, s: &String| {
            if joined == String::new() {
                s.to_string()
            } else {
                joined + "\n" + s
            }
        });
    println!("{}", strings);
    Ok(())
}
