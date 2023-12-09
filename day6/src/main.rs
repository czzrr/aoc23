fn main() {
    let lines = io::read_lines("day6/input");
    // let times: Vec<_> = lines[0]
    //     .split_once(':')
    //     .unwrap()
    //     .1
    //     .split_ascii_whitespace()
    //     .map(|s| s.parse().unwrap())
    //     .collect();
    // let distances: Vec<_> = lines[1]
    //     .split_once(':')
    //     .unwrap()
    //     .1
    //     .split_ascii_whitespace()
    //     .map(|s| s.parse().unwrap())
    //     .collect();

    // let time_distance_pairs: Vec<(usize, usize)> = zip(times, distances).collect();
    // let mut result = 1;
    // for (t, d) in time_distance_pairs {
    //     let mut count = 0;
    //     for s in 1..t {
    //         if s * (t - s) > d {
    //             count += 1;
    //         }
    //     }
    //     dbg!(count);
    //     result *= count;
    // }
    // dbg!(result);
    let time: usize = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();
    let distance: usize = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();
    dbg!(time);
    dbg!(distance);

    let mut count = 0;
    for speed in 1..time {
        if speed * (time - speed) > distance {
            count += 1;
        }
    }
    dbg!(count);

    // -s^2 + t*s - d = 0
    // disc = t^2 - 4*(-1)*(-d) = t^2 - 4*d
    // root1 = (-t + sqrt(disc)) / (2a)
    // root2 = (-t - sqrt(disc)) / (2a)
    let disc = (time.pow(2) - 4 * distance) as f32;
    let root1 = (-(time as f32) + disc.sqrt()) / (-2.0);
    let root2 = (-(time as f32) - disc.sqrt()) / (-2.0);
    dbg!(root1);
    dbg!(root2);
    let start = if root1 == root1.ceil() {
        root1 + 1.0
    } else {
        root1.ceil()
    } as usize;
    let end = root2.floor() as usize;
    dbg!(start);
    dbg!(end);
    let answer = end - start + 1;
    dbg!(answer);
}
