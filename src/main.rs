fn main() {
    const discount_rate:f64=0.10;
    let item_price:f64=200.0;
    let quantity:i32=4;
    let mut total:f64=item_price*quantity as f64;
    let total:f64=total*discount_rate;
    println!("Price is:{}",total);
}
