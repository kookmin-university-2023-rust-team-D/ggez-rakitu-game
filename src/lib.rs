use bevy::{prelude::{Component, Vec3, Resource}, time::{Timer, TimerMode}};
// enemy, player 모듈화
pub mod enemy;
pub mod player;
pub mod turtles;

// 쓰일 컴포넌트들
#[derive(Component)]
pub struct Player{}

#[derive(Component)]
pub struct Enemy{}

#[derive(Component)]
pub struct Velocity{
    pub speed: Vec3,
}

#[derive(Component)]
pub struct Turtle{
}

#[derive(Resource)]
pub struct TurtleSpawnTimer {
    pub timer: Timer,
}

impl Default for TurtleSpawnTimer {
    fn default() -> TurtleSpawnTimer {
        TurtleSpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}


//사용된 전역 변수들
pub const PLANE_X: f32 = 200.0;
pub const PLANE_SIZE: Vec3 = Vec3::new(PLANE_X, 3.0, 0.0);
pub const PLANE: f32 = 48.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 70.0;
pub const ENEMY_SPEED: f32 = 300.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPAWN_TIME: f32 = 2.0;
