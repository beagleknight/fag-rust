use crate::player::Player;
use crate::tile_map::TileMap;
use quicksilver::{
  graphics::Color,
  lifecycle::{State, Window},
  Result,
};

pub struct Game {
  tile_map: TileMap,
  player: Player,
}

impl State for Game {
  fn new() -> Result<Game> {
    let tile_map = TileMap::new(21, 21, 30, 30, 0.1);
    let player = Player::new(300, 550, 38, 43);
    Ok(Game { tile_map, player })
  }

  fn draw(&mut self, window: &mut Window) -> Result<()> {
    window.clear(Color::WHITE)?;
    self.tile_map.draw(window)?;
    self.player.draw(window)?;
    Ok(())
  }
}
