// #Medium #String #2024_09_04_Time_0_ms_(100.00%)_Space_2.1_MB_(94.72%)

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 || s.len() < num_rows {
            return s;
        }
        
        let mut result = String::with_capacity(s.len());
        let s_bytes = s.as_bytes();
        let complete_jump = 2 * (num_rows - 1);

        for row in 0..num_rows {
            let mut index = row;
            let zig_jump = 2 * (num_rows - 1 - row);
            let zag_jump = complete_jump - zig_jump;
            let mut use_zig_jump = true;

            while index < s.len() {
                result.push(s_bytes[index] as char);
                if row == 0 || row == num_rows - 1  {
                    index += complete_jump;
                } else {
                    index += if use_zig_jump { zig_jump } else { zag_jump };
                    use_zig_jump = !use_zig_jump;
                }
            } 
        }

        result
    }
}
