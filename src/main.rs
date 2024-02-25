use std::fs::File;
use std::io::Read;

fn main() {
    
    let f = File::open("foo.txt");
    // let f: = match f {
    //     Ok(file) => {
    //         println!("File opened successfully");
    //         file
    //     }
    //     Err(err) => println!("Error opening file: {}", err)
    // };

    let mut f = match f {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    // let mut data = vec![];
    // f.read_to_end(&mut data);
    // println!("{:?}", data);

    let mut content = String::new();
    match f.read_to_string(&mut content) {
        Ok(_) => {
            println!("File read successfully");
            println!("{}", content);
        }
        Err(err) => println!("Error reading file: {}", err)
    }
    
    println!("Hello, world!");
}
