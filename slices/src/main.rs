fn main() {
    let arr = [1,2,3,4];
    let slice = &arr[1 .. 3];
    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]){
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!{"{} {}", slice[0], slice[1]};
}