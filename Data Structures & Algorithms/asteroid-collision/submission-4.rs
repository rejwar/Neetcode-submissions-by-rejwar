impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        for asteroid in asteroids {
            let mut survived = true;
            
            while let Some(&top) = stack.last() {
                // Collision only happens if top is moving Right (+) and current is moving Left (-)
                if top > 0 && asteroid < 0 {
                    if top < -asteroid {
                        stack.pop();
                        continue; // Check the current asteroid against the new top
                    } else if top == -asteroid {
                        stack.pop(); // Both explode
                    }
                    survived = false; // The current asteroid explodes (either by tie or being smaller)
                    break;
                }
                break; // No collision condition
            }

            if survived {
                stack.push(asteroid);
            }
        }

        stack
    }
}