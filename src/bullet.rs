use crate::sprite::Dimensions2D;
use crate::sprite::Sprite;
use crate::sprite::Vector2D;
use quicksilver::{
  geom::Shape,
  graphics::Image,
  lifecycle::{Asset, Window},
  prelude::Background::Img,
  Result,
};

const BULLET_SPEED: f64 = 0.5;

pub struct Bullet {
  asset: Asset<Image>,
  x: i32,
  y: i32,
  from_enemy: bool,
  pub dead: bool,
}

impl Sprite for Bullet {
  fn position(&self) -> Vector2D {
    Vector2D {
      x: self.x,
      y: self.y,
    }
  }
  fn size(&self) -> Dimensions2D {
    Dimensions2D { w: 11, h: 10 }
  }
}

impl Bullet {
  pub fn new(x: i32, y: i32, from_enemy: bool) -> Bullet {
    let asset = if from_enemy {
      Asset::new(Image::load("sprites/enemy_bullet.gif"))
    } else {
      Asset::new(Image::load("sprites/player_bullet.gif"))
    };

    Bullet {
      asset,
      x,
      y,
      from_enemy,
      dead: false,
    }
  }

  pub fn draw(&mut self, window: &mut Window) -> Result<()> {
    let x = self.x;
    let y = self.y;

    self.asset.execute(|image| {
      window.draw(&image.area().with_center((x, y)), Img(&image));
      Ok(())
    })?;

    let speed = (BULLET_SPEED * window.update_rate()) as i32;

    self.y = if self.from_enemy {
      self.y + speed
    } else {
      self.y - speed
    };

    if !self.dead {
      self.dead = self.y <= 0;
    }

    Ok(())
  }
}
