fn main() {
    let lines = io::read_lines("day3/input");
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }

    // Part 1
    // let mut sum = 0;
    // let rows = map.len();
    // let columns = map[0].len();
    // for i in 0..rows {
    //     for j in 0..columns {
    //         if !map[i][j].is_ascii_digit() && map[i][j] != '.' {
    //             for di in -1..=1 {
    //                 let mut visited = Vec::new();
    //                 for dj in -1..=1 {
    //                     let ii = i as i32 + di;
    //                     let jj = j as i32 + dj;
    //                     if (0..rows).contains(&(ii as usize))
    //                         && (0..columns).contains(&(jj as usize))
    //                     {
    //                         if map[ii as usize][jj as usize].is_ascii_digit()
    //                             && !visited.contains(&jj)
    //                         {
    //                             let mut jjj = jj as usize;
    //                             while jjj > 0 && map[ii as usize][jjj - 1].is_ascii_digit() {
    //                                 jjj -= 1;
    //                             }
    //                             let s: String = map[ii as usize][jjj..]
    //                                 .iter()
    //                                 .take_while(|c| c.is_ascii_digit())
    //                                 .collect();
    //                             let n: usize = s.parse().unwrap();
    //                             dbg!(n);
    //                             sum += n;
    //                             for idx in 0..s.len() {
    //                                 visited.push((jjj + idx) as i32);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // println!("{}", sum);

    // Part 2
    let mut sum = 0;
    let rows = map.len();
    let columns = map[0].len();
    for i in 0..rows {
        for j in 0..columns {
            if map[i][j] == '*' {
                let mut parts = Vec::new();
                for di in -1..=1 {
                    let mut visited = Vec::new();
                    for dj in -1..=1 {
                        let ii = i as i32 + di;
                        let jj = j as i32 + dj;
                        if (0..rows).contains(&(ii as usize))
                            && (0..columns).contains(&(jj as usize))
                        {
                            if map[ii as usize][jj as usize].is_ascii_digit()
                                && !visited.contains(&jj)
                            {
                                let mut jjj = jj as usize;
                                while jjj > 0 && map[ii as usize][jjj - 1].is_ascii_digit() {
                                    jjj -= 1;
                                }
                                let s: String = map[ii as usize][jjj..]
                                    .iter()
                                    .take_while(|c| c.is_ascii_digit())
                                    .collect();
                                let n: usize = s.parse().unwrap();
                                parts.push(n);
                                for idx in 0..s.len() {
                                    visited.push((jjj + idx) as i32);
                                }
                            }
                        }
                    }
                }
                if parts.len() == 2 {
                    sum += parts.iter().product::<usize>();
                }
            }
        }
    }
    println!("{}", sum);
}
