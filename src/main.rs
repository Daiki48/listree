use std::fs;

fn main() {
    let target = "./";
    let mut files: Vec<String> = Vec::new();
    for path in fs::read_dir(target).unwrap(){
        files.push(path.unwrap().path().display().to_string().replacen(target, "", 1))
    }
    files.sort();
    let strings = files.iter()
        .fold(String::new(), |joined, s| {
            if joined == String::new() { s.to_string() } else { joined + " " + s }
        });

    println!("{}", strings);
}
