use std::collections::{HashMap, VecDeque};
use std::str::Lines;
use crate::challenges_compute::challenge::Challenge;
use crate::messages::input::challenges::monstrous_maze_input::MonstrousMazeInput;
use crate::messages::output::challenges::monstrous_maze_output::MonstrousMazeOutput;

pub struct MonstrousMaze{
    input : MonstrousMazeInput
}

impl MonstrousMaze {
    fn bfs(maze: &mut [&mut [char]], start: (usize, usize), end: (usize, usize)) -> Vec<char> {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut parents: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        parents.insert(start, start);

        while let Some(current_node) = queue.pop_front() {
            let (x, y) = current_node;

            if current_node == end {
                let mut path = vec![];
                let mut current = current_node;
                while current != start {
                    let parent = parents[&current];
                    if parent.0 < current.0 {
                        path.push('v');
                    } else if parent.0 > current.0 {
                        path.push('^');
                    } else if parent.1 < current.1 {
                        path.push('>');
                    } else if parent.1 > current.1 {
                        path.push('<');
                    }
                    current = parent;
                }
                path.reverse();
                return path;
            }

            if x > 0 && ( maze[x - 1][y] == ' ' || maze[x - 1][y] == 'X'){
                queue.push_back((x - 1, y));
                parents.insert((x - 1, y), current_node);
                maze[x - 1][y] = '.';
            }

            if x < maze[0].len() - 1 && ( maze[x + 1][y] == ' ' || maze[x + 1][y] == 'X')  {
                queue.push_back((x + 1, y));
                parents.insert((x + 1, y), current_node);
                maze[x + 1][y] = '.';
            }

            if y > 0 && ( maze[x][y - 1] == ' ' || maze[x][y - 1] == 'X') {
                queue.push_back((x, y - 1));
                parents.insert((x, y - 1), current_node);
                maze[x][y - 1] = '.';
            }

            if y < maze[0].len() - 1 && ( maze[x][y + 1] == ' ' || maze[x][y + 1] == 'X') {
                queue.push_back((x, y + 1));
                parents.insert((x, y + 1), current_node);
                maze[x][y + 1] = '.';
            }
        }
        vec![]
    }
}


impl Challenge for MonstrousMaze {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        "MonstrousMaze".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MonstrousMaze {input}
    }

    fn solve(&self) -> Self::Output {
        let laby = self.input.grid.clone();
        let split : Lines = laby.lines();
        let cols: isize = split.clone().nth(0).expect("Not found").len() as isize;
        let rows: isize = split.clone().count() as isize;
        //let mut arrayLab = vec![[0 as u8 ; cols]; rows as usize];
        let mut array_lab_raw = vec!['0'; (cols * rows) as usize];

        // Vector of 'width' elements slices
        let mut array_lab_base: Vec<_> = array_lab_raw.as_mut_slice().chunks_mut(cols as usize).collect();

        // Final 2d array `&mut [&mut [_]]`
        let array_lab = array_lab_base.as_mut_slice();
        let mut count: usize = 0;
        let mut start_lab_x = 0;
        let mut start_lab_y = 0;
        let mut end_lab_x = 0;
        let mut end_lab_y = 0;
        for i in 0..rows {
            for j in 0..cols {
                if laby.chars().nth(count).expect("IDK") == '\n' {
                    count = count + 1;
                }
                array_lab[i as usize][j as usize] = laby.chars().nth(count).expect("IDK");
                //print!("{}", arrayLab[i as usize][j as usize]);
                count = count + 1;
                if array_lab[i as usize][j as usize] == 'I' {
                    start_lab_x = i;
                    start_lab_y = j;
                }
                if array_lab[i as usize][j as usize] == 'X' {
                    end_lab_x = i;
                    end_lab_y = j;
                }
            }
        }
        let result : Vec<char> = MonstrousMaze::bfs(array_lab, (start_lab_x as usize, start_lab_y as usize), (end_lab_x as usize, end_lab_y as usize));
        return MonstrousMazeOutput{
            path:result.into_iter().collect()
        };
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}
