use std::sync::mpsc;
use std::thread;
use crate::challenges_compute::challenge::Challenge;
use crate::messages::input::challenges::nonogram_input::NonogramSolverInput;
use crate::messages::output::challenges::nonogram_output::NonogramSolverOutput;

pub struct Nonogram {
    input: NonogramSolverInput
}

impl Nonogram {

    fn create_solution(line: &Vec<Vec<u32>>, nb: usize) -> Vec<Vec<Vec<bool>>> {

        let mut r: Vec<Vec<Vec<bool>>> = Vec::new();

        for i in line {
            let groups = i.len();
            let mut sum = 0;
            let mut nb_empty = nb;
            for j in i {
                sum += *j;
            }
            if groups > 0 {
                nb_empty = 1 + nb - sum as usize - groups;
            }
            r.push(Nonogram::_create_solution(nb_empty, groups, i));
        }

        r
    }

    fn _create_solution(nb_empty: usize, groups: usize, line: &Vec<u32>) -> Vec<Vec<bool>> {

        let mut r: Vec<Vec<bool>> = Vec::new();
        let combi: Vec<Vec<bool>> = Nonogram::_create_combi(nb_empty + groups, groups);
        let mut v: Vec<bool>;
        let mut index: usize;

        for p in combi {
            v = Vec::new();
            index = 0;

            for i in 0..p.len() {
                if p[i] {
                    for _ in 0..(*line)[index] {
                        v.push(true);
                    }
                    if index < (*line).len() - 1 {
                        v.push(false);
                    }
                    index += 1;
                } else {
                    v.push(false);
                }
            }

            r.push(v);
        }

        r
    }

    fn _create_combi(range: usize, nb: usize) -> Vec<Vec<bool>> {

        let mut r: Vec<Vec<bool>> = Vec::new();
        let mut a_range: Vec<bool> = Vec::new();

        for _ in 0..range {
            a_range.push(false);
        }

        r = Nonogram::_combi(a_range, nb, 0, 0, r.clone());

        r
    }

    fn _combi(mut array: Vec<bool>, nb: usize, index: usize, cnt: usize, mut r: Vec<Vec<bool>>) -> Vec<Vec<bool>> {

        if cnt == nb {
            r.push(array);
            return r;
        }

        if index - cnt > array.len() - nb {
            return r;
        }

        let r = Nonogram::_combi(array.clone(), nb, index + 1, cnt, r.clone());

        array[index] = true;
        let r = Nonogram::_combi(array.clone(), nb, index + 1, cnt + 1, r.clone());

        r
    }

    fn _solve(s: Vec<u32>, cols: Vec<Vec<Vec<bool>>>, rows: Vec<Vec<u32>>, index: usize) -> Vec<Vec<bool>> {

        let mut grid: Vec<Vec<bool>> = Vec::new();

        for i in 0..cols.len() {
            let a = s[i] as usize;
            grid.push(cols[i][a].clone());
        }

        let verif = Nonogram::_verify(grid.clone(), rows.clone());

        if verif == -1 {
            return grid;
        }

        if index == cols.len() || verif < index as isize {
            return Vec::new();
        }

        let (tx, rx) = mpsc::channel();

        for i in 0..cols[index].len() {
            let tx1 = tx.clone();
            let mut s_t = s.clone();
            let cols_t = cols.clone();
            let rows_t = rows.clone();
            thread::spawn(move || {
                s_t[index] = i as u32;
                let grid_t = Nonogram::_solve(s_t.clone(), cols_t.clone(), rows_t.clone(), index + 1);
                tx1.send(grid_t).unwrap();
            });
        }

        for received in rx {
            if received.len() != 0 {
                return received;
            }
        }

        Vec::new()
    }

    fn _verify(grid: Vec<Vec<bool>>, rows: Vec<Vec<u32>>) -> isize {

        let mut index: usize;
        let mut cnt: u32;

        for j in 0..rows.len() {
            index = 0;
            cnt = 0;

            for i in 0..grid.len() {
                if grid[i][j] {
                    if index == rows[j].len() {
                        return i as isize;
                    }
                    cnt += 1;
                    continue;
                }
                if grid[i][j] == false && cnt != 0 {
                    if cnt != rows[j][index] {
                        return i as isize;
                    }
                    cnt = 0;
                    index += 1;
                }
            }
            if cnt != 0 && cnt != rows[j][index] {
                return grid.len() as isize;
            }
        }
        -1
    }

