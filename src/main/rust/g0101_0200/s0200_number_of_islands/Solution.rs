// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Depth_First_Search
// #Breadth_First_Search #Matrix #Union_Find
// #Algorithm_II_Day_6_Breadth_First_Search_Depth_First_Search
// #Graph_Theory_I_Day_1_Matrix_Related_Problems #Level_1_Day_9_Graph/BFS/DFS #Udemy_Graph
// #Big_O_Time_O(M*N)_Space_O(M*N) #2024_09_10_Time_7_ms_(86.79%)_Space_9.1_MB_(42.14%)

impl Solution {
	pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
		let mut num_of_islands: i32 = 0;
		let mut current_index: i32 = 1;
		let height = grid.len();
		let width = grid[0].len();
		let mut left: i32;
		let mut up: i32;
		let mut part_of_island = vec![0; width * height];
        let mut pos;
		for current in 0..height * width {
			let x = current / width;
			let y = current % width;
			if grid[x][y] == '1' {
                let left = if y > 0 {
                    part_of_island[current - 1]
                } else {
                    0
                };
                let up = if x > 0 {
                    part_of_island[current - width]
                } else {
                    0
                };
				if up > 1 {
					if left > 1 && left != up {
						
						for i in 0..width {
							pos = part_of_island[current - i];
							if pos == left {
								part_of_island[current - i] = up;
							}
						}
						num_of_islands -= 1;
					}
					part_of_island[current] = up;
					continue;
				} else if (left > 1) {
					part_of_island[current] = left;
					continue;
				}
				current_index += 1;
				num_of_islands += 1;
				part_of_island[current] = current_index;
			}
		}
		return num_of_islands;
	}
}
