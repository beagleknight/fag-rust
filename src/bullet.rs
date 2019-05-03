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
  pub dead: bool,
}

impl Bullet {
  pub fn new(x: i32, y: i32) -> Bullet {
    let asset = Asset::new(Image::load("sprites/player_bullet.gif"));

    Bullet {
      asset,
      x,
      y,
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

    self.y -= speed;

    self.dead = self.y <= 0;

    Ok(())
  }
}
