//TODO
pub struct Constraint {
    predicate_function: fn(&Vec<Variable>) -> bool,
    error_function: fn(&Vec<Variable>) -> i32,
    variables: Vec<Variable>,
    param: i32,
}

impl Constraint {
    pub fn new(
        predicate_function: fn(&Vec<Variable>) -> bool,
        error_function: fn(&Vec<Variable>) -> i32,
        variables: Vec<Variable>,
    ) -> Self {
        Constraint {
            predicate_function: predicate_function,
            error_function: error_function,
            variables: variables,
            param: None,
        }
    }

    pub fn new(
        predicate_function: fn(&Vec<Variable>) -> bool,
        error_function: fn(&Vec<Variable>) -> i32,
        variables: Vec<Variable>,
        param: i32,
    ) -> Self {
        Constraint {
            predicate_function: predicate_function,
            error_function: error_function,
            variables: variables,
            param: param,
        }
    }

    pub fn get_predicate_function(&self) -> fn(&Vec<Variable>) -> bool {
        self.predicate_function
    }
    pub fn get_error_function(&self) -> fn(&Vec<Variable>) -> i32 {
        self.error_function
    }

    pub fn get_variables(&self) -> &Vec<Variable> {
        &self.variables
    }

    pub fn update_variables(&mut self, variables: Vec<Variable>) {
        self.variables = variables;
    }
}
