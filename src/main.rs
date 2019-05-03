use crate::game::Game;
use crate::tile_map::{CANVAS_HEIGHT, CANVAS_WIDTH};
use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
};

mod bullet;
mod enemy;
mod game;
mod player;
mod tile_map;

fn main() {
    run::<Game>(
        "F.A.G. (Fast Arcade Game)",
        Vector::new(CANVAS_WIDTH, CANVAS_HEIGHT),
        Settings::default(),
    );
}
