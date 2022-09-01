// Comparing Performance: Loops vs. Iterators
// https://doc.rust-lang.org/stable/book/ch13-04-performance.html





/*
We ran a benchmark by loading the entire contents of The Adventures of Sherlock Holmes by Sir Arthur Conan Doyle into a String and looking for the word the in the contents. Here are the results of the benchmark on the version of search using the for loop and the version using iterators:

test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)

*/


/*
We’ve declared the variables within this example but not given them any values; although this code doesn’t have much meaning outside of its context, it’s still a concise, real-world example of how Rust translates high-level ideas to low-level code.
*/
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
}

















