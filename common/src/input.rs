use std::io::stdin;

pub fn get_i_j() -> (usize, usize) {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    let vec = buf
        .split(" ")
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();
    (vec[0], vec[1])
}
