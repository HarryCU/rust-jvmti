extern crate jvmti;

use std::env;
use std::fs::File;
use std::io::{stdout};

use jvmti::bytecode::*;

// The main program is a simple interface to access the bytecode parsing and generating
// functionality and as such, it's not intended for actual use.
fn main() {
    if let (Some(action), Some(class_name)) = (env::args().nth(1), env::args().nth(2)) {
        match File::open(class_name) {
            Ok(mut file) => {
                match ClassReader::read_class(&mut file) {
                    Ok(class) => {
                        match action.as_str() {
                            "read" => println!("{}", format!("{:#?}", class)),
                            "write" => {
                                let mut out = stdout();
                                let mut writer = ClassWriter::new(&mut out);
                                let _ = writer.write_class(&class);
                            },
                            _ => println!("Unknown action: {}", action)
                        }
                    },
                    Err(err) => assert!(false, format!("{:?}", err))
                }

            },
            Err(err) => assert!(false, format!("{:?}", err))
        }
    } else {
        println!("Invalid arguments. Usage: jvmti [read|write] <Class file>")
    }
}
