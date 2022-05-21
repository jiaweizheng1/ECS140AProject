use std::env;
use std::io::Read;
use std::fs::File;
use std::process::exit; 

struct CStream
{
    contents: String,
}

impl CStream
{
    fn init(in_file_name: &str) -> CStream
    {
        let mut file = File::open(in_file_name.to_string()).expect("Error Opening File");
        let mut temp_contents = String::new();
        file.read_to_string(&mut temp_contents).expect("Error Reading File");

        CStream
        {
            contents: temp_contents,
        }
    }
}

fn main() {
    //collect additional arguments after "cargo run" for txt file input name
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args.len() > 2 
    {
        println!("Error: Missing File Name");
        exit(1);
    }

    let mut f: CStream = CStream::init(&args[1]);

    println!("{:?}", f.contents);
}