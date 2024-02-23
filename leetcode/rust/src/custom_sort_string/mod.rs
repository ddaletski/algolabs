struct Solution {}

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut precedence = [u8::MAX; 256];

        for (i, c) in order.chars().enumerate() {
            precedence[c as usize] = i as u8;
        }

        let mut vec_string: Vec<char> = s.chars().collect();

        vec_string.sort_by_key(|ch| precedence[*ch as usize]);

        vec_string.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn case_1() {
        let order = "cba";
        let s = "abcd";

        let sorted = super::Solution::custom_sort_string(order.into(), s.into());

        assert!(sorted == "cbad");
    }
}
