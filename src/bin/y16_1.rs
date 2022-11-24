use std::collections::HashSet;

const DIRECTIONS: &str = r#"R3, L5, R2, L1, L2, R5, L2, R2, L2, L2, L1, R2, L2, R4, R4, R1, L2, L3, R3, L1, R2, L2, L4, R4, R5, L3, R3, L3, L3, R4, R5, L3, R3, L5, L1, L2, R2, L1, R3, R1, L1, R187, L1, R2, R47, L5, L1, L2, R4, R3, L3, R3, R4, R1, R3, L1, L4, L1, R2, L1, R4, R5, L1, R77, L5, L4, R3, L2, R4, R5, R5, L2, L2, R2, R5, L2, R194, R5, L2, R4, L5, L4, L2, R5, L3, L2, L5, R5, R2, L3, R3, R1, L4, R2, L1, R5, L1, R5, L1, L1, R3, L1, R5, R2, R5, R5, L4, L5, L5, L5, R3, L2, L5, L4, R3, R1, R1, R4, L2, L4, R5, R5, R4, L2, L2, R5, R5, L5, L2, R4, R4, L4, R1, L3, R1, L1, L1, L1, L4, R5, R4, L4, L4, R5, R3, L2, L2, R3, R1, R4, L3, R1, L4, R3, L3, L2, R2, R2, R2, L1, L4, R3, R2, R2, L3, R2, L3, L2, R4, L2, R3, L4, R5, R4, R1, R5, R3"#;
fn main() {
    let mut position = [0i32; 2];
    // north is 0, east is 1, south is 2, west is 3.
    // we add 1 to rotation to go clockwise to the next direction
    // we sub 1 to rotation to go anti clockwise to the previous direction
    // finally, we just do rotation mod 4 to get the final direction.
    let mut rotation = 0u8;
    let mut positions_visited = HashSet::new();
    positions_visited.insert(position);
    let mut positions_visited_twice = None;
    for dir in DIRECTIONS.split(',') {
        let previous_position = position;

        let dir = dir.trim();
        // get direction L or R
        let direction = dir.chars().next().unwrap();
        // get count by parsing the remaining digits
        let block_count: i32 = dir[1..].parse().unwrap();
        // change the facing direction based on the character
        match direction {
            'R' => {
                if rotation == 3 {
                    rotation = 0;
                } else {
                    rotation += 1;
                }
            }
            'L' => {
                // wrap around instead of going negative
                if rotation == 0 {
                    // if north and turn left, we set to west
                    rotation = 3
                } else {
                    rotation -= 1;
                }
            }
            _ => unreachable!("only right or left turns allowed"),
        };
        // based on direction, add the units to the relevant axis
        match rotation {
            0 => {
                position[1] += block_count;
            }
            1 => {
                position[0] += block_count;
            }
            2 => {
                position[1] -= block_count;
            }
            3 => {
                position[0] -= block_count;
            }
            _ => unreachable!(),
        }

        // check if still haven't visited a position twice
        if positions_visited_twice.is_none() {
            // iterate over all the points we will travel through
            // remember that range in rust requires that start < end
            // if we use start > end, we will fail
            let x_min = previous_position[0].min(position[0]);
            let x_max = previous_position[0].max(position[0]);
            let y_min = previous_position[1].min(position[1]);
            let y_max = previous_position[1].max(position[1]);
            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    let current_position = [x, y];
                    // skip the first element as it will just be the previous position
                    if current_position == previous_position {
                        assert!(
                            positions_visited.contains(&current_position),
                            "position is previous position but still not found in set: {current_position:?}"
                        );
                        continue;
                    }
                    if positions_visited_twice.is_none()
                        && !positions_visited.insert(current_position)
                    {
                        positions_visited_twice = Some(current_position);
                    }
                }
            }
        }
    }
    let location_visited_twice = positions_visited_twice.unwrap();
    dbg!(
        position[0].abs() + position[1].abs(),
        location_visited_twice[0].abs() + location_visited_twice[1].abs(),
        location_visited_twice
    );
}
