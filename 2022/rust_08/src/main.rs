// https://adventofcode.com/2022/day/8

use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut trees: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let row: Vec<u8> = line
            .chars()
            .map(|s| s.to_string().parse::<u8>().unwrap())
            .collect();

        trees.push(row);
    }

    let tree_slices: Vec<&[u8]> = trees.iter().map(|v| v.as_slice()).collect();
    let visible_tree_idx = get_visible_trees(&tree_slices);

    println!("Count of visible trees: {}", visible_tree_idx.len());

    // part 2
    let mut positions: Vec<(usize, usize)> = Vec::new();

    for (x, row) in trees.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            positions.push((x, y));
        }
    }

    let max_score = positions
        .iter()
        .map(|position| calculate_scenic_score(tree_slices.as_ref(), *position))
        .max();

    if let Some(score) = max_score {
        println!("Max scenic score: {}", score);
    } else {
        eprintln!("No scenic score found!");
    }
}

fn get_visible_trees(trees: &[&[u8]]) -> Vec<(usize, usize)> {
    let mut visible_trees: Vec<(usize, usize)> = Vec::new();

    for (x, &row) in trees.iter().enumerate() {
        for (y, &tree) in row.iter().enumerate() {
            let left = row.iter().take(y).all(|&a| a < tree);
            let right = row.iter().skip(y + 1).all(|&a| a < tree);

            let column: Vec<u8> = trees.iter().map(|row| row[y]).collect();

            let up = column.iter().take(x).all(|&a| a < tree);
            let down = column.iter().skip(x + 1).all(|&a| a < tree);

            if left || right || up || down {
                visible_trees.push((x, y));
            }
        }
    }

    visible_trees
}

fn calculate_scenic_score(trees: &[&[u8]], position: (usize, usize)) -> usize {
    let (x, y) = position;

    let hight = trees[x][y];

    let row: Vec<u8> = trees[x].to_vec();
    let column: Vec<u8> = trees.iter().map(|row| row[y]).collect();

    let left: Vec<u8> = row.iter().take(y).rev().copied().collect();
    let right: Vec<u8> = row.iter().skip(y + 1).copied().collect();
    let up: Vec<u8> = column.iter().take(x).rev().copied().collect();
    let down: Vec<u8> = column.iter().skip(x + 1).copied().collect();

    let left_count = scenic_score_side_count(left.as_ref(), hight);
    let right_count = scenic_score_side_count(right.as_ref(), hight);
    let up_count = scenic_score_side_count(up.as_ref(), hight);
    let down_count = scenic_score_side_count(down.as_ref(), hight);

    left_count * right_count * up_count * down_count
}

fn scenic_score_side_count(side_trees: &[u8], hight: u8) -> usize {
    let mut count = 0;

    for &tree in side_trees {
        count += 1;

        // counts the first bigger tree
        if tree >= hight {
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = include_str!("../resources/test_input.txt");

    #[test]
    fn test_visibility() {
        let mut trees: Vec<Vec<u8>> = Vec::new();

        for line in TEST_FILE.lines() {
            let row: Vec<u8> = line
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect();

            trees.push(row);
        }

        let tree_slices: Vec<&[u8]> = trees.iter().map(|v| v.as_slice()).collect();

        let v_trees = get_visible_trees(&tree_slices);

        let right_trees: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 4),
            (2, 0),
            (2, 1),
            (2, 3),
            (2, 4),
            (3, 0),
            (3, 2),
            (3, 4),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),
        ];

        assert_eq!(v_trees, right_trees);
    }

    #[test]
    fn test_scenic_score_1() {
        let mut trees: Vec<Vec<u8>> = Vec::new();

        for line in TEST_FILE.lines() {
            let row: Vec<u8> = line
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect();

            trees.push(row);
        }
        let tree_slices: Vec<&[u8]> = trees.iter().map(|v| v.as_slice()).collect();

        let score = calculate_scenic_score(tree_slices.as_slice(), (1, 2));

        assert_eq!(score, 4);
    }

    #[test]
    fn test_scenic_score_2() {
        let mut trees: Vec<Vec<u8>> = Vec::new();

        for line in TEST_FILE.lines() {
            let row: Vec<u8> = line
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect();

            trees.push(row);
        }
        let tree_slices: Vec<&[u8]> = trees.iter().map(|v| v.as_slice()).collect();

        let score = calculate_scenic_score(tree_slices.as_slice(), (3, 2));

        assert_eq!(score, 8);
    }
}
