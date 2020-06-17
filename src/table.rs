use crate::evaluator;

pub struct Table{
    pub evaluator: evaluator::Evaluator,
}

impl Table{
    pub fn new() -> Table{
        let table = Table{
            evaluator: evaluator::Evaluator::new(),
        };
        return table;
    }
}
