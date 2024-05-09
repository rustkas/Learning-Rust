// 02/addstring-2/main.rs
fn main() {
    let home_team = "Liverpool";
    let result = " beat ";
    let away_team = "Manchester United";
    
    let full_line = home_team.to_owned() + result + away_team;
    
    println!("{}", full_line);
}
