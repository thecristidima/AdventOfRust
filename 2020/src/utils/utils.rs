pub mod files {
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{BufReader, Lines};
    use std::str::FromStr;

    pub fn read_lines_iter(path: &str) -> Lines<BufReader<File>> {
        let file = File::open(path).expect("file exists");
        let reader = BufReader::new(file);

        reader.lines()
    }

    pub fn read_lines_as_vec<T>(path: &str) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        read_lines_iter(path)
            .map(|line| line.unwrap().parse::<T>().unwrap())
            .collect::<Vec<T>>()
    }
}
