impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {


        let  n = arr.len()

        for i in 0..n{

            for j in i+1..n {

                if arr[i] < arr[j]  
                
                {
                    arr[i] == arr[i+1]; 
                } else {
                    arr[i] = arr[i];
                }
            }
    let temp = arr[i];

        }




    }
}
