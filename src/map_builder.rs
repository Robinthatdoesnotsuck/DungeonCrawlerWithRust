use crate::prelude::*;

const NUM_ROOMS: usize = 20;
pub struct MapBuilder {
    pub map:Map,
    pub rooms:Vec<Rect>,
    pub plaer_start:Point,
}

impl MapBuilder{
    pub fn new() -> Self{
        Self {

        }
    }

    fn fill(&mut self, tile:TileType) {
        self.map.tiles.iter_mut().for_each(|t|*t = tile);   
    }
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0
                        && p.y < SCREEN_HEIGHT {
                            let idx = map_idx(p.x, p.y);
                            self.map.tiles[idx] = TileType::Floor;
                        }
                });
                self.rooms.push(room)
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2)..=max(y1, y2) {
            
        }
    }
}