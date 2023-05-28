use std::{error, io};
use std::thread::park;

fn eval_calc(vec: Vec<&str>) -> f64 {

    if let Err(_e) = vec.get(0).unwrap().parse::<f64>() {

        return 0.0

    }
    else if let Err(_e) = vec.get(2).unwrap().parse::<f64>() {

        return 0.0

    }

    let val1 = vec.get(0).unwrap().parse::<f64>().unwrap();
    let val2 = vec.get(2).unwrap().parse::<f64>().unwrap();

    match *vec.get(1).unwrap() {

        "+" => val1 + val2,
        "-" => val1 - val2,
        "*" => val1 * val2,
        "/" => val1 / val2,
        "log" => val2.log(val1),
        "^" => val1.powf(val2),
        &_ => 0.0

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

    if input_vec.len() == 3 && eval_calc(input_vec.clone()) != 0.0 {

        println!("{}", eval_calc(input_vec));

    }
    else {

        println!("Provide a valid input.");

    }

    park();
    Ok(())

}
