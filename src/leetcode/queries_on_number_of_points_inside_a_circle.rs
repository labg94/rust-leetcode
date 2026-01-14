struct Solution;



impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut response = Vec::with_capacity(queries.len());
        let points = points.iter().map(|p| (p[0], p[1])).collect::<Vec<_>>();        
        let queries = queries.iter().map(|p| (p[0], p[1], p[2]* p[2])).collect::<Vec<_>>();        
        
        for c in queries {
            
            let mut sum = 0;
            
            for p in &points {
                let distance = (c.0 - p.0) * (c.0 - p.0)  + (c.1 - p.1) * (c.1 - p.1);

                if distance <= c.2 {
                    sum += 1;
                }
            }
            response.push(sum);
        }
        
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case_1(){
        assert_eq!(Solution::count_points(vec![vec![1,3],vec![3,3],vec![5,3],vec![2,2]],vec![vec![2,3,1],vec![4,3,1],vec![1,1,2]]),vec![3,2,2])
    }    
    
    #[test]
    fn test_case_2(){
        assert_eq!(Solution::count_points(vec![vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![5,5]],vec![vec![1,2,2],vec![2,2,2],vec![4,3,2],vec![4,3,3]]),vec![2,3,2,4])
    }
}