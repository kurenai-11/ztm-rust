// Topic: Arrays & Slices
//
// Requirements:
// * Print pairs of numbers and their sums as they are streamed from a data source
// * If only one number is received, then print "Unpaired value: V",
//   where V is the value
//
// Notes:
// * A simulated data stream is already configured in the code
// * See the stdlib docs for the "chunks" method on "slice" for more info

fn data() -> &'static [u64] {
    &[5, 5, 4, 4, 3, 3, 1]
}

fn main() {
    // `stream` is not an iterator of &[u64]
    let stream = data().chunks(2);
    for pair in stream {
        match pair {
            [a, b] => println!("Pair {a}, {b}, sum {}", a + b),
            [a] => println!("Unpaired value: {a}"),
            [..] => unreachable!("chunk size should be 2"),
        }
    }
}
