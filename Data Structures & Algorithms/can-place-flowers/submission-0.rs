impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let len = flowerbed.len();
        
        for i in 0..len {
            
            if flowerbed[i] == 0 {
                
                let empty_left = (i == 0) || (flowerbed[i - 1] == 0);
                
                let empty_right = (i == len - 1) || (flowerbed[i + 1] == 0);

                if empty_left && empty_right {
                    flowerbed[i] = 1; 
                    n -= 1;           
                    
                    if n <= 0 { return true; } 
                }
            }
        }
        
        n <= 0
    }
}