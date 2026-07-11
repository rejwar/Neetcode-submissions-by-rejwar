impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        
        for a in asteroids {
            let mut alive = true;
            
            while alive && a < 0 && !stack.is_empty() && *stack.last().unwrap() > 0 {
                let top = *stack.last().unwrap();
                
                if top < -a {
                    stack.pop();
                } else if top == -a {
                    stack.pop();
                    alive = false;
                } else {
                    alive = false;
                }
            }
            
            if alive {
                stack.push(a);
            }
        }
        
        stack
    }
}