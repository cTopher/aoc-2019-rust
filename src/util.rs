pub fn read_numbers<P: AsRef<Path>>(path: P) -> impl Iterator<Item=u32> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u32>().unwrap())
}