fn bubble_sort(arr : & mut [i8]) {

    let mut pass : bool = false;

    for x in 0..(arr.len() - 1){

        if arr[x] > arr[x+1] {
            arr.swap(x, x+1);
            pass = true;
        }
    } 

    if pass {bubble_sort(arr)}
}


fn main() {

    let mut array : [i8;7] = [100,1,2,20,6,10,5];

    bubble_sort(& mut array);
    
    println!("sorted array (low-high) : {:#?}", array);
}
