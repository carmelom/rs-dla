use specs::{System, ReadExpect};
use crate::walker::Counter;
use crate::integrator::Step;

pub struct OutputPositionSystem;

impl<'a> System<'a> for OutputPositionSystem {
    type SystemData = (
        ReadExpect<'a, Step>,
        ReadExpect<'a, Counter>,
    );

    fn run(&mut self, (step, counter): Self::SystemData) {
        
        if step.n % 100 == 0 {
            println!("Step {}, {}/{} walkers in the tree", step.n, counter.fix, counter.tot);
        }
    }
}