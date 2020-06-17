use std::collections::HashMap;

pub struct Expression{
    input_map: HashMap<i32, i32>,
    output_map: HashMap<i32, i32>,
}

impl Expression{

    pub fn new(input_map: HashMap<i32,i32>, output_map: HashMap<i32,i32>) -> Expression{
        let expression = Expression{
            input_map,
            output_map,
        };
        return expression;
    }



}
