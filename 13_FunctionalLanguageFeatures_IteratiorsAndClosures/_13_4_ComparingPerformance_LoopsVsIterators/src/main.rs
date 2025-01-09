 //   Closures and iterators are Rust features inspired by functional programming language ideas. 
 //   They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. 
 //   The implementations of closures and iterators are such that runtime performance is not affected. 
 //   This is part of Rust’s goal to strive to provide zero-cost abstractions.


fn main() {
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;
    
    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
                                     .zip(&buffer[i - 12..i])
                                     .map(|(&c, &s)| c * s as i64)
                                     .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }}
 