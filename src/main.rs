use std::collections::HashMap;
mod evaluator;
mod expression;
mod input;
mod reader;
mod table;
#[macro_use]
extern crate prettytable;
use ace::App;

fn main() {
    let app = App::new()
        .config("Logu", env!("CARGO_PKG_VERSION"))
        .cmd("run", "Run the table generator with given --i and --o")
        .cmd("help", "")
        .cmd("version", env!("CARGO_PKG_VERSION"))
        .opt("--input", "Input file")
        .opt("--output", "Output file");
    if let Some(cmd) = app.command() {
        match cmd.as_str() {
            "help" => {
                app.print_help();
            }
            "version" => {
                app.print_version();
            }
            "run" => {
                run_table_generation(app.value("--input"), app.value("--output"));
            }
            _ => {
                app.print_error_try("help");
            }
        }
    } else {
        dbg!(app.args());
    }
}

fn run_table_generation(input: Option<Vec<&String>>, output: Option<Vec<&String>>) {
    match input {
        Some(v) => {
            let input_file_content = reader::read_json_file(v[0].as_ref());
            let parsed = json::parse(input_file_content.as_ref()).unwrap();
            let mut table = table::Table::new();
            let mut inputs = &parsed["inputs"];
            let mut outputs = &parsed["outputs"];
            let mut expressions = &parsed["expressions"];
            match inputs {
                json::JsonValue::Array(v) => {
                    for x in 0..v.len() {
                        table.evaluator.add_input(input::Entry::new(
                            String::from(v[x].as_str().unwrap()),
                            x as i32,
                            input::EntryType::INPUT,
                        ));
                    }
                }
                _ => {}
            }
            match outputs {
                json::JsonValue::Array(v) => {
                    for x in 0..v.len() {
                        table.evaluator.add_output(input::Entry::new(
                            String::from(v[x].as_str().unwrap()),
                            x as i32,
                            input::EntryType::OUTPUT,
                        ));
                    }
                }
                _ => {}
            }
            match expressions {
                json::JsonValue::Array(v) => {
                    for x in 0..v.len() {
                        let mut input_hash = HashMap::<i32, i32>::new();
                        let mut output_hash = HashMap::<i32, i32>::new();
                        let inner_inputs = &v[x]["inputs"];
                        let inner_outputs = &v[x]["outputs"];
                        match inner_inputs {
                            json::JsonValue::Array(v) => {
                                for x in 0..v.len() {
                                    let values: Vec<&str> =
                                        v[x].as_str().unwrap().split("=").collect();
                                    let first = values[0].parse::<i32>().unwrap();
                                    let second = values[1].parse::<i32>().unwrap();
                                    input_hash.insert(first, second);
                                }
                            }
                            _ => {}
                        }
                        match inner_outputs {
                            json::JsonValue::Array(v) => {
                                for x in 0..v.len() {
                                    let values: Vec<&str> =
                                        v[x].as_str().unwrap().split("=").collect();
                                    let first = values[0].parse::<i32>().unwrap();
                                    let second = values[1].parse::<i32>().unwrap();
                                    output_hash.insert(first, second);
                                }
                            }
                            _ => {}
                        }
                        table
                            .evaluator
                            .add_expression(expression::Expression::new(input_hash, output_hash));
                    }
                }
                _ => {}
            }
            let headers: Vec<String> = table.evaluator.get_headers();
            let result_table = table.evaluator.evaluate_matrix();
            table.print_table(result_table, headers);
        }
        None => {
            println!("{}", "");
        }
    }
}

