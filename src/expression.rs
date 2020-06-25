use std::collections::HashMap;

pub struct Expression {
    input_map: HashMap<i32, i32>,
    output_map: HashMap<i32, i32>,
}

pub enum ExpressionResult {
    Ok(HashMap<i32, i32>),
    NONE,
}

impl Expression {
    pub fn new(input_map: HashMap<i32, i32>, output_map: HashMap<i32, i32>) -> Expression {
        let expression = Expression {
            input_map,
            output_map,
        };
        return expression;
    }

    pub fn evaluate_expression(&self, input_vec: &Vec<i32>) -> ExpressionResult {
        for (key, value) in &self.input_map {
            if (input_vec[*key as usize] == *value) == false {
                return ExpressionResult::NONE;
            }
        }
        // return a clone of the hashMap
        return ExpressionResult::Ok(self.output_map.clone());
    }
}
