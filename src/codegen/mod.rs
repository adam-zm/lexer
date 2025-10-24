use crate::parser::Operation;

pub fn eval(op: &Operation) {
    match op {
        Operation::Addition(n1, n2) => {
            println!("Addition = {}", n1 + n2);
        }
        _ => {}
    }
}
