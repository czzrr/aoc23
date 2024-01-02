#[derive(Debug)]
struct Map {
    rows: Vec<Vec<char>>,
}

impl Map {
    // Find neighbors of pipe at (i, j).
    pub fn connected_pipes(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let i = i as i32;
        let j = j as i32;
        let pipe_from = self.get(i, j).unwrap();
        let mut pipes = Vec::new();
        for (ii, jj, pipe_syms_from, pipe_syms_to) in [
            (i - 1, j, ['S', 'J', 'L', '|'], ['S', '7', 'F', '|']),
            (i, j + 1, ['S', 'L', 'F', '-'], ['S', 'J', '7', '-']),
            (i + 1, j, ['S', 'F', '|', '7'], ['S', 'J', 'L', '|']),
            (i, j - 1, ['S', 'J', '7', '-'], ['S', 'L', 'F', '-']),
        ] {
            match self.get(ii, jj) {
                Some(pipe_to)
                    if pipe_syms_from.contains(&pipe_from) && pipe_syms_to.contains(&pipe_to) =>
                {
                    pipes.push((ii as usize, jj as usize))
                }
                _ => (),
            }
        }
        pipes
    }

    // Get pipe symbol at (i, j).
    pub fn get(&self, i: i32, j: i32) -> Option<char> {
        if 0 <= i && i < self.rows.len() as i32 && 0 <= j && j < self.rows[0].len() as i32 {
            Some(self.rows[i as usize][j as usize])
        } else {
            None
        }
    }
}

fn main() {
    let lines = io::read_lines("day10/input");
    // Construct map and determine starting position
    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut start_pos = None;
    for (i, line) in lines.iter().enumerate() {
        let row: Vec<_> = line.chars().collect();
        for j in 0..row.len() {
            if row[j] == 'S' {
                start_pos = Some((i, j));
            }
        }
        rows.push(line.chars().collect());
    }
    let start_pos = start_pos.unwrap();
    let map = Map { rows };
    dbg!(&start_pos);
    dbg!(&map);

    let mut path = Vec::new();
    path.push(start_pos);
    // Pick one of the two neighbors of the starting point; it's guaranteed to have exactly two neighbors
    let mut current_pos = map.connected_pipes(start_pos.0, start_pos.1)[0];
    while current_pos != start_pos {
        path.push(current_pos);
        let (i, j) = current_pos;
        let nbs = map.connected_pipes(i, j);
        dbg!(&nbs); 
        // Find next neighbor not visited.
        // If both neighbors are visited, we must have arrived at S.
        match nbs.iter().find(|nb| !path.contains(nb)) {
            Some(nb) => current_pos = *nb,
            _ => current_pos = start_pos,
        }
    }
    dbg!(&path);
    println!("{}", path.len() / 2);
}

// enum Pipe {
//     VerticalPipe,
//     HorizontalPipe,
//     NorthEastPipe,
//     NorthWestPipe,
//     SouthWestPipe,
//     SouthEastPipe,
//     StartingPosition
// }
