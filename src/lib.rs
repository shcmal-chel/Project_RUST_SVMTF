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
    light_cycle: f64,
    light_green1: f64,
    light_green2: f64,
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
            light_cycle: 70.0,
            light_green1: 35.0,
            light_green2: 25.0,
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
    
    fn get_state(&self) -> SimulationState {
        let vehicle_data: Vec<VehicleData> = self.vehicles.iter()
            .map(|v| {
                let road_name = if v.current_road == "road_1" { "West-East Road".to_string() }
                               else { "North-South Road".to_string() };
                VehicleData {
                    id: v.id.parse().unwrap_or(0),
                    vehicle_type: format!("{:?}", v.vehicle_type),
                    x: v.position.x,
                    y: v.position.y,
                    progress: (v.distance_traveled / 80.0 * 100.0).min(100.0),
                    current_road: road_name,
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
                let color = if congestion < 30.0 { "green".to_string() }
                           else if congestion < 60.0 { "yellow".to_string() }
                           else { "red".to_string() };
                RoadData {
                    id: r.id.clone(),
                    name: r.name.clone(),
                    congestion,
                    color,
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
        self.light_cycle = 70.0;
        self.light_green1 = 35.0;
        self.light_green2 = 25.0;
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
                self.light_cycle = 70.0;
                self.light_green1 = 35.0;
                self.light_green2 = 25.0;
                for entry in &mut self.network.entry_points {
                    entry.spawn_rate = 0.3;
                }
            }
            1 => {
                self.scenario_name = "Увеличение интенсивности".to_string();
                self.light_cycle = 70.0;
                self.light_green1 = 35.0;
                self.light_green2 = 25.0;
                for entry in &mut self.network.entry_points {
                    entry.spawn_rate = 0.9;
                }
            }
            2 => {
                self.scenario_name = "Перекрытие дороги".to_string();
                self.light_cycle = 70.0;
                self.light_green1 = 35.0;
                self.light_green2 = 25.0;
                for entry in &mut self.network.entry_points {
                    if entry.road_id == "road_1" {
                        entry.spawn_rate = 0.0;
                    } else {
                        entry.spawn_rate = 0.3;
                    }
                }
                for road in &mut self.network.roads {
                    if road.id == "road_1" {
                        road.capacity = 0;
                    }
                }
            }
            3 => {
                self.scenario_name = "Оптимизация светофоров".to_string();
                self.light_cycle = 41.0;
                self.light_green1 = 20.0;
                self.light_green2 = 15.0;
                for entry in &mut self.network.entry_points {
                    entry.spawn_rate = 0.4;
                }
            }
            _ => {}
        }
    }
    
    pub fn get_scenario_name(&self) -> String {
        self.scenario_name.clone()
    }
    
    fn update_vehicles(&mut self) {
        let mut to_remove = Vec::new();
        
        let cycle = self.light_cycle;
        let green1 = self.light_green1;
        let time_in_cycle = self.current_time % cycle;
        let is_road1_green = time_in_cycle < green1;
        
        for (idx, vehicle) in self.vehicles.iter_mut().enumerate() {
            let road = self.network.roads.iter().find(|r| r.id == vehicle.current_road);
            
            if let Some(road) = road {
                let progress = vehicle.distance_traveled / road.length;
                
                let stop_start = 0.40;
                let stop_end = 0.45;
                let is_at_stop_zone = progress >= stop_start && progress < stop_end;
                
                let should_stop = if vehicle.current_road == "road_1" {
                    !is_road1_green && is_at_stop_zone
                } else {
                    is_road1_green && is_at_stop_zone
                };
                
                if should_stop {
                    continue;
                }
                
                let step = vehicle.target_speed * self.simulation_speed * 0.05;
                vehicle.distance_traveled += step;
                let t = (vehicle.distance_traveled / road.length).min(1.0);
                
                if vehicle.current_road == "road_1" {
                    vehicle.position.x = road.start.x + (road.end.x - road.start.x) * t;
                    vehicle.position.y = road.start.y;
                } 
                else if vehicle.current_road == "road_2" {
                    vehicle.position.x = road.start.x;
                    vehicle.position.y = road.start.y + (road.end.y - road.start.y) * t;
                }
                
                if t >= 1.0 {
                    to_remove.push(idx);
                }
            }
        }
        
        for idx in to_remove.into_iter().rev() {
            self.vehicles.remove(idx);
        }
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