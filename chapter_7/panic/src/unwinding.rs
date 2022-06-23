pub fn run() {
    let crew_share = pirate_share(100, 0);
    println!("{}", crew_share);
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}
