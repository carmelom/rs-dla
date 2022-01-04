use crate::globals;
use crate::walker::{Mobile, Position};
use specs::{Entities, LazyUpdate, Read, ReadStorage, System};

pub struct AggregateSystem;

impl<'a> System<'a> for AggregateSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Mobile>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, mobile, position, updater): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;
        // for (fix, _) in (&position, !&mobile).join() {
        //     for (ent, mob, _) in (&entities, &position, &mobile).join() {
        //         let distance = (fix.pos - mob.pos).norm();
        //         if distance <= globals::RADIUS {
        //             updater.remove::<Mobile>(ent);
        //         }
        //     }
        // }
        (&position, !&mobile).par_join().for_each(|(fix, _)| {
            (&entities, &position, &mobile)
                .par_join()
                .for_each(|(ent, mob, _)| {
                    let distance = (fix.pos - mob.pos).norm();
                    if distance <= 2.0 * globals::RADIUS {
                        updater.remove::<Mobile>(ent);
                    }
                })
        })
    }
}
