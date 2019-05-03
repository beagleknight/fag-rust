use quicksilver::{
  geom::Shape,
  graphics::Image,
  lifecycle::{Asset, Window},
  prelude::Background::Img,
  Result,
};
use rand::Rng;

pub const CANVAS_WIDTH: i32 = 600;
pub const CANVAS_HEIGHT: i32 = 600;

pub struct TileMap {
  space_assets: Vec<Asset<Image>>,
  rows: i32,
  cols: i32,
  tile_width: i32,
  tile_height: i32,
  step_y: f64,
  offset_y: i32,
  canvas_outside: i32,
  map: Vec<usize>,
}

impl TileMap {
  pub fn new(rows: i32, cols: i32, tile_width: i32, tile_height: i32, step_y: f64) -> TileMap {
    let mut map = Vec::new();
    let tiles_number = rows * cols;
    let mut rng = rand::thread_rng();
    let rows_per_canvas = CANVAS_HEIGHT / tile_height;
    for _ in 0..tiles_number {
      map.push(rng.gen_range(0, 6));
    }

    let mut space_assets = Vec::new();
    for i in 1..7 {
      space_assets.push(Asset::new(Image::load(format!("sprites/space{}.gif", i))));
    }

    TileMap {
      space_assets,
      rows,
      cols,
      tile_width,
      tile_height,
      step_y,
      offset_y: 0,
      canvas_outside: (rows - rows_per_canvas) * tile_height,
      map,
    }
  }

  pub fn draw(&mut self, window: &mut Window) -> Result<()> {
    let tile_width = self.tile_width;
    let tile_height = self.tile_height;
    let canvas_outside = self.canvas_outside;
    let offset_y = self.offset_y;
    let mut x = 0;
    let mut y = offset_y;

    for i in 0..self.rows {
      for j in 0..self.cols {
        let tile_id = self.map[(i * self.rows + j) as usize];
        self.space_assets[tile_id].execute(|image| {
          window.draw(&image.area().with_center((x, y)), Img(&image));
          Ok(())
        })?;
        x += tile_width;
      }
      x = 0;
      y += tile_height;

      if y >= CANVAS_HEIGHT + (tile_height / 2) {
        y -= CANVAS_HEIGHT + canvas_outside;
      }
    }

    self.offset_y += (self.step_y * window.update_rate()).ceil() as i32;

    if self.offset_y > CANVAS_HEIGHT + tile_height / 2 {
      self.offset_y = self.offset_y - CANVAS_HEIGHT - self.canvas_outside;
    }

    Ok(())
  }
}
