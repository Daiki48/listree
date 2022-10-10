use std::fs;

/// Displays the current directory, like the `ls` command.
///

fn main() {
    let target = "./";
    let mut files: Vec<String> = Vec::new();    // init Vec

    // Store in vec
    for path in fs::read_dir(target).unwrap(){
        files.push(
            path.unwrap()
                .path()
                .display()
                .to_string()
                .replacen(target, "", 1)
        )
    }

    // sorting
    files.sort();

    // Converts from vec to String
    let strings = files.iter()
        .fold(String::new(), |joined, s| {
            if joined == String::new() {
                s.to_string()
            } else {
                joined + " " + s
            }
        });
    println!("{}", strings);

}
