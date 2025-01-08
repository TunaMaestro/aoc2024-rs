// advent_of_code::solution!(11);

#[derive(PartialEq, Debug)]
struct Stone {
    value: usize,
    next: Option<Box<Stone>>,
}

impl Stone {
    fn blink(mut stone: Box<Stone>) -> Box<Stone> {
        stone.next = stone.next.map(Stone::blink);
        if stone.value == 0 {
            stone.value = 1;
            return stone;
        }
        let digits = stone.value.ilog10() + 1;
        if digits % 2 == 0 && true {
            let base_10_mask = 10usize.pow(digits / 2);
            let new_left_val = stone.value / base_10_mask;
            let new_right_val = stone.value % base_10_mask;

            stone.value = new_right_val;

            let new = Box::new(Stone {
                value: new_left_val,
                next: Some(stone),
            });
            return new;

            // if self.left.is_none() {
            //     self.left = Some(Box::new(Stone {
            //         value: new_left_val,
            //         left: None,
            //         right: None,
            //     }));
            //     self.value = new_right_val;
            // } else if self.right.is_none() {
            //     self.right = Some(Box::new(Stone {
            //         value: new_right_val,
            //         left: None,
            //         right: None,
            //     }));
            //     self.value = new_left_val;
            // } else {
            //     // Right rotate always

            //     // // randomly rotate
            //     // if hash(self.value) % 2 == 0 {
            //     //     // left
            //     // } else {
            //     //     // right
            //     // }
            // }
        } else {
            stone.value *= 2024;
            return stone;
        }
    }

    fn blink_r(maybe_stone: Option<Box<Stone>>) {
        if let Some(mut stone) = maybe_stone {
            let next = stone.next;
            if stone.value == 0 {
                stone.value = 1;
            } else {
                let digits = stone.value.ilog10() + 1;
                if digits % 2 == 0 {
                    if digits % 2 == 0 && true {
                        let base_10_mask = 10usize.pow(digits / 2);
                        let new_left_val = stone.value / base_10_mask;
                        let new_right_val = stone.value % base_10_mask;

                        stone.value = new_left_val;

                        let new = Box::new(Stone {
                            value: new_right_val,
                            next,
                        });
                        stone.next = Some(new);
                    } else {
                        stone.value *= 2024;
                    }
                }
            }
            Stone::blink_r(next);
        }
    }
    fn sum(stone: Box<Stone>) -> usize {
        return stone.value + stone.next.map(Stone::sum).unwrap_or(0);
    }

    fn display(stone: &Box<Stone>) -> String {
        let a = format!("{0} ", stone.value);
        let b = stone;
        let c = b
            .next
            .as_ref()
            .map(|x| Stone::display(x))
            .unwrap_or_else(|| "\n".to_owned());
        a + c.as_str()
    }
}

fn parse(input: &str) -> Box<Stone> {
    let vals: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Expect integer"))
        .collect();
    let mut stones = None;
    for &val in vals.iter().rev() {
        let s = Some(Box::new(Stone {
            value: val,
            next: stones,
        }));
        stones = s;
    }
    stones.expect("Expected at least once stone")
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones = parse(input);
    println!("{}", Stone::display(&stones));
    for _ in 0..25 {
        stones = Stone::blink(stones);
        println!("{}", Stone::display(&stones));
    }
    Some(Stone::sum(stones))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let r = parse("1 2 3");
        let exp = Box::new(Stone {
            value: 1,
            next: Some(Box::new(Stone {
                value: 2,
                next: Some(Box::new(Stone {
                    value: 3,
                    next: None,
                })),
            })),
        });
        assert_eq!(r, exp);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
