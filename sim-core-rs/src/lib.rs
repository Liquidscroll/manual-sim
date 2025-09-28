use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests;

#[wasm_bindgen]
pub struct Drivetrain {
    rpm: f32,
    speed: f32,
    gear: i32,
    throttle: f32,
    clutch: f32,
    engine_on: bool,
}

#[wasm_bindgen]
impl Drivetrain {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Drivetrain {
        Drivetrain {
            rpm: 900.0,
            speed: 0.0,
            gear: 0,       // 0 == neutral
            throttle: 0.0, // 0..1
            clutch: 0.0,   // 0..1, 1=fully engaged
            engine_on: true,
        }
    }

    pub fn rpm(&self) -> f32 {
        self.rpm
    }
    pub fn speed_kmh(&self) -> f32 {
        self.speed * 3.6
    }
    pub fn gear(&self) -> i32 {
        self.gear
    }
    pub fn throttle(&self) -> f32 {
        self.throttle
    }
    pub fn clutch(&self) -> f32 {
        self.clutch
    }
    pub fn engine_on(&self) -> bool {
        self.engine_on
    }
}
