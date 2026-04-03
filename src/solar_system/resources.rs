use bevy::{
    ecs::{entity::Entity, resource::Resource},
    math::ops::powf,
};

#[derive(Resource)]
pub(crate) struct CameraController {
    pub(crate) scale: f32,
    pub(crate) target: Option<Entity>,
}

impl CameraController {
    pub(crate) fn zoom_in(&mut self) {
        self.scale *= 0.91;
    }

    pub(crate) fn zoom_out(&mut self) {
        self.scale *= 1.1;
    }

    pub(crate) fn zoom_in_continuous(&mut self, delta_secs: f32) {
        self.scale *= powf(0.25f32, delta_secs);
    }

    pub(crate) fn zoom_out_continuous(&mut self, delta_secs: f32) {
        self.scale *= powf(4.0f32, delta_secs);
    }
}

pub(crate) fn new_camera_controller() -> CameraController {
    CameraController {
        scale: 1.0,
        target: None,
    }
}

#[derive(Resource)]
pub(crate) struct OrbitRunner {
    pub(crate) current_interval: usize,
    pub(crate) timesteps: [f32; 5], // time to pass per tick, increasing with index
    pub(crate) paused: bool,
    pub(crate) timestep: f32,
}

impl OrbitRunner {
    pub(crate) fn speed_up(&mut self) {
        if self.current_interval == self.timesteps.len() - 1 {
            return;
        }
        self.current_interval += 1;
        self.timestep = self.timesteps[self.current_interval];
        println!("timestep: {0}s", self.timestep);
    }

    pub(crate) fn slow_down(&mut self) {
        if self.current_interval == 0 {
            return;
        }
        self.current_interval -= 1;
        self.timestep = self.timesteps[self.current_interval];
        println!("timestep: {0}s", self.timestep);
    }

    pub(crate) fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        println!("paused: {0}", self.paused);
    }
}

pub(crate) fn new_orbit_timer() -> OrbitRunner {
    let timesteps = [1., 60., 3600., 86400., 604800.]; // 1 sec, 1 min, 1 hr, 1 day, 1 week
    let current_interval = 0;

    OrbitRunner {
        current_interval,
        timesteps,
        paused: false,
        timestep: timesteps[current_interval],
    }
}
