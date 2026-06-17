fn stack_heavy() {
    let mut sum = 0;
    for i in 0..10_000_000 {
        sum += i; // Fast, pure stack math
    }
    std::hint::black_box(sum);
}

fn heap_heavy() {
    let mut vec = Vec::new();
    for i in 0..10_000_000 {
        vec.push(i.to_string()); // Slow, allocates Strings on the Heap
    }
    std::hint::black_box(vec);
}

fn main() {
    println!("Running Stack test...");
    stack_heavy();
    
    println!("Running Heap test...");
    heap_heavy();
}