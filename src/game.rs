use crate::enemy::Enemy;
use crate::physics::sprite_collision;
use crate::player::Player;
use crate::tile_map::TileMap;
use crate::tile_map::CANVAS_WIDTH;
use quicksilver::{
  graphics::Color,
  lifecycle::{State, Window},
  Result,
};
use rand::Rng;

pub struct Game {
  tile_map: TileMap,
  player: Player,
  enemies: Vec<Enemy>,
  last_enemy_spawned_at: f64,
  time_to_spawn_enemy: f64,
}

impl State for Game {
  fn new() -> Result<Game> {
    let tile_map = TileMap::new(21, 21, 30, 30, 0.1);
    let player = Player::new(300, 550, 38, 43);
    Ok(Game {
      tile_map,
      player,
      enemies: Vec::new(),
      last_enemy_spawned_at: 0.0,
      time_to_spawn_enemy: 1000.0,
    })
  }

  fn draw(&mut self, window: &mut Window) -> Result<()> {
    window.clear(Color::WHITE)?;
    self.tile_map.draw(window)?;
    self.player.draw(window)?;

    for enemy in self.enemies.iter_mut() {
      enemy.draw(window)?;
    }

    self.enemies.retain(|enemy| !enemy.dead);

    if self.last_enemy_spawned_at >= self.time_to_spawn_enemy {
      self.last_enemy_spawned_at = 0.0;
      self.spawn_enemy();
    }

    self.last_enemy_spawned_at += window.update_rate();

    self.check_collisions();

    Ok(())
  }
}

impl Game {
  fn spawn_enemy(&mut self) {
    let mut rng = rand::thread_rng();
    let enemy = Enemy::new(rng.gen_range(20, CANVAS_WIDTH - 20), -20, 40, 32);
    self.enemies.push(enemy);
  }

  fn check_collisions(&mut self) {
    for player_bullet in self.player.bullets.iter_mut() {
      for enemy in self.enemies.iter_mut() {
        if sprite_collision(player_bullet, enemy) {
          enemy.hit();
        }
      }
    }
  }
}
