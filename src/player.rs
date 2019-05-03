use crate::tile_map::CANVAS_HEIGHT;
use crate::tile_map::CANVAS_WIDTH;
use quicksilver::{
  geom::Shape,
  graphics::Image,
  input::Key,
  lifecycle::{Asset, Window},
  prelude::Background::Img,
  Result,
};

const PLAYER_SPEED: f64 = 0.5;

pub struct Player {
  asset: Asset<Image>,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
}

impl Player {
  pub fn new(x: i32, y: i32, width: i32, height: i32) -> Player {
    let asset = Asset::new(Image::load("sprites/nau.gif"));

    Player {
      asset,
      x,
      y,
      width,
      height,
    }
  }

  pub fn draw(&mut self, window: &mut Window) -> Result<()> {
    let x = self.x;
    let y = self.y;

    self.asset.execute(|image| {
      window.draw(&image.area().with_center((x, y)), Img(&image));
      Ok(())
    })?;

    let speed = (PLAYER_SPEED * window.update_rate()).ceil() as i32;

    if window.keyboard()[Key::Left].is_down() {
      self.change_position_by(-speed, 0);
    }
    if window.keyboard()[Key::Right].is_down() {
      self.change_position_by(speed, 0);
    }
    if window.keyboard()[Key::Down].is_down() {
      self.change_position_by(0, speed);
    }
    if window.keyboard()[Key::Up].is_down() {
      self.change_position_by(0, -speed);
    }

    Ok(())
  }

  fn change_position_by(&mut self, x: i32, y: i32) {
    self.x += x;
    self.y += y;

    if self.x - self.width / 2 <= 0 {
      self.x = self.width / 2;
    }

    if self.x + self.width / 2 >= CANVAS_WIDTH {
      self.x = CANVAS_WIDTH - self.width / 2;
    }

    if self.y - self.height / 2 <= 0 {
      self.y = self.height / 2;
    }

    if self.y + self.height / 2 >= CANVAS_HEIGHT {
      self.y = CANVAS_HEIGHT - self.height / 2;
    }
  }
}
