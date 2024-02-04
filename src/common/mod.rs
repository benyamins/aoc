
pub trait Solution {
    fn sol_1();
    fn sol_2();
    const TEST_INPUT: &'static str;

    fn solve(n_sol: u16) {
        match n_sol {
            1 => Self::sol_1(),
            2 => Self::sol_2(),
            _ => panic!("There is no other solution than 1 or 2")
        }
    }
}
