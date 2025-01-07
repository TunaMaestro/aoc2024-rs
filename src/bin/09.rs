#![feature(let_chains)]
advent_of_code::solution!(9);

fn to_blocks(input: &str) -> (Vec<usize>, Vec<isize>) {
    let blocks: Vec<usize> = input
        .as_bytes()
        .iter()
        .filter(|x| x.is_ascii_digit())
        .map(|&x| (x - b'0') as usize)
        .collect();
    let size: usize = blocks.iter().sum();
    let mut fs: Vec<isize> = Vec::with_capacity(size);
    for (i, &size) in blocks.iter().enumerate() {
        for _ in 0..size {
            if i % 2 == 0 {
                fs.push(i as isize / 2);
            } else {
                fs.push(-1);
            }
        }
    }
    assert!(blocks.len() % 2 == 1);
    (blocks, fs)
}

fn p(l: &[isize]) {
    // return;
    let s = String::from_utf8(
        l.iter()
            .map(|&x| if x == -1 { b'.' } else { (x as u8) + b'0' })
            .collect(),
    )
    .unwrap();
    println!("{}", s);
}

pub fn part_one(input: &str) -> Option<usize> {
    let (blocks, mut fs) = to_blocks(input);

    let mut left = blocks[0];
    let mut right = fs.len() - 1;
    while left < right {
        fs[left] = fs[right];
        fs[right] = -1;
        left += 1;
        right -= 1;
        // if fs[left] != -1 {
        //     // Must be an original block
        //     let block_index = fs[left];
        //     assert!(block_index >= 0);
        //     let block_size = blocks[block_index as usize * 2];
        //     left += block_size;
        // }
        // if fs[right] == -1 {
        //     let block_size = blocks[right_block_i as usize * 2 - 1];
        //     right -= block_size;
        // }
        while fs[left] != -1 {
            left += 1;
        }
        while fs[right] == -1 {
            right -= 1;
        }
    }

    Some(
        fs.iter()
            .take_while(|&&x| x >= 0)
            .enumerate()
            .map(|(i, &x)| i * (x as usize))
            .sum(),
    )
}

pub fn part_two_shit(input: &str) -> Option<usize> {
    let (blocks, mut fs) = to_blocks(input);
    let mut left = blocks[0];
    let mut right = fs.len() - 1;
    let mut left_size = 1;
    let mut right_size = 1;
    while left < right {
        p(&fs);

        println!("{left} > {left_size}  {right_size} < {right}\n");
        // dbg!(right, right_size);
        // Find size of right
        if fs[right - right_size] != -1 {
            right_size += 1;
        } else {
            // Try to copy if enough space
            if left_size < right_size {
                // Not enough space
                if fs[left + left_size] == -1 {
                    // Free space, expand
                    left_size += 1
                } else {
                    // Not enough space, move onto next right block
                    right -= right_size;
                    right_size = 0;
                }
            } else {
                for _ in 0..right_size {
                    fs[left] = fs[right];
                    fs[right] = -1;
                    left += 1;
                    right -= 1;
                }
                right_size = 0;
                left_size = 0;
            }
        }
    }

    p(&fs);

    Some(score(&fs))
}

fn score(fs: &[isize]) -> usize {
    fs.iter()
        .enumerate()
        .filter(|(_, &x)| x >= 0)
        .map(|(i, &x)| i * (x as usize))
        .sum()
}

fn find_first_free(disk: &[isize], required: usize) -> Option<usize> {
    // disk.iter()
    //     .enumerate()
    //     .fold(None, |acc, x| {
    //         if x != -1 {
    //             acc
    //         } else {

    //         }
    //     })
    //     .map(|(index, length)| index)

    let mut start = None;
    let mut length = 0;
    for (i, &x) in disk.iter().enumerate() {
        if length >= required {
            return start;
        }
        if x != -1 {
            length = 0;
        } else {
            if length == 0 {
                start = Some(i);
            }
            length += 1
        }
    }
    if length >= required {
        start
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_blocks, mut fs) = to_blocks(input);

    // let mut left_open = blocks[0];
    let mut right_filled = fs.len() - 1;

    while 0 < right_filled {
        // dbg!(right_filled);
        // p(&fs);

        let mut right_size = 0;
        let right_block_id = fs[right_filled];
        while right_filled > right_size && fs[right_filled - right_size] == right_block_id {
            right_size += 1;
        }

        if right_filled - right_size == 0 {
            break;
        }

        let left_open = find_first_free(&fs, right_size);

        // eprintln!("{left_open}({left_size}) {right_filled}({right_size})");
        if let Some(mut left) = left_open
            && left < right_filled
        {
            for _ in 0..right_size {
                (fs[left], fs[right_filled]) = (fs[right_filled], fs[left]);

                left += 1;
                right_filled -= 1;
            }
        } else {
            right_filled -= right_size;
        }
        while fs[right_filled] == -1 {
            right_filled -= 1;
        }
    }
    // p(&fs);

    Some(score(&fs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        {
            let result = part_one(&advent_of_code::template::read_file_part(
                "examples", DAY, 1,
            ));
            assert_eq!(result, Some(60));
        }
        {
            let result = part_one(&advent_of_code::template::read_file("examples", DAY));
            assert_eq!(result, Some(1928));
        }
    }

    #[test]
    fn test_find_first() {
        assert_eq!(
            find_first_free(&[-1, -1, -1, 1, 2, 3, -1, -1, -1, -1], 3),
            Some(0)
        );
        assert_eq!(
            find_first_free(&[-1, -1, -1, 1, 2, 3, -1, -1, -1, -1], 4),
            Some(6)
        );
        assert_eq!(
            find_first_free(&[-1, -1, -1, 1, 2, 3, -1, -1, -1, -1], 5),
            None
        );
    }

    #[test]
    fn test_part_two() {
        {
            let result = part_two(&advent_of_code::template::read_file_part(
                "examples", DAY, 1,
            ));
            assert_eq!(result, Some(132));
        }
        {
            let result = part_two(&advent_of_code::template::read_file("examples", DAY));
            assert_eq!(result, Some(2858));
        }
    }
}