    fn _solve_rows(s: &mut Vec<u32>, rows: &Vec<Vec<Vec<bool>>>, cols: &Vec<Vec<u32>>, index: usize) -> Vec<Vec<bool>> {

        let verify = Nonogram::_verify_rows(s, rows, cols);
        let mut grid = Vec::new();

        if verify == -1 {
            let mut v;
            for i in 0..(*cols).len() {
                v = Vec::new();
                for j in 0..(*rows).len() {
                    v.push((*rows)[j][(*s)[j] as usize][i]);
                }
                grid.push(v);
            }
            return grid;
        }

        if index == (*rows).len() || verify < index as isize {
            return grid;
        }

        for i in 0..(*rows)[index].len() {
            (*s)[index] = i as u32;
            grid = Nonogram::_solve_rows(s, rows, cols, index + 1);
            if grid.len() != 0 {
                return grid;
            }
        }

        grid
    }

    fn _solve_rows_thearded(s: Vec<u32>, rows: &Vec<Vec<Vec<bool>>>, cols: &Vec<Vec<u32>>, index: usize) -> Vec<Vec<bool>> {

        let verify = Nonogram::_verify_rows(&s, rows, cols);
        let mut grid = Vec::new();

        if verify == -1 {
            let mut v;
            for i in 0..(*cols).len() {
                v = Vec::new();
                for j in 0..(*rows).len() {
                    v.push((*rows)[j][s[j] as usize][i]);
                }
                grid.push(v);
            }

            return grid;
        }

        if index == (*rows).len() || verify < index as isize {
            return grid;
        }

        let (tx, rx) = mpsc::channel();

        for i in 0..(*rows)[index].len() {
            let tx1 = tx.clone();
            let rows_t = rows.clone();
            let cols_t = cols.clone();
            let mut s_t = s.clone();
            thread::spawn(move || {
                s_t[index] = i as u32;
                tx1.send(Nonogram::_solve_rows(&mut s_t, &rows_t, &cols_t, index + 1)).unwrap();
            });
        }

        for _ in 0..rows[index].len() {
            match rx.recv() {
                Ok(data) => {
                    if data.len() != 0 {
                        return data;
                    }
                },
                Err(e) => println!("Une erreur s'est produite : {:?}", e)
            };
        }

        /*for received in rx {
            if received.len() != 0 {
                return received;
            }
        }*/

        grid
    }

    fn _verify_rows(s: &Vec<u32>, rows: &Vec<Vec<Vec<bool>>>, cols: &Vec<Vec<u32>>) -> isize {

        let mut index: usize;
        let mut cnt: u32;

        for i in 0..(*cols).len() {

            index = 0;
            cnt = 0;

            for j in 0..(*rows).len() {
                if (*rows)[j][(*s)[j] as usize][i] {
                    if index == (*cols)[i].len() {
                        return j as isize;
                    }
                    cnt += 1;
                    continue;
                }
                if (*rows)[j][s[j] as usize][i] == false && cnt != 0 {
                    if cnt != (*cols)[i][index] {
                        return j as isize;
                    }
                    cnt = 0;
                    index += 1;
                }
            }

            if cnt != 0 && cnt != (*cols)[i][index] {
                return rows.len() as isize;
            }
        }
        -1
    }

    fn _vec_to_string(grid: Vec<Vec<bool>>) -> String {

        let mut s: String = String::new();

        if grid.len() == 0 {
            return s;
        }

        for j in 0..grid[0].len() {
            for i in 0..grid.len() {
                if grid[i][j] {
                    s.push('#');
                } else {
                    s.push(' ');
                }
            }
            s.push('\n');
        }

        s
    }
}

impl Challenge for Nonogram {

    type Input = NonogramSolverInput;
    type Output = NonogramSolverOutput;

    fn name () -> String {
        "Nonogram".to_string()
    }

    fn new(input: Self::Input) -> Self {
        Nonogram {input}
    }

    fn solve(&self) -> Self::Output {

        let s_rows = Nonogram::create_solution(&self.input.rows, self.input.cols.len());

        let mut a: Vec<u32> = Vec::new();

        for _ in 0..self.input.rows.len() {
            a.push(0);
        }

        let answer = Nonogram::_solve_rows_thearded(a, &s_rows, &self.input.cols, 0);
        return NonogramSolverOutput {
            grid: Nonogram::_vec_to_string(answer)
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}
