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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tt() {
        let test_file: &str = include_str!("../resources/test_input.txt");

        let mut trees: Vec<Vec<u8>> = Vec::new();

        for line in test_file.lines() {
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
}
