impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        //sorting the arrayz
        strs.sort();
        let first = strs[0].clone();
        let last = strs[strs.len() - 1].clone();

        let first_bytes = first.as_bytes();
        let last_bytes = last.as_bytes();
        let mut i = 0;

        while i < first_bytes.len() && i < last_bytes.len() {
            if first_bytes[i] != last_bytes[i] {
                break
            }
            i += 1;
        }
        first[..i].to_string()
    }
}