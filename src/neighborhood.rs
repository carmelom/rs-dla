// https://github.com/prixt/ggez-ecs-boids/blob/master/src/neighborhood.rs
use specs::{BitSet, world::Index};
use nalgebra::Vector2;

pub type AreaIndex = usize;

pub struct Neighborhood{
	areas: Vec<BitSet>,
	width: i32, height: i32,
    area_width: f32, area_hight: f32,
}

impl Neighborhood {
	pub fn new(width: i32, height: i32, area_width: f32, area_hight: f32) -> Self {
		let mut areas = Vec::with_capacity( (width * height) as AreaIndex );
		for _ in 0..(width*height) {
			areas.push(BitSet::new());
		}
		Self{areas, width, height, area_width, area_hight}
	}

    pub fn new_from_field(field_width: f32, field_height: f32, area_width: f32, area_hight: f32) -> Self {
        let width = (field_width / area_width).ceil() as i32;
        let height = (field_height / area_hight).ceil() as i32;
        Neighborhood::new(width, height, area_width, area_hight)
    }

	pub fn get(&self, x: i32, y: i32) -> &BitSet {
		let x = (x + self.width) % self.width;
		let y = (y + self.height) % self.height;
		&self.areas[(self.width * y + x) as AreaIndex]
	}

	pub fn insert(&mut self, x: i32, y: i32, id: Index) -> bool {
		self.areas[(self.width * y + x) as AreaIndex].add(id)
	}

	pub fn remove(&mut self, x: i32, y: i32, id: Index) -> bool {
		self.areas[(self.width * y + x) as AreaIndex].remove(id)
	}

    pub fn get_area_xy(&mut self, pos: Vector2<f32>) -> (i32, i32) {
        let px = (pos[0] / self.area_width) as i32;
        let py = (pos[1] / self.area_hight) as i32;
        (px, py)
    }

    pub fn get_neighbours_in_area(&mut self, pos: Vector2<f32>) -> &BitSet {
        let (px, py) = self.get_area_xy(pos);
        self.get(px, py)
    }

    pub fn get_neighbours_in_range(&mut self, pos: Vector2<f32>, range_x: f32, range_y: f32) -> BitSet {
        let v = Vector2::new(range_x, range_y);
            let (tl_x, tl_y) = self.get_area_xy(pos - v);
            let (br_x, br_y) = self.get_area_xy(pos + v);
            let mut bitset = BitSet::new();
            for dy in tl_y ..= br_y {
                for dx in tl_x ..= br_x {
                    bitset |= self.get(dx, dy);
                }
            }
        bitset
    }
}
