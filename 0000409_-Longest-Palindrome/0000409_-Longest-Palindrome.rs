impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        // 조건
        // 알파벳은 반드시 짝수개만 활용될 수 있음 ex) abccba
        // 홀수개인 경우 전체 길이는 반드시 홀수개에 가운데에는 홀수개가 들어가야만 함.
        // 가장 긴 값을 가져야만 함

        let mut alphabet = std::collections::HashMap::new();

        for c in s.chars() {
            *alphabet.entry(c).or_insert(0) += 1;
        }

        let mut length = 0;
        let mut bigOdd = 0;
        let mut bigOddS = None;

        for (key, value) in alphabet.iter() {
            if (*value % 2 != 0 && bigOdd < *value) {
                bigOdd = *value;
                bigOddS = Some(key);
            } 
        }

