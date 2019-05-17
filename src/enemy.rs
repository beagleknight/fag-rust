use crate::bullet::Bullet;
use crate::sprite::Dimensions2D;
use crate::sprite::Sprite;
use crate::sprite::Vector2D;
use crate::tile_map::CANVAS_HEIGHT;
use crate::tile_map::CANVAS_WIDTH;
use quicksilver::{
  geom::Shape,
  graphics::Image,
  lifecycle::{Asset, Window},
  prelude::Background::Img,
  Result,
};
use rand::Rng;

const CANNON_Y: i32 = 15;
const TIME_TO_CHANGE_DIRECTION: f64 = 2000.0;
const TIME_TO_SHOOT: f64 = 1000.0;

pub struct Enemy {
  asset: Asset<Image>,
  x: i32,
  y: i32,
  vx: f64,
  vy: f64,
  width: i32,
  height: i32,
  pub dead: bool,
  pub bullets: Vec<Bullet>,
  move_right: bool,
  last_changed_direction_at: f64,
  last_shoot_at: f64,
}

impl Sprite for Enemy {
  fn position(&self) -> Vector2D {
    Vector2D {
      x: self.x,
      y: self.y,
    }
  }
  fn size(&self) -> Dimensions2D {
    Dimensions2D {
      w: self.width,
      h: self.height,
    }
  }
}

impl Enemy {
  pub fn new(x: i32, y: i32, width: i32, height: i32) -> Enemy {
    let mut rng = rand::thread_rng();
    let asset = Asset::new(Image::load("sprites/tiefighter.gif"));

    Enemy {
      asset,
      x,
      y,
      vx: 0.1,
      vy: rng.gen_range(0.1, 0.2),
      width,
      height,
      dead: false,
      bullets: Vec::new(),
      move_right: true,
      last_changed_direction_at: 0.0,
      last_shoot_at: 0.0,
    }
  }

  pub fn draw(&mut self, window: &mut Window) -> Result<()> {
    let x = self.x;
    let y = self.y;

    self.asset.execute(|image| {
      window.draw(&image.area().with_center((x, y)), Img(&image));
      Ok(())
    })?;

    self.y += (self.vy * window.update_rate()) as i32;
    self.x += if self.move_right {
      (self.vx * window.update_rate()) as i32
    } else {
      (-self.vx * window.update_rate()) as i32
    };

    if !self.dead {
      self.dead = self.y >= CANVAS_HEIGHT + self.height / 2
        || self.x - self.width / 2 <= 0
        || self.x + self.width / 2 >= CANVAS_WIDTH;
    }

    if self.last_changed_direction_at >= TIME_TO_CHANGE_DIRECTION {
      let mut rng = rand::thread_rng();
      self.last_changed_direction_at = 0.0;
      self.move_right = rng.gen_bool(0.5);
    }

    if self.vy > 0.0 && self.last_shoot_at >= TIME_TO_SHOOT {
      self.last_shoot_at = 0.0;
      self.shoot();
    }

    for bullet in self.bullets.iter_mut() {
      bullet.draw(window)?;
    }

    self.bullets.retain(|bullet| !bullet.dead);

    self.last_changed_direction_at += window.update_rate();
    self.last_shoot_at += window.update_rate();

    Ok(())
  }

  fn shoot(&mut self) {
    let bullet = Bullet::new(self.x, self.y + CANNON_Y, true);
    self.bullets.push(bullet);
  }

  pub fn hit(&mut self) {
    self.dead = true;
  }
}
