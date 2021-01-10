struct Slope {
    along: usize,
    down: usize,
}

impl Slope {
    fn new(along: usize, down: usize) -> Slope {
        Slope {
            along,
            down
        }
    }
}

pub fn run(lines: Vec<String>, part: usize) -> Result<(), &'static str> {
    let matrix: Vec<Vec<_>> = lines.iter().map(|x| x.chars().collect()).collect();
    let slope = Slope::new(3, 1);

    let mut num_of_trees = number_of_trees_hit(&matrix, slope);
    if part == 1 {
        println!("number of trees hit = {}", num_of_trees);
        return Ok(())
    }

    let slopes = [(1, 1), (5, 1), (7, 1), (1, 2)];
    for slope_coord in slopes.iter() {
        let slope = Slope::new(slope_coord.0, slope_coord.1);
        num_of_trees *= number_of_trees_hit(&matrix, slope);
    }

    println!("total number from multiplying = {}", num_of_trees);
    Ok(())
}

fn number_of_trees_hit(matrix: &Vec<Vec<char>>, slope: Slope) -> u64 {
    let mut row = 0;
    let mut col = 0;
    let mut num_of_trees = 0;
    let repeat_len = matrix[0].len();

    while row < matrix.len() {
        let this_pos = matrix[row][col % repeat_len];
        if this_pos.to_string() == "#" {
            num_of_trees += 1;
        }
        row += slope.down;
        col += slope.along;
    }

    num_of_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_trees() {
        let matrix: Vec<Vec<char>> = vec![
            "..##.#.".chars().collect(),
            ".###.##".chars().collect(),
            "..#..#.".chars().collect(),
            "..#..##".chars().collect(),
            ".##....".chars().collect(),
        ];
        let slope = Slope::new(2, 1);
        assert_eq!(3, number_of_trees_hit(&matrix, slope));
    }
}