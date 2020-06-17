use std::collections::HashMap;
mod table;
mod input;
mod expression;
mod evaluator;

fn main() {
    let mut table = table::Table::new();
    let mut input = HashMap::new();
    input.insert(0,1);
    input.insert(1,1);
    let mut output = HashMap::new();
    output.insert(0,1);
    let expression = expression::Expression::new(input,output);
    table.evaluator.add_expression(expression);
    table.evaluator.add_input(input::Entry::new(String::from("Klick-Schalter"), 0, input::EntryType::INPUT));
    table.evaluator.add_input(input::Entry::new(String::from("Button"), 1, input::EntryType::INPUT));
    table.evaluator.add_output(input::Entry::new(String::from("LICHT"), 0, input::EntryType::OUTPUT));
    table.evaluator.evaluate_matrix();
}

