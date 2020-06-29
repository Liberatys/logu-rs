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
            let mut expressions = &parsed["expressions"];
            let inputs = unwrap_json_format_into_vec(&parsed, "inputs");
            let outputs = unwrap_json_format_into_vec(&parsed, "outputs");
            for x in 0..inputs.len() {
                table.evaluator.add_input(input::Entry::new(
                    inputs[x].to_string(),
                    x as i32,
                    input::EntryType::INPUT,
                ));
            }
            for x in 0..outputs.len() {
                table.evaluator.add_output(input::Entry::new(
                    outputs[x].to_string(),
                    x as i32,
                    input::EntryType::OUTPUT,
                ));
            }
            match expressions {
                json::JsonValue::Array(v) => {
                    for x in 0..v.len() {
                        let mut input_hash = HashMap::<i32, i32>::new();
                        let mut output_hash = HashMap::<i32, i32>::new();
                        let inner_inputs = unwrap_json_format_into_vec(&v[x], "inputs");
                        let inner_outputs = unwrap_json_format_into_vec(&v[x], "outputs");

                        for x in 0..inner_inputs.len() {
                            let values: Vec<&str> = inner_inputs[x].split("=").collect();
                            let first = values[0].parse::<i32>().unwrap();
                            let second = values[1].parse::<i32>().unwrap();
                            input_hash.insert(first, second);
                        }

                        for x in 0..outputs.len() {
                            let values: Vec<&str> = inner_outputs[x].split("=").collect();
                            let first = values[0].parse::<i32>().unwrap();
                            let second = values[1].parse::<i32>().unwrap();
                            output_hash.insert(first, second);
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

fn unwrap_json_format_into_vec(value: &json::JsonValue, name: &str) -> Vec<String> {
    let mut result_vec: Vec<String> = Vec::new();
    match &value[name] {
        json::JsonValue::Array(array) => {
            for x in 0..array.len() {
                result_vec.push(array[x].as_str().unwrap().to_owned());
            }
        }
        _ => {
            return result_vec;
        }
    }
    return result_vec;
}
