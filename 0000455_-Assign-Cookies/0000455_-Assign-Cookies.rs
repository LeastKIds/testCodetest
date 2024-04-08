impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let mut count = 0;

        g.sort_by(|a, b| b.cmp(a));
        s.sort_by(|a, b| b.cmp(a));

        for child in g.iter() {
            let new_s = s.clone();
            let mut deleteIndex = None;

            for (index, value) in new_s.iter().enumerate() {
                if (child <= value) {
                    deleteIndex = Some(index);
                    count += 1;
                    break;
                }
            }

             if let Some(index) = deleteIndex {
                    s.remove(index);
            }
        }

