pub trait Solution {
    fn sol_1(input: String);
    fn sol_2(input: String);

    fn solve(n_sol: u16, input: String) {
        match n_sol {
            1 => Self::sol_1(input),
            2 => Self::sol_2(input),
            _ => panic!("There is no other solution than 1 or 2"),
        }
    }
}
