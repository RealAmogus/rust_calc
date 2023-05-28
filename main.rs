use std::{error, io};
use std::thread::park;

#[derive(Debug)]
enum NotAnOperationError {

    InvalidSign,
    InvalidValue

}

fn eval_calc(vec: Vec<&str>) -> Result<f64, NotAnOperationError> {

    if let Err(_e) = vec.get(0).unwrap().parse::<f64>() {

        return Err(NotAnOperationError::InvalidValue)

    }
    else if let Err(_e) = vec.get(2).unwrap().parse::<f64>() {

        return Err(NotAnOperationError::InvalidValue)

    }

    let val1 = vec.get(0).unwrap().parse::<f64>().unwrap();
    let val2 = vec.get(2).unwrap().parse::<f64>().unwrap();

    match *vec.get(1).unwrap() {

        "+" => Ok(val1 + val2),
        "-" => Ok(val1 - val2),
        "*" => Ok(val1 * val2),
        "/" => Ok(val1 / val2),
        "log" => Ok(val2.log(val1)),
        "^" => Ok(val1.powf(val2)),
        &_ => Err(NotAnOperationError::InvalidSign)

    }

}

fn main() -> Result<(), Box<dyn error::Error>> {

    println!("Supported operations:
    value + value
    value - value
    value * value
    value / value
    base log exponent
    base ^ power");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    let input_iter = user_input.split_whitespace();

    let mut input_vec: Vec<&str> = Vec::new();

    for str in input_iter {

        input_vec.push(str);

    }

    let calc_result = eval_calc(input_vec.clone());

    if input_vec.len() == 3 && calc_result.is_ok() {

        println!("{}", calc_result.unwrap());

    }
    else {

        println!("Provide a valid input.");

    }

    park();
    Ok(())

}
