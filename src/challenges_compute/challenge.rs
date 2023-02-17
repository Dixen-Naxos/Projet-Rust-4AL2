pub trait Challenge {

    type Input;
    type Output;

    fn name() -> String;

    fn new(input: Self::Input) -> Self;

    fn solve(&self) -> Self::Output;

    fn verify(&self, answer: &Self::Output) -> bool;
}