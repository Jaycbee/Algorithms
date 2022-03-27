
fn partition(arr : & mut [i8], low : usize, high : usize) -> usize { 
    
    let pivot = arr[high]; // pick last element
    let mut last_index : usize = low - 1; //last index since <= pivot

    for curr_index in low..high { //order elements as [low < pivot][low > pivot][pivot]
        
        if arr[curr_index] <= pivot {

            last_index += 1; // increment
            arr.swap(curr_index, last_index);
        }

    }

    arr.swap(high, last_index + 1); //rearrange as [low < pivot][pivot][low > pivot]
    return last_index + 1 //pivot position after sorting  & rearrangement 

}


fn Quicksort(arr : & mut [i8], low : usize, high : usize) -> () {
    
    if (low  < high) && (arr.len() > 1)  { // constraints

        let index = partition(arr, low, high); 


        Quicksort(arr, low, index - 1); // work on left portion
        Quicksort(arr, index + 1, high); // work on right portion
       

   }

}


fn main() -> () {
    let mut array = [1,2,100,4,8,3];
    let start = 1;
    let end : usize = array.len() -1;

    Quicksort(& mut array, start, end);

    println!("Sorted array (low-high) : {:#?}", array);
}
