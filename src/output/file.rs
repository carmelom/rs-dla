//! Writes output files containing atomic trajectories.
use specs::{Component, Entities, Entity, Join, ReadStorage, System};
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;

/// A system that writes simulation data to file.
///
/// This system writes data `C` of entities associated with `A` to a file at a defined interval.
/// The data type `C` must be a [Component](specs::Component) and implement the
/// [Clone](struct.Clone.html) trait.
pub struct OutputSystem<C: Component + Clone, W: Write, F: Format<C, W>> {
    /// The [Write](std::io::Write)able output stream.
    stream: W,
    formatter: PhantomData<F>,
    marker: PhantomData<C>,
}

/// Creates a new [OutputSystem](struct.OutputSystem.html) to write per-atom [Component](specs::Component) data
/// according to the specified [Format](struct.Format.html).
///
/// The interval specifies how often, in integration steps, the file should be written.
///
/// Only component data of entities associated with `Atom` is written down.
///
/// For example, `new::<Position, Text>("pos.txt", 10).
pub fn new<C, F>(file_name: String) -> OutputSystem<C, BufWriter<File>, F>
where
    C: Component + Clone,
    F: Format<C, BufWriter<File>>,
{
    let path = Path::new(&file_name);
    let display = path.display();
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let writer = BufWriter::new(file);
    OutputSystem {
        stream: writer,
        formatter: PhantomData,
        marker: PhantomData,
    }
}

impl<'a, C, W, F> System<'a> for OutputSystem<C, W, F>
where
    C: Component + Clone,
    W: Write,
    F: Format<C, W>,
{
    type SystemData = (Entities<'a>, ReadStorage<'a, C>);

    fn run(&mut self, (entities, data): Self::SystemData) {
        // write each entity
        for (data, ent) in (&data, &entities).join() {
            F::write(&mut self.stream, ent, data.clone()).expect("Could not write.");
        }
    }
}

/// A trait implemented for each file output format.
pub trait Format<C, W>
where
    C: Component + Clone,
    W: Write,
{
    /// Writes data associated with an entity
    fn write(writer: &mut W, atom: Entity, data: C) -> Result<(), io::Error>;
}

/// Prints files in a [Format](struct.Format.html) that is human readable.
///
/// The output file is structured as follows. Each frame begins with the line
/// `step n atomNumber`, where `n` is the step number and `atomNumber` the number of
/// atoms to write to the file. This is followed by the `data : T` for each atom,
/// written to the file in the format `gen id: data`, where `gen` and `id` are the
/// [Entity](specs::Entity) generation and id, and data consists of the per-atom payload.
///
/// Components printed using text must implement the [Display](std::fmt::Display) trait.
pub struct Text {}
impl<C, W> Format<C, W> for Text
where
    C: Component + Clone + Display,
    W: Write,
{
    fn write(writer: &mut W, atom: Entity, data: C) -> Result<(), io::Error> {
        writeln!(writer, "{:?},{:?}: {}", atom.gen().id(), atom.id(), data)?;
        Ok(())
    }
}
