use std::error::Error;
use std::fmt::{Debug, Display, Error as fmtError, Formatter};
use std::io;

enum NotAnOperationError {
    InvalidSign,
    InvalidValue,
}

impl Debug for NotAnOperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmtError> {
        match self {
            NotAnOperationError::InvalidSign => write!(f, "Couldn't recognize the operation."),
            NotAnOperationError::InvalidValue => write!(f, "Couldn't parse the input to f64."),
        }
    }
}

impl Display for NotAnOperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmtError> {
        match self {
            NotAnOperationError::InvalidSign => write!(f, "Couldn't recognize the operation."),
            NotAnOperationError::InvalidValue => write!(f, "Couldn't parse the input to f64."),
        }
    }
}

impl Error for NotAnOperationError {}

fn eval_calc(vec: Vec<&str>) -> Result<f64, NotAnOperationError> {
    let val1 = vec[0].parse::<f64>();
    let val2 = vec[2].parse::<f64>();

    let Some((val1, val2)) = val1.ok().zip(val2.ok())
        else {
        return Err(NotAnOperationError::InvalidValue);
    };

    match vec[1] {
        "+" => Ok(val1 + val2),
        "-" => Ok(val1 - val2),
        "*" => Ok(val1 * val2),
        "/" => Ok(val1 / val2),
        "log" => Ok(val2.log(val1)),
        "^" => Ok(val1.powf(val2)),
        &_ => Err(NotAnOperationError::InvalidSign),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Supported operations:
    value + value
    value - value
    value * value
    value / value
    base log exponent
    base ^ power"
    );

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    let input_vec: Vec<&str> = user_input.split_whitespace().collect();

    if input_vec.len() == 3 {
        println!("{}", eval_calc(input_vec)?);
    } else {
        println!("Provide a valid input.");
    }

    Ok(())
}
