fn day1() -> std::io::Result<()> {
    let mut input_vec: Vec<i32> = Vec::new();
    let mut seen_set = BTreeSet::new();
    let mut sum: i32 = 0;

    let f = File::open("day1.txt")?;
    let reader = BufReader::new(f);
    reader
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .for_each(|x| input_vec.push(x));

    for value in input_vec.iter().cycle() {
        sum += value;
        if seen_set.contains(&sum) {
            println!("The frequency is {}.", sum);
            break;
        } else {
            println!("Adding sum = {}.", sum);
            seen_set.insert(sum);
        }
    }
    Ok(())
}
