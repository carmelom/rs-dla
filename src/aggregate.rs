use crate::globals;
use crate::walker::{Mobile, Position};
use crate::neighborhood::Neighborhood;
use specs::{Entities, LazyUpdate, Read, ReadStorage, System, WriteExpect};
use std::sync::Mutex;

pub struct AggregateSystem;

impl<'a> System<'a> for AggregateSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Mobile>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
        WriteExpect<'a, Neighborhood>,
    );

    fn run(&mut self, (entities, mobile, position, updater, neighborhood): Self::SystemData) {
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
        let nh = Mutex::new(neighborhood);
        (&position, !&mobile).par_join().for_each(|(fix, _)| {
            let mut nh = nh.lock().unwrap();
            let bitset = nh.get_neighbours_in_area(fix.pos);
            (&entities, &position, &mobile, bitset)
                .par_join()
                .for_each(|(ent, mob, _, _)| {
                    let distance = (fix.pos - mob.pos).norm();
                    if distance <= globals::RADIUS {
                        updater.remove::<Mobile>(ent);
                    }
                })
        })
    }
}
