use specs::{System, ReadStorage, ReadExpect, Entities, Join};
use crate::walker::Mobile;
use crate::integrator::Step;

pub struct OutputPositionSystem;

impl<'a> System<'a> for OutputPositionSystem {
    type SystemData = (
        ReadExpect<'a, Step>,
        Entities<'a>,
        ReadStorage<'a, Mobile>,
    );

    fn run(&mut self, (step, ent, mobile): Self::SystemData) {
        
        if step.n % 100 == 0 {
            let tot_walkers = (&ent).join().count();
            let fix_walkers = (&ent, !&mobile).join().count();
            println!("Step {}, {}/{} walkers in the tree", step.n, fix_walkers, tot_walkers);
        }
    }
}