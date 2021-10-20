extern crate bevy;

use bevy::prelude::*;

pub const PLAYER_SPRITE: &str = "player_a_01.png";
pub const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
pub const ENEMY_SPRITE: &str = "enemy_a_01.png";
pub const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
pub const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
pub const SCALE: f32 = 0.5;
pub const TIME_STEP: f32 = 1. / 144.;
pub const MAX_ENEMIES: u32 = 4;
pub const MAX_FORMATION_MEMBERS: u32 = 3;
pub const PLAYER_RESPAWN_DELAY: f64 = 1.;

pub struct Materials {
    pub player: Handle<ColorMaterial>,
    pub player_laser: Handle<ColorMaterial>,
    pub enemy: Handle<ColorMaterial>,
    pub enemy_laser: Handle<ColorMaterial>,
    pub explosion: Handle<TextureAtlas>,
}
pub struct WinSize {
    #[allow(unused)]
    pub w: f32,
    pub h: f32,
}
pub struct ActiveEnemies(pub u32);

pub struct PlayerState {
    pub on: bool,
    pub last_shot: f64,
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: 0.,
        }
    }
}
impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }
    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = 0.
    }
}

pub struct Laser;

pub struct Player;
pub struct PlayerReadyFire(pub bool);
pub struct FromPlayer;

pub struct Enemy;
pub struct FromEnemy;

pub struct Explosion;
pub struct ExplosionToSpawn(pub Vec3);

pub struct Speed(pub f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}
