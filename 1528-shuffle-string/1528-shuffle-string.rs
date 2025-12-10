impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut y = vec![' '; s.len()];
        let mut i = 0;
        while i < indices.len() {
            let c = s.chars().nth(i).unwrap();
            y[indices[i] as usize] = c;
            i+=1;
        }
        y.into_iter().collect()
    }
}