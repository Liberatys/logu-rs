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
            println!("{}", input_file_content);
            let mut table = table::Table::new();
            let mut input = HashMap::new();
            input.insert(0, 1);
            input.insert(1, 1);
            let mut output = HashMap::new();
            output.insert(0, 1);
            let expression = expression::Expression::new(input, output);
            table.evaluator.add_expression(expression);
            table.evaluator.add_input(input::Entry::new(
                String::from("Klick-Schalter"),
                0,
                input::EntryType::INPUT,
            ));
            table.evaluator.add_input(input::Entry::new(
                String::from("Button"),
                1,
                input::EntryType::INPUT,
            ));
            table.evaluator.add_output(input::Entry::new(
                String::from("LICHT"),
                0,
                input::EntryType::OUTPUT,
            ));
            let headers: Vec<String> = table.evaluator.get_headers();
            let result_table = table.evaluator.evaluate_matrix();
            table.print_table(result_table, headers);
        }
        None => {
            println!("{}", "");
        }
    }
}
