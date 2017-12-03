use std::collections::HashMap;

fn find_nearest_corner(input: i32, corner: i32, side: i32) -> i32 {
    let mut current_corner;
    for i in 0..4 {
        current_corner = corner - (i * (side - 1));
        let distance = ((current_corner - input) as i32).abs();
        if distance <= ((side / 2) as f32).ceil() as i32 {
            return current_corner;
        }
    }
    unreachable!();
}

fn problem1(input: i32) -> i32 {
    let mut side = (input as f32).sqrt().ceil() as i32;
    if side % 2 == 0 {
        side += 1;
    }

    let corner = side * side;
    let nearest_corner = find_nearest_corner(input, corner, side);
    let distance = ((nearest_corner - input) as i32).abs() as i32;
    side - distance - 1
}

fn get_value(map: &HashMap<(i32, i32), i32>, x: i32, y: i32) -> i32 {

    if x == 0 && y == 0 {
        return 1;
    }

    let mut acc = 0;

    acc += map.get(&(x + 1, y)).unwrap_or(&0);
    acc += map.get(&(x + 1, y + 1)).unwrap_or(&0);
    acc += map.get(&(x, y + 1)).unwrap_or(&0);
    acc += map.get(&(x - 1, y + 1)).unwrap_or(&0);
    acc += map.get(&(x - 1, y)).unwrap_or(&0);
    acc += map.get(&(x - 1, y - 1)).unwrap_or(&0);
    acc += map.get(&(x, y - 1)).unwrap_or(&0);
    acc += map.get(&(x + 1, y - 1)).unwrap_or(&0);
    println!("value: {:?} {}", (x, y), acc);
    acc
}

fn problem2(input: i32) -> i32 {
    let mut matrix: HashMap<(i32, i32), i32> = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;
    let mut max_x = -1;
    let mut max_y = -1;
    matrix.insert((0, 0), 1);

    loop {
        max_x += 2;
        max_y += 2;

        for _ in 0..max_x * max_x {
            if (-max_x / 2 <= x && x <= max_x / 2) && (-max_y / 2 <= y && y <= max_y / 2) {
                let value = get_value(&matrix, x, y);
                matrix.insert((x, y), value);
                if matrix[&(x, y)] > input {
                    return matrix[&(x, y)];
                }
                if x == y || ((x < 0) && (x == -y)) || ((x > 0) && (x == 1 - y)) {
                    let tmp = dx;
                    dx = -dy;
                    dy = tmp;
                }
                x += dx;
                y += dy;
            }
        }
    }
}

const INPUT: &[i32] = &[1, 12, 23, 1024, 277678];

fn main() {
    for value in INPUT {
        println!("{} steps: {}, reset: {}",
                 value,
                 problem1(*value),
                 problem2(*value as i32));
    }
}
