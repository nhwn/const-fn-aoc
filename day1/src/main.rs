mod data;
use data::DATA;

mod part1 {
    pub const fn solve<const N: usize>(depths: [u32; N]) -> usize {
        let mut i = 1usize;
        let mut counter = 0;
        while i < N {
            if depths[i] > depths[i - 1] {
                counter += 1;
            }
            i += 1;
        }
        counter
    }
}

mod part2 {
    pub const fn solve<const N: usize>(depths: [u32; N]) -> usize {
        let mut i = 3usize;
        let mut counter = 0;
        while i < N {
            // if depths[i] + depths[i - 1] + depths[i - 2] > depths[i - 1] + depths[i - 2] + depths[i - 3] {
            if depths[i] > depths[i - 3] {
                counter += 1;
            }
            i += 1;
        }
        counter
    }
}

fn main() {
    let _: [(); 0] = [(); part1::solve(DATA)];
    let _: [(); 0] = [(); part2::solve(DATA)];
}

