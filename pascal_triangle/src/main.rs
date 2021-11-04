
// calculating factorial via a linearly iterative method
fn factorial(n: i32) -> i32{
    facto_iteration(1, 1, n)
}

// the linear iterative method of factorial calculation
fn facto_iteration(product: i32, counter: i32, max_count: i32) -> i32{
    if(counter > max_count){
        product
    }
    else{
        facto_iteration(product * counter, counter++, max_count)
    }
}

// calculating binomial coefficient
fn binom_coeff(n: i32, k: i32) -> i32{
    factorial(n)/(factorial(k)*factorial(n-k))
}

fn pascal_triangle(n: i32){
    
}
fn main() {
    println!("Hello, world!");
}
