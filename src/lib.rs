use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod models;
mod simulation;
mod traffic_network;
mod statistics;
mod scenarios;
mod validators;

use models::*;
use simulation::*;
use traffic_network::*;

// Глобальное состояние симуляции
thread_local! {
    static SIMULATION: Rc<RefCell<Option<SimulationEngine>>> = Rc::new(RefCell::new(None));
    static NETWORK: Rc<RefCell<Option<TrafficNetwork>>> = Rc::new(RefCell::new(None));
}

#[wasm_bindgen]
pub struct TrafficSimulation {
    engine: SimulationEngine,
    network: TrafficNetwork,
}

#[wasm_bindgen]
impl TrafficSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TrafficSimulation {
        console_error_panic_hook::set_once();
        let network = TrafficNetwork::create_demo_network();
        let engine = SimulationEngine::new(network.clone());
        
        TrafficSimulation { engine, network }
    }
    
    pub fn step(&mut self) -> JsValue {
        let result = self.engine.step();
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    }
    
    pub fn get_statistics(&self) -> JsValue {
        let stats = self.engine.calculate_statistics();
        serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
    }
    
    pub fn get_network_data(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.network).unwrap_or(JsValue::NULL)
    }
    
    pub fn set_traffic_light_phase(&mut self, light_id: &str, phase: usize) {
        if let Some(light) = self.network.traffic_lights.iter_mut()
            .find(|l| l.id == light_id) {
            light.current_phase = phase;
        }
    }
    
    pub fn set_spawn_rate(&mut self, entry_id: &str, rate: f64) {
        if let Some(entry) = self.network.entry_points.iter_mut()
            .find(|e| e.id == entry_id) {
            entry.spawn_rate = rate.clamp(0.0, 1.0);
        }
    }
    
    pub fn load_scenario(&mut self, scenario_name: &str) -> Result<(), JsValue> {
        let scenarios = scenarios::get_demo_scenarios();
        if let Some((_, scenario)) = scenarios.into_iter().find(|(name, _)| name == scenario_name) {
            scenario.apply(&mut self.network).map_err(|e| JsValue::from_str(&e))?;
            Ok(())
        } else {
            Err(JsValue::from_str("Scenario not found"))
        }
    }
    
    pub fn reset(&mut self) {
        self.network = TrafficNetwork::create_demo_network();
        self.engine = SimulationEngine::new(self.network.clone());
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}