// Create a function in rust that calculates the average price of the items in the list. the list is passed as a parameter to the function.

fn average_price(prices: &[f64]) -> f64 {
    // This function takes a slice of f64 values (prices) as input and returns an f64 value (the average price).
    let sum: f64 = prices.iter().sum();
    // It calculates the sum of all the prices in the list using the iterator's sum method.
    sum / prices.len() as f64
    // It then divides the sum by the number of items in the list (converted to f64) to get the average price.
}

fn control_flow(){
    println!("Control Flow");
    let proceed = true;
    if proceed{
        println!("Proceeding...");
    } else {
        println!("Not proceeding...");
    }
    let height = 190;
    if height < 150 {
        println!("Short");
    } else if height < 180 {
        println!("Average");
    } else {
        println!("Tall");
    }
}

fn fn_some(){
    let mut maybe_number = Some(42); // or None in some cases
    if let Some(num) = maybe_number {
        println!("The number is {}", num);
    } else {
        println!("No number provided.");
    }

    maybe_number = None; // or None in some cases
    if let Some(num) = maybe_number {
        println!("The number is {}", num);
    } else {
        println!("No number provided.");
    }    
}

fn loop_fn(){
    let mut counter = 0;
    loop {
        println!("Counter: {}", counter);
        counter += 1;
        if counter >= 10 {
            break;
        }
    }
}


fn main() {
    let prices = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    // In the main function, a vector of prices is created.
    let avg = average_price(&prices);
    // The average_price function is called with a reference to the prices vector.
    println!("Average price: {}", avg);
    // The average price is printed to the console.
    let mut height = 100.0;
    println!("Height: {}", height);
    height = 2000.0;
    println!("Height: {}", height);

    control_flow();
    fn_some();
    loop_fn();
    

}
//write tests for all functions in the main.rs file.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_price() {
        let prices = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        assert_eq!(average_price(&prices), 30.0);
    }

    #[test]
    fn test_control_flow() {
        control_flow();
    }

    #[test]
    fn test_fn_some() {
        fn_some();
    }
    // write test for fn loop_fn
    #[test]
    fn test_loop_fn() {
        loop_fn();
    }
}