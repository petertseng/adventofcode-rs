fn paper(dimensions: &[Vec<u32>]) -> u32 {
    let paper = dimensions.iter().map(|dim| {
        let sides = vec![dim[0] * dim[1], dim[0] * dim[2], dim[1] * dim[2]];
        sides.iter().sum::<u32>() * 2 + sides.iter().min().unwrap()
    });
    paper.sum()
}

fn ribbon(dimensions: &[Vec<u32>]) -> u32 {
    dimensions
        .iter()
        .map(|dim| (dim[0] + dim[1]) * 2 + dim.iter().product::<u32>())
        .sum()
}

fn main() {
    let dimensions = adventofcode::read_input_lines(|line| {
        let mut dimensions = adventofcode::numbers(line);
        dimensions.sort_unstable();
        dimensions
    });

    println!("{}", paper(&dimensions));
    println!("{}", ribbon(&dimensions));
}
