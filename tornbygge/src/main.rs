fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap(); // Appends
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums = buf
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut current_tower: Vec<i32> = vec![];
    let mut completed_towers = 0;
    let mut idx = 0;
    for &num in nums.iter() {
        if !current_tower.is_empty() && current_tower[idx - 1] < num {
            completed_towers += 1;
            idx = 0;
            current_tower.clear();
        }

        current_tower.push(num);
        idx += 1
    }

    if !current_tower.is_empty() {
        completed_towers += 1;
    }

    println!("{completed_towers}");
}
