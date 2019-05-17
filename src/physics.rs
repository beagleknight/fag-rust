use crate::sprite::Dimensions2D;
use crate::sprite::Sprite;
use crate::sprite::Vector2D;

pub fn sprite_collision(sprite_1: &mut Sprite, sprite_2: &mut Sprite) -> bool {
  let Vector2D { x: x1_1, y: y1_1 } = sprite_1.position();
  let Dimensions2D { w: w1, h: h1 } = sprite_1.size();
  let x2_1 = x1_1 + w1;
  let y2_1 = y1_1 + h1;
  let Vector2D { x: x1_2, y: y1_2 } = sprite_2.position();
  let Dimensions2D { w: w2, h: h2 } = sprite_2.size();
  let x2_2 = x1_2 + w2;
  let y2_2 = y1_2 + h2;
  (x1_1 < x2_2) && (x2_1 > x1_2) && (y1_1 < y2_2) && (y2_1 > y1_2)
}
