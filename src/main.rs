extern crate ack;

use std::io;
use std::io::prelude::*;
// use std::fs;

use ack::parser;
use ack::runtime;

fn console_log(arguments: Vec<runtime::Value>, _: &runtime::Object) -> runtime::Value {
    for value in arguments {
        print!("{:?} ", value);
    }
    println!("");

    runtime::Value::Undefined
}

fn main() {
    // let source = {
    //     let mut f = fs::File::open("foo.js").ok().expect("Could not open foo.js");
    //     let mut s = String::new();
    //     f.read_to_string(&mut s).ok().expect("Could not read foo.js");

    //     s
    // };

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let source = {
            let mut total = String::new();

            // hacky, but it's just a temporary testing solution
            let mut again = true;

            while again {
                again = false;
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();

                let mut s: Vec<char> = line.trim().chars().collect();
                let last = s.pop();

                match last {
                    Some('\\') => again = true,
                    Some(c) => s.push(c),
                    None => ()
                }

                s.push('\n');

                for c in s {
                    total.push(c)
                }
            }

            total
        };

        let parsed = parser::parse(&source);

        println!("");
        println!("AST: {:?}", parsed);

        match parsed {
            Ok(ast) => {
                let mut global = runtime::Object::new();
                let mut console = runtime::Object::new();
                console.insert("log".to_string(), runtime::Value::Function(runtime::Rc::new(runtime::Function::Native(console_log))));
                global.insert("console".to_string(), runtime::Value::Object(console));
                let result = runtime::eval(ast, global);
                println!("Result: {:?}", result);
            },
            Err(e) => println!("{:?}", e)
        }
    }
}
