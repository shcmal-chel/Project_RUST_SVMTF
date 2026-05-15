use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use js_sys::Math;

mod models;
mod traffic_network;
mod scenarios;

use models::*;
use traffic_network::TrafficNetwork;

#[derive(Serialize, Deserialize, Clone)]
pub struct VehicleData {
    pub id: u32,
    pub vehicle_type: String,
    pub x: f64,
    pub y: f64,
    pub progress: f64,
    pub current_road: String,
    pub speed: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoadData {
    pub id: String,
    pub name: String,
    pub congestion: f64,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SimulationState {
    pub vehicles: Vec<VehicleData>,
    pub roads: Vec<RoadData>,
    pub total_vehicles: u32,
    pub current_time: f64,
    pub simulation_speed: f64,
    pub is_running: bool,
    pub is_paused: bool,
    pub avg_speed: f64,
    pub throughput: f64,
    pub zoom: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub scenario_name: String,
}

#[wasm_bindgen]
pub struct TrafficSimulation {
    network: TrafficNetwork,
    vehicles: Vec<Vehicle>,
    total_vehicles: u32,
    current_time: f64,
    simulation_speed: f64,
    is_running: bool,
    is_paused: bool,
    next_id: u32,
    zoom: f64,
    offset_x: f64,
    offset_y: f64,
    scenario_name: String,
}

#[wasm_bindgen]
impl TrafficSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TrafficSimulation {
        console_error_panic_hook::set_once();
        let network = TrafficNetwork::create_demo_network();
        
        TrafficSimulation {
            network,
            vehicles: Vec::new(),
            total_vehicles: 0,
            current_time: 0.0,
            simulation_speed: 0.5,
            is_running: false,
            is_paused: false,
            next_id: 1,
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            scenario_name: "Базовое движение".to_string(),
        }
    }
    
    pub fn step(&mut self) -> JsValue {
        if self.is_running && !self.is_paused {
            self.current_time += 0.05 * self.simulation_speed;
            self.update_vehicles();
            self.spawn_vehicles();
        }
        
        let state = self.get_state();
        serde_wasm_bindgen::to_value(&state).unwrap_or(JsValue::NULL)
    }
    
    pub fn start(&mut self) {
        self.is_running = true;
        self.is_paused = false;
    }
    
    pub fn pause(&mut self) {
        self.is_running = false;
        self.is_paused = true;
    }
    
    pub fn stop(&mut self) {
        self.is_running = false;
        self.is_paused = false;
        self.vehicles.clear();
        self.current_time = 0.0;
        self.total_vehicles = 0;
    }
    
    pub fn reset(&mut self) {
        self.vehicles.clear();
        self.current_time = 0.0;
        self.total_vehicles = 0;
        self.is_running = false;
        self.is_paused = false;
        self.network = TrafficNetwork::create_demo_network();
        self.next_id = 1;
        self.simulation_speed = 0.5;
        self.zoom = 1.0;
        self.offset_x = 0.0;
        self.offset_y = 0.0;
        self.scenario_name = "Базовое движение".to_string();
    }
    
    pub fn set_speed(&mut self, speed: f64) {
        self.simulation_speed = speed.clamp(0.1, 5.0);
    }
    
    pub fn get_speed(&self) -> f64 {
        self.simulation_speed
    }
    
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.2).min(3.0);
    }
    
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.2).max(0.5);
    }
    
    pub fn move_left(&mut self) {
        self.offset_x += 20.0;
    }
    
    pub fn move_right(&mut self) {
        self.offset_x -= 20.0;
    }
    
    pub fn move_up(&mut self) {
        self.offset_y += 20.0;
    }
    
    pub fn move_down(&mut self) {
        self.offset_y -= 20.0;
    }
    
    pub fn load_scenario(&mut self, index: usize) {
        self.network = TrafficNetwork::create_demo_network();
        self.vehicles.clear();
        self.total_vehicles = 0;
        self.current_time = 0.0;
        self.is_running = false;
        self.is_paused = false;
        self.next_id = 1;
        
        match index {
            0 => {
                self.scenario_name = "Базовое движение".to_string();
                for entry in &mut self.network.entry_points {
                    entry.spawn_rate = 0.3;
                }
            }
            1 => {
                self.scenario_name = "Увеличение интенсивности".to_string();
                for entry in &mut self.network.entry_points {
                    entry.spawn_rate = 0.8;
                }
            }
            2 => {
                self.scenario_name = "Перекрытие дороги".to_string();
                for entry in &mut self.network.entry_points {
                    if entry.road_id == "road_1" {
                        entry.spawn_rate = 0.0;
                    } else {
                        entry.spawn_rate = 0.3;
                    }
                }
            }
            3 => {
                self.scenario_name = "Оптимизация светофоров".to_string();
                for entry in &mut self.network.entry_points {
                    entry.spawn_rate = 0.4;
                }
                for light in &mut self.network.traffic_lights {
                    for phase in &mut light.phases {
                        if phase.road_directions.values().any(|s| *s == LightState::Green) {
                            phase.duration = 8.0;
                        }
                    }
                }
            }
            _ => {}
        }
        println!("✅ Сценарий: {}", self.scenario_name);
    }
    
    pub fn get_scenario_name(&self) -> String {
        self.scenario_name.clone()
    }
    
    fn get_state(&self) -> SimulationState {
        let vehicle_data: Vec<VehicleData> = self.vehicles.iter()
            .map(|v| {
                let road_name = if v.current_road == "road_1" { "Main Street East" }
                               else if v.current_road == "road_2" { "Main Street West" }
                               else { "Cross Street" };
                VehicleData {
                    id: v.id.parse().unwrap_or(0),
                    vehicle_type: format!("{:?}", v.vehicle_type),
                    x: v.position.x,
                    y: v.position.y,
                    progress: (v.distance_traveled / 40.0 * 100.0).min(100.0),
                    current_road: road_name.to_string(),
                    speed: v.speed,
                }
            })
            .collect();
        
        let road_data: Vec<RoadData> = self.network.roads.iter()
            .map(|r| {
                let vehicle_count = self.vehicles.iter()
                    .filter(|v| v.current_road == r.id)
                    .count();
                let congestion = (vehicle_count as f64 / r.capacity as f64 * 100.0).min(100.0);
                let color = if congestion < 30.0 { "green" }
                           else if congestion < 60.0 { "yellow" }
                           else { "red" };
                RoadData {
                    id: r.id.clone(),
                    name: r.name.clone(),
                    congestion,
                    color: color.to_string(),
                }
            })
            .collect();
        
        let avg_speed = if !self.vehicles.is_empty() {
            self.vehicles.iter().map(|v| v.speed).sum::<f64>() / self.vehicles.len() as f64
        } else { 0.0 };
        
        let throughput = if self.current_time > 0.0 {
            (self.total_vehicles as f64 / self.current_time) * 60.0
        } else { 0.0 };
        
        SimulationState {
            vehicles: vehicle_data,
            roads: road_data,
            total_vehicles: self.total_vehicles,
            current_time: self.current_time,
            simulation_speed: self.simulation_speed,
            is_running: self.is_running,
            is_paused: self.is_paused,
            avg_speed,
            throughput,
            zoom: self.zoom,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            scenario_name: self.scenario_name.clone(),
        }
    }
    
    fn update_vehicles(&mut self) {
    for vehicle in &mut self.vehicles {
        let step = vehicle.target_speed * self.simulation_speed * 0.05;
        vehicle.distance_traveled += step;
        
        let current_road = vehicle.current_road.clone();
        let road = self.network.roads.iter().find(|r| r.id == current_road);
        
        if let Some(road) = road {
            if vehicle.distance_traveled >= road.length {
                if current_road == "road_1" {
                    vehicle.current_road = "road_1_to_3".to_string();
                    vehicle.distance_traveled = 0.0;
                } 
                else if current_road == "road_1_to_3" {
                    vehicle.current_road = "road_1_to_2".to_string();
                    vehicle.distance_traveled = 0.0;
                } 
                else if current_road == "road_3" {
                    vehicle.current_road = "road_3_to_2".to_string();
                    vehicle.distance_traveled = 0.0;
                } 
                else if current_road == "road_1_to_2" {
                    // Доехал до конца road_1_to_2 - удаляем
                    vehicle.progress = 100.0;
                    continue;
                }
                else if current_road == "road_3_to_2" {
                    // Доехал до конца road_3_to_2 - удаляем
                    vehicle.progress = 100.0;
                    continue;
                }
                else if current_road == "road_2" {
                    vehicle.progress = 100.0;
                    continue;
                }
                continue;
            }
            
            let t = vehicle.distance_traveled / road.length;
            
            if current_road == "road_1_to_3" || current_road == "road_3" {
                vehicle.position.x = road.start.x;
                vehicle.position.y = road.start.y + (road.end.y - road.start.y) * t;
            }
            else if current_road == "road_1_to_2" || current_road == "road_3_to_2" {
                vehicle.position.x = road.start.x + (road.end.x - road.start.x) * t;
                vehicle.position.y = road.start.y;
            }
            else {
                vehicle.position.x = road.start.x + (road.end.x - road.start.x) * t;
                vehicle.position.y = road.start.y;
            }
        }
    }
    
    self.vehicles.retain(|v| v.progress < 100.0);
}
    
    fn spawn_vehicles(&mut self) {
        for entry in &self.network.entry_points {
            if entry.spawn_rate == 0.0 {
                continue;
            }
            
            let chance = entry.spawn_rate * self.simulation_speed / 15.0;
            
            if Math::random() < chance && self.vehicles.len() < 30 {
                let r = Math::random();
                let vehicle_type = if r < 0.7 {
                    VehicleType::Car
                } else if r < 0.9 {
                    VehicleType::Truck
                } else {
                    VehicleType::Bus
                };
                
                let target_speed = match vehicle_type {
                    VehicleType::Car => 50.0,
                    VehicleType::Truck => 35.0,
                    VehicleType::Bus => 30.0,
                    VehicleType::Emergency => 60.0,
                };
                
                let vehicle = Vehicle {
                    id: self.next_id.to_string(),
                    vehicle_type,
                    position: entry.position.clone(),
                    speed: target_speed,
                    target_speed,
                    route: vec![],
                    current_road: entry.road_id.clone(),
                    distance_traveled: 0.0,
                    waiting_time: 0.0,
                    progress: 0.0,
                };
                
                self.vehicles.push(vehicle);
                self.total_vehicles += 1;
                self.next_id += 1;
            }
        }
    }
}