use crate::evaluator;
use prettytable::{Cell, Row};

pub struct Table {
    pub evaluator: evaluator::Evaluator,
}

impl Table {
    pub fn new() -> Table {
        let table = Table {
            evaluator: evaluator::Evaluator::new(),
        };
        return table;
    }

    pub fn print_table(self, table_data: Vec<Vec<i32>>, table_headers: Vec<String>) {
        let mut table = prettytable::Table::new();
        let mut current_cells: Vec<Cell> = Vec::new();
        for i in 0..table_headers.len() {
            current_cells.push(Cell::new(table_headers[i].as_ref()));
        }
        table.add_row(Row::new(current_cells));
        for x in 0..table_data.len() {
            let mut row_cells = Vec::new();
            for i in 0..table_data[x].len() {
                row_cells.push(Cell::new(table_data[x][i].to_string().as_ref()));
            }
            table.add_row(Row::new(row_cells));
        }
        table.printstd();
    }
}
