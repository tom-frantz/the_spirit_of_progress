use crate::components::world::render::resources::GeographicWorld;
use crate::components::world::render::MainTileMap;
use bevy::prelude::*;

pub enum TectonicSimulationSpeed {
    Slow,
    Medium,
    Fast,
}

impl Default for TectonicSimulationSpeed {
    fn default() -> Self {
        Self::Medium
    }
}

impl TectonicSimulationSpeed {
    pub fn speed_up(&self) -> Self {
        match self {
            TectonicSimulationSpeed::Slow => TectonicSimulationSpeed::Medium,
            TectonicSimulationSpeed::Medium => TectonicSimulationSpeed::Fast,
            TectonicSimulationSpeed::Fast => TectonicSimulationSpeed::Fast,
        }
    }

    pub fn slow_down(&self) -> Self {
        match self {
            TectonicSimulationSpeed::Slow => TectonicSimulationSpeed::Slow,
            TectonicSimulationSpeed::Medium => TectonicSimulationSpeed::Slow,
            TectonicSimulationSpeed::Fast => TectonicSimulationSpeed::Medium,
        }
    }

    pub fn steps_in_seconds(&self) -> f32 {
        match self {
            TectonicSimulationSpeed::Slow => 1.,
            TectonicSimulationSpeed::Medium => 0.5,
            TectonicSimulationSpeed::Fast => 0.25,
        }
    }
}

pub struct TectonicSimulationState {
    paused: bool,
    speed: TectonicSimulationSpeed,
    last_tick: f32,
}

impl Default for TectonicSimulationState {
    fn default() -> Self {
        TectonicSimulationState {
            paused: true,
            speed: Default::default(),
            last_tick: 0.0,
        }
    }
}

impl TectonicSimulationState {
    pub fn tick(&mut self, seconds_time_delta: f32) -> bool {
        if self.paused {
            return false;
        }

        self.last_tick += seconds_time_delta;
        let sim_speed_in_seconds = self.speed.steps_in_seconds();

        if self.last_tick >= sim_speed_in_seconds {
            self.last_tick -= sim_speed_in_seconds;
            true
        } else {
            false
        }
    }
}

pub struct TectonicWorldSimPlugin;

impl Plugin for TectonicWorldSimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TectonicSimulationState::default())
            .add_system(tectonic_speed_controls)
            .add_system(update_tectonic_simulation);
    }
}

pub fn step(
    mut geo_world: ResMut<GeographicWorld>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tile_maps: Query<Entity, With<MainTileMap>>,
) {
    let (next_tectonics, next_heights) = geo_world.tectonic_map.tick(&geo_world.height_map);
    geo_world.update(next_heights, next_tectonics);
    geo_world.draw_world_type(commands, asset_server, tile_maps)
}

pub fn update_tectonic_simulation(
    time: Res<Time>,
    mut tectonic_sim_state: ResMut<TectonicSimulationState>,

    mut geo_world: ResMut<GeographicWorld>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tile_maps: Query<Entity, With<MainTileMap>>,
) {
    let time = time.delta_seconds();

    let tick_happened = tectonic_sim_state.tick(time);

    if tick_happened {
        println!("TICK!");
        step(geo_world, commands, asset_server, tile_maps);
    }
}

pub fn tectonic_speed_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut tectonic_sim_state: ResMut<TectonicSimulationState>,

    mut geo_world: ResMut<GeographicWorld>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tile_maps: Query<Entity, With<MainTileMap>>,
) {
    let mut up_pressed = false;

    for key_code in keyboard_input.get_just_pressed() {
        match key_code {
            KeyCode::Space => tectonic_sim_state.paused = !tectonic_sim_state.paused,
            KeyCode::Left => tectonic_sim_state.speed = tectonic_sim_state.speed.slow_down(),
            KeyCode::Right => tectonic_sim_state.speed = tectonic_sim_state.speed.speed_up(),
            KeyCode::Up => {
                up_pressed = true;
            }
            _ => {}
        };
    }

    // Do this here since borrow checker doesn't like the for loop.
    if up_pressed {
        step(geo_world, commands, asset_server, tile_maps);
    }
}
