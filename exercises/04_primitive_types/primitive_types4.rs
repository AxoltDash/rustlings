
fn main() {
    const N : usize = 10;
    const K : usize = 3; // <- ventana de 3 elementos
    let a : [i32 ; N]= [0,1,2,3,4,5,6,7,8,9];

    for i in 0..N-K+1 {
        let nice_slice = &a[i..i+K];
        
        //print
        for j in nice_slice {
            print!("{}",j);
            print!(" ");
        }
        println!();
    }
    println!("=========");
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        //

        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
