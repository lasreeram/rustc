// Create a function in rust that calculates the average price of the items in the list. the list is passed as a parameter to the function.

fn average_price(prices: &[f64]) -> f64 {
    // This function takes a slice of f64 values (prices) as input and returns an f64 value (the average price).
    let sum: f64 = prices.iter().sum();
    // It calculates the sum of all the prices in the list using the iterator's sum method.
    sum / prices.len() as f64
    // It then divides the sum by the number of items in the list (converted to f64) to get the average price.
}

fn main() {
    let prices = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    // In the main function, a vector of prices is created.
    let avg = average_price(&prices);
    // The average_price function is called with a reference to the prices vector.
    println!("Average price: {}", avg);
    // The average price is printed to the console.
}
