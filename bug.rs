fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 5;
    // This will panic because of an out of bounds index
    println!("Value at index {}: {}", index, vec[index]);
}