fn main() {
    let pi: f64 = pi(4);
    println!("{pi}");
}

fn pi(n: i32) -> f64 {
    let mut pi: f64 = 0.0;
    let precision: f64 = f64::powi(10., -n);

    let mut count: i32 = 0;

    loop {

        /* Calculatincg row member and adding to sum */
        let series_member = 8. / (4.0 * count as f64 + 1.0) / (4.0 * count as f64 + 3.0);
        pi += series_member;

        /* Calculating remainder */
        let remainder: f64 = 4. / (count + 1) as f64; 
        
        if remainder < precision  { break };
        count += 1;
 
    }

    f64_floor(pi, n as u32)

}


fn f64_floor(x: f64, n: u32) -> f64{
    let y = 10i32.pow(n) as f64;
    (x * y).floor()/y
}