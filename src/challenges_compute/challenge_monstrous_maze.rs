use std::collections::{HashMap, VecDeque};
use std::str::Lines;
use crate::challenges_compute::challenge::Challenge;
use crate::messages::input::challenges::monstrous_maze_input::MonstrousMazeInput;
use crate::messages::output::challenges::monstrous_maze_output::MonstrousMazeOutput;
use std::sync::mpsc;
use std::thread;

pub struct MonstrousMaze{
    input: MonstrousMazeInput,
    start: (usize, usize),
    end: (usize, usize),
}

impl MonstrousMaze {
    fn bfs(maze: &mut [&mut [char]], start: (usize, usize), end: (usize, usize), endurance: u32) -> Vec<char> {
        let mut endurance = endurance;
        let mut queue = VecDeque::new();
        queue.push_back((start, endurance));
        let mut parents: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        parents.insert(start, start);
        maze[start.0][start.1] = '#';

        while let Some((current_node, endurance)) = queue.pop_front() {
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
                return path.into_iter().rev().collect();
            }

            if x > 0 && maze[x - 1][y] != '#' && maze[x - 1][y] != 'M' {
                queue.push_back(((x - 1, y), endurance));
                parents.insert((x - 1, y), current_node);
                maze[x - 1][y] = '#';
            }
            if x < maze.len() - 1 && maze[x + 1][y] != '#' && maze[x + 1][y] != 'M' {
                queue.push_back(((x + 1, y), endurance));
                parents.insert((x + 1, y), current_node);
                maze[x + 1][y] = '#';
            }
            if y > 0 && maze[x][y - 1] != '#' && maze[x][y - 1] != 'M' {
                queue.push_back(((x, y - 1), endurance));
                parents.insert((x, y - 1), current_node);
                maze[x][y - 1] = '#';
            }
            if y < maze.len() - 1 && maze[x][y + 1] != '#' && maze[x][y + 1] != 'M' {
                queue.push_back(((x, y + 1), endurance));
                parents.insert((x, y + 1), current_node);
                maze[x][y + 1] = '#';
            }
            if maze[x][y] == 'M' {
                let new_endurance = endurance - 1;
                if new_endurance > 0 {
                    queue.push_back(((x, y), new_endurance));
                    parents.insert((x, y), current_node);
                    maze[x][y] = '#';
                }
            }
        }
        return vec![];
    }

    fn bfs_threadable(&self, mut way: Vec<(usize, usize)>, endurance: u32, maze: &mut [&mut [char]], end: &(usize,usize)) -> Vec<(usize, usize)> {

        /*for i in 0..way.len() {
            print!("( {} {} )", way[i].0, way[i].1);
        }
        println!("\n");*/

        if endurance == 0 {
            return vec![];
        }
        if way[way.len() -1] == *end {
            return way;
        }
        let mut newWay;
        let mut sentWay;
        let (x, y) = way[way.len() - 1];
        if x > 0 && maze[x - 1][y] != '#' && !MonstrousMaze::inVec(&way,&(x - 1,y)){
            sentWay = way.clone();
            sentWay.push((x - 1, y));
            if maze[x - 1][y] != 'M' {
                newWay = self.bfs_threadable(sentWay,endurance,maze,end)
            }else{
                newWay = self.bfs_threadable(sentWay,endurance - 1,maze,end)
            }
            if newWay.len() != 0 {
                return newWay;
            }

        }
        if x < maze.len() - 1 && maze[x + 1][y] != '#' && !MonstrousMaze::inVec(&way,&(x + 1,y)){
            sentWay = way.clone();
            sentWay.push((x + 1, y));
            if maze[x + 1][y] != 'M' {
                newWay = self.bfs_threadable(sentWay,endurance,maze,end)
            }else{
                newWay = self.bfs_threadable(sentWay,endurance - 1,maze,end)
            }
            if newWay.len() != 0 {
                return newWay;
            }
        }
        if y > 0 && maze[x][y - 1] != '#' && !MonstrousMaze::inVec(&way,&(x,y - 1)){
            sentWay = way.clone();
            sentWay.push((x, y - 1));
            if(maze[x][y - 1] != 'M'){
                newWay = self.bfs_threadable(sentWay,endurance,maze,end)
            }else{
                newWay = self.bfs_threadable(sentWay,endurance - 1,maze,end)
            }
            if newWay.len() != 0 {
                return newWay;
            }
        }
        if y < maze.len() - 1 && maze[x][y + 1] != '#' && !MonstrousMaze::inVec(&way,&(x,y + 1)){
            sentWay = way.clone();
            sentWay.push((x, y + 1));
            if(maze[x][y + 1] != 'M'){
                newWay = self.bfs_threadable(sentWay,endurance,maze,end)
            }else{
                newWay = self.bfs_threadable(sentWay,endurance - 1,maze,end)
            }
            if newWay.len() != 0 {
                return newWay;
            }
        }
        return vec![];
    }

    fn inVec(vec: &Vec<(usize, usize)>, tuple: &(usize,usize)) -> bool {
        for i in 0..vec.len() {
            if vec[i] == *tuple {
                return true
            }
        }
        false
    }
}


impl Challenge for MonstrousMaze {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        "MonstrousMaze".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MonstrousMaze {
            input,
            start: (0,0),
            end: (0,0),
        }
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
                if array_lab[i as usize][j as usize] == 'Y' {
                    start_lab_x = i;
                    start_lab_y = j;
                }
                if array_lab[i as usize][j as usize] == 'X' {
                    end_lab_x = i;
                    end_lab_y = j;
                }
            }
        }
        /*for i in 0..rows {
            for j in 0..cols {
                print!("{}",array_lab[i as usize][j as usize]);
            }
            print!("\n");
        }*/

        let start = (start_lab_x as usize, start_lab_y as usize);
        let end = (end_lab_x as usize, end_lab_y as usize);
        println!("({} {})",end.0, end.1);
        let result : Vec<(usize, usize)> = self.bfs_threadable(vec![start],self.input.endurance, array_lab, &end);
        let mut parent = result[0];
        let mut path = vec![];
        for i in 1..result.len(){
            if parent.0 < result[i].0 {
                path.push('v');
            } else if parent.0 > result[i].0 {
                path.push('^');
            } else if parent.1 < result[i].1 {
                path.push('>');
            } else if parent.1 > result[i].1 {
                path.push('<');
            }
            parent = result[i];
        }

        return MonstrousMazeOutput{
            path:path.into_iter().collect()
        };
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}
