fn bubble_sort(arr : &mut [i32]){
let mut permutation = true;
let len = arr.len();

while permutation{
    permutation = false;
    for i in 0..len - 1{
        if arr[i] > arr[i + 1]{
            arr.swap(i, i + 1);
            permutation = true;
            println!("permutation: {:?}", arr);
        }
    }
}}

fn main(){
    let mut arr = [215, 136, 327, 38, 9569, 12, 45555];
    println!("Unsorted array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
