fn main() {
    let dist1: Vec<i32> = vec![1, 3, 4];
    let speed1: Vec<i32> = vec![1, 1, 1];
    let dist2: Vec<i32> = vec![3, 3, 5, 7, 7];
    let speed2: Vec<i32> = vec![1, 1, 4, 2, 2];
    let dist3: Vec<i32> = vec![3, 4, 8, 8, 3, 6, 8];
    let speed3: Vec<i32> = vec![1, 1, 1, 2, 1, 1, 2];
    println!("{}", eliminate_maximum(dist1, speed1));
    println!("{}", eliminate_maximum(dist2, speed2));
    println!("{}", eliminate_maximum(dist3, speed3));
}

fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut sort: Vec<i32> = (0..dist.len())
        .map(|i| (dist[i] + speed[i] - 1) / speed[i])
        .collect();
    sort.sort_unstable();
    let iter = sort
        .iter()
        .enumerate()
        .map(|(i, v)| (i, v - i as i32))
        .skip(1);

    for (i, v) in iter {
        if v <= 0 {
            return i as i32;
        }
    }

    dist.len() as i32
}

/* Unefficient :(

fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut dist: Vec<i32> = dist.clone();
    let mut speed: Vec<i32> = speed.clone();
    let mut eliminated: i32 = 0;
    loop {
        let mut fastest: f32 = if dist.len() != 0 {
            dist[0] as f32 / speed[0] as f32
        } else {
            0.0
        };
        let mut fastest_idx: usize = 0;
        for i in 0..dist.len() {
            if dist[i] <= 0 {
                return eliminated;
            }
            let monster_speed = dist[i] as f32 / speed[i] as f32;
            if monster_speed < fastest {
                fastest = monster_speed;
                fastest_idx = i;
            }
            dist[i] = dist[i] - speed[i];
        }
        if dist.len() == 0 {
            return eliminated;
        }
        eliminated += 1;
        dist.swap_remove(fastest_idx);
        speed.swap_remove(fastest_idx);
    }
}

*/
