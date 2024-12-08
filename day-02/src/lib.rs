pub mod part1;
pub mod part2;

pub fn get_package_dimension(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            line.split('x')
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}
