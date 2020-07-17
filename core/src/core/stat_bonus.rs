pub fn create_all_stat_bonuses() -> Vec<i32> {

    let mut stat_bonus: Vec<i32> = Vec::new();

    stat_bonus.push(-6);

    for i in 1..18 {
        match i {
            1 => stat_bonus.push(-4),
            2 | 3 => stat_bonus.push(-3),
            4 | 5 => stat_bonus.push(-2),
            6 | 7 => stat_bonus.push(-1),
            8 | 9 => stat_bonus.push(0),
            10 | 11 => stat_bonus.push(1),
            12 | 13 => stat_bonus.push(2),
            14 | 15 => stat_bonus.push(3),
            16 | 17 => stat_bonus.push(4),
            18  => stat_bonus.push(5),
            _ => panic!("Oops!"),
        }
    }

    return stat_bonus;
}
