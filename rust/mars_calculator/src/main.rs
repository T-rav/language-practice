use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Please enter your weight in lbs:");
    io::stdin().read_line(&mut input);
    let weight= convert_weight(input);
    let result= calculate_weight_on_mars(weight);

    println!("Your weigth on Earth of {:.2}lb is {:.2}lb on Mars!", weight, result);
}

fn calculate_weight_on_mars(weight:f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn convert_weight(weight:String) -> f32 {
    weight.trim().parse().unwrap()
}