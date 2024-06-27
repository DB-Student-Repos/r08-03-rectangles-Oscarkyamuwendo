pub fn count(lines: &[&str]) -> u32 {
    let height = lines.len();
    if height == 0 {
        return 0;
    }
    let width = lines[0].len();
    if width == 0 {
        return 0;
    }

    let mut rectangles = 0;

    for top in 0..height {
        for left in 0..width {
            if lines[top].as_bytes()[left] == b'+' {
                for bottom in top + 1..height {
                    if lines[bottom].as_bytes()[left] == b'+' {
                        for right in left + 1..width {
                            if lines[top].as_bytes()[right] == b'+' &&
                               lines[bottom].as_bytes()[right] == b'+' {
                                if (top + 1..bottom).all(|row| lines[row].as_bytes()[left] == b'|' || lines[row].as_bytes()[left] == b'+') &&
                                   (top + 1..bottom).all(|row| lines[row].as_bytes()[right] == b'|' || lines[row].as_bytes()[right] == b'+') &&
                                   (left + 1..right).all(|col| lines[top].as_bytes()[col] == b'-' || lines[top].as_bytes()[col] == b'+') &&
                                   (left + 1..right).all(|col| lines[bottom].as_bytes()[col] == b'-' || lines[bottom].as_bytes()[col] == b'+') {
                                    rectangles += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    rectangles
}

fn main() {
    // Example usage of the count function.
    let lines = &[
        "+------+----+",
        "|      |    |",
        "+---+--+    |",
        "|   |       |",
        "+---+-------+",
    ];
    println!("Number of rectangles: {}", count(lines));
}
