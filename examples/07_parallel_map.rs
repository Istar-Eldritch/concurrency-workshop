// We are mapping over a vector, multiplying each of its values by two.
// Can you do it in parallel?
// TIP: You can share the array with 2 - 4 threads and map portions of it in each thread.

fn main() {
    // Initialization of vector
    let vec_len =  100;
    let mut v = vec![];
    for i in 0..=vec_len {
        v.push(i);
    }
    // End of initialization of vector

    for i in 0..=vec_len {
        v[i] *= 2;
    }

    dbg!(v);
}