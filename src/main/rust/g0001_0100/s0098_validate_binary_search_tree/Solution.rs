// #Medium #String #Dynamic_Programming #2024_09_08_Time_1_ms_(77.46%)_Space_2.2_MB_(26.76%)

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let mut state = vec![vec![None::<bool>; s2.len() + 1]; s1.len() + 1];
        is_interleave_0(&s1, &s2, &s3, &mut state)
    }
}

fn is_interleave_0(s1: &str, s2: &str, s3: &str, state: &mut Vec<Vec<Option<bool>>>) -> bool {
    if let Some(result) = state[s1.len()][s2.len()] {
        return result;
    }

    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    if s1.is_empty() {
        return s2 == s3;
    }

    if s2.is_empty() {
        return s1 == s3;
    }

    let (head1, tail1) = s1.split_at(1);
    let (head2, tail2) = s2.split_at(1);
    let (head3, tail3) = s3.split_at(1);

    if head1 == head3 && is_interleave_0(tail1, s2, tail3, state) {
        state[s1.len()][s2.len()] = Some(true);
        return true;
    } else if head2 == head3 && is_interleave_0(s1, tail2, tail3, state) {
        state[s1.len()][s2.len()] = Some(true);
        return true;
    } else {
        state[s1.len()][s2.len()] = Some(false);
        return false;
    }
}
