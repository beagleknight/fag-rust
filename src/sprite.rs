pub struct Vector2D {
  pub x: i32,
  pub y: i32,
}

pub struct Dimensions2D {
  pub w: i32,
  pub h: i32,
}

pub trait Sprite {
  fn position(&self) -> Vector2D;
  fn size(&self) -> Dimensions2D;
}
