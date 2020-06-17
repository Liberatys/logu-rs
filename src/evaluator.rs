use crate::input;
use crate::expression;

pub struct Evaluator{
    inputs: Vec<input::Entry>,
    outputs: Vec<input::Entry>,
    expressions: Vec<expression::Expression>,
}

impl Evaluator{
    pub fn new() -> Evaluator{
        let eval = Evaluator{
        inputs: Vec::new(), outputs: Vec::new(), expressions: Vec::new(),};
        return eval;
    }

    pub fn add_input(&mut self, input: input::Entry){
       self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: input::Entry){
        self.outputs.push(output);
    }

    pub fn add_expression(&mut self, expression: expression::Expression){
        self.expressions.push(expression);
    }

    pub fn generate_default_bit_table(&self) -> Vec<Vec<i32>>{
        let mut evaluation_matrix : Vec<Vec<i32>>= Vec::new();
        let hor_vector_length = self.inputs.len() + self.outputs.len();
        let vec_vector_length = i32::pow(2,self.inputs.len() as u32);
        for i in 0..vec_vector_length{
            let mut current_vec : Vec<i32> = Vec::new();
            for x in 0..hor_vector_length{
                current_vec.push(0);
            }
            evaluation_matrix.push(current_vec);
        }
        for index in 0..vec_vector_length{
                let binary_sequence = format!("{:b}",index);
                let mut binary_vec:Vec<&str> = binary_sequence.split("").collect();
                // remove all empty strings in vec
                binary_vec.retain(|&x| x != "");
                for y in 0..binary_vec.len(){
                    let transfered_index_position = hor_vector_length - y - 1 - self.outputs.len(); // subtract with the length of output to get the first starting integer for input
                    let bit_value : i32 = binary_vec[binary_vec.len() - 1 - y].parse().unwrap(); // translate the index of the first bit value to the index of the array
                    evaluation_matrix[index as usize][transfered_index_position as usize] = bit_value;
                }
        }
        return evaluation_matrix;
    }

    pub fn evaluate_matrix(&self) -> Vec<Vec<i32>>{
        let mut evaluation_matrix = self.generate_default_bit_table();
        let vec_vector_length = i32::pow(2,self.inputs.len() as u32);
        let input_index_range = self.inputs.len();
        for x in 0..vec_vector_length{
            for t in 0..self.expressions.len(){
            let result = self.expressions[t as usize].evaluate_expression(&evaluation_matrix[x as usize]);
            match result{
                expression::ExpressionResult::Ok(v) =>{
                    for (key, value) in v {
                        evaluation_matrix[x as usize][(key + input_index_range as i32)as usize] =value
                    }
                },
                expression::ExpressionResult::NONE=> continue,
            }
            }
        }
        return evaluation_matrix;
    }
}
