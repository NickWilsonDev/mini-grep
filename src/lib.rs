/// lib.rs
use std::fs::File;

fn handle_file(&filename) {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        println!("With text:\n{}", contents);

}

