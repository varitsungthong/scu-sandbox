fn main() {
    let a = [0;25000];
    let slice1 = &a[0..6767];
    let slice2 = &a[69..6836];
    assert_eq!(slice1, slice2);

    if slice1 == slice2 {
        println!("absolutely!");

    }
    else {
        println!("How do we get here?");
    }

}