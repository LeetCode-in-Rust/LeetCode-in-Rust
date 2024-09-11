// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Depth_First_Search
// #Breadth_First_Search #Graph #Topological_Sort #Big_O_Time_O(N)_Space_O(N)
// #2024_09_10_Time_0_ms_(100.00%)_Space_2.6_MB_(47.48%)

impl Solution {
    const WHITE: i32 = 0;
    const GRAY: i32 = 1;
    const BLACK: i32 = 2;

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut adj: Vec<Vec<i32>> = vec![vec![]; num_courses];
        let mut colors: Vec<i32> = vec![Self::WHITE; num_courses];

        for pre in prerequisites.iter() {
            adj[pre[1] as usize].push(pre[0]);
        }

        for i in 0..num_courses {
            if colors[i] == Self::WHITE && !adj[i].is_empty() {
                if Self::has_cycle(&adj, i, &mut colors) {
                    return false;
                }
            }
        }

        true
    }

    fn has_cycle(adj: &Vec<Vec<i32>>, node: usize, colors: &mut Vec<i32>) -> bool {
        colors[node] = Self::GRAY;

        for &neighbor in adj[node].iter() {
            let neighbor = neighbor as usize;
            if colors[neighbor] == Self::GRAY {
                return true;
            }
            if colors[neighbor] == Self::WHITE {
                if Self::has_cycle(adj, neighbor, colors) {
                    return true;
                }
            }
        }

        colors[node] = Self::BLACK;
        false
    }
}
