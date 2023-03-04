use wasm_bindgen::prelude::*;

// // Called when the wasm module is instantiated
// #[wasm_bindgen(start)]
// pub fn main() -> Result<(), JsValue> {
//     // Use `web_sys`'s global `window` function to get a handle on the global
//     // window object.
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");

//     // Manufacture the element we're gonna append
//     let val = document.create_element("p")?;
//     val.set_inner_html("Hello from Rust!");

//     body.append_child(&val)?;

//     Ok(())
// }

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello hot-reloading, {}!", name));
}

trait Command {
    fn execute(&self, args: Vec<&str>) -> String;
    fn help(&self) -> String;
    fn name(&self) -> String;
}

struct Welcome;

impl Command for Welcome {
    fn name(&self) -> String {
        String::from("welcome")
    }

    fn execute(&self, _args: Vec<&str>) -> String {
        String::from("Welcome to the Rust shell!")
    }

    fn help(&self) -> String {
        // print a help message on how to use the command
        String::from("Usage: welcome

Prints a welcome message.")
    }
}

#[wasm_bindgen]
pub fn evaluate_expression(expr: &str) -> String {
    let expr = expr.split_whitespace().collect::<Vec<&str>>();
    let command = expr.get(0);

    let commands = vec![Welcome];

    match command {
        Some(&"help") => {
            if let Some(command) = expr.get(1) {
                for c in commands {
                    if c.name() == *command {
                        return c.help();
                    }
                }
                return String::from(format!("{}: command not found", command));
            }
            let mut help = String::from("Available commands: ");
            for c in commands {
                help.push_str(&format!("{} ", c.name()));
            }
            help
        }
        Some(command) => {
            for c in commands {
                if c.name() == *command {
                    return c.execute(expr);
                }
            }
            String::from(format!("{}: command not found", command))
        }
        None => String::from("Error: no command given"),
        _ => String::from(format!("{}: command not found", command.unwrap())),
    }
}