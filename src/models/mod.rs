#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrafficNetwork {
    pub roads: Vec<RoadSegment>,
    pub intersections: Vec<Intersection>,
    pub traffic_lights: Vec<TrafficLight>,
    pub entry_points: Vec<EntryPoint>,
    pub exit_points: Vec<ExitPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadSegment {
    pub id: String,
    pub name: String,
    pub start: Point,
    pub end: Point,
    pub length: f64,
    pub lanes: u32,
    pub speed_limit: f64,
    pub capacity: u32,
    pub current_vehicles: Vec<String>,
    pub road_type: RoadType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intersection {
    pub id: String,
    pub position: Point,
    pub roads_connected: Vec<String>,
    pub traffic_light_id: Option<String>,
    pub priority_rules: PriorityRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficLight {
    pub id: String,
    pub intersection_id: String,
    pub phases: Vec<LightPhase>,
    pub current_phase: usize,
    pub cycle_duration: f64,
    pub timer: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightPhase {
    pub duration: f64,
    pub road_directions: HashMap<String, LightState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]  // Добавлен PartialEq
pub enum LightState {
    Red,
    Yellow,
    Green,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    pub id: String,
    pub position: Point,
    pub road_id: String,
    pub spawn_rate: f64,
    pub vehicle_types: Vec<VehicleTypeDistribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitPoint {
    pub id: String,
    pub position: Point,
    pub road_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub vehicle_type: VehicleType,
    pub position: Point,
    pub speed: f64,
    pub target_speed: f64,
    pub route: Vec<String>,
    pub current_road: String,
    pub distance_traveled: f64,
    pub waiting_time: f64,
    pub progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VehicleType {
    Car,
    Truck,
    Bus,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoadType {
    Highway,
    Arterial,
    Collector,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityRules {
    pub main_road: Option<String>,
    pub yield_signs: Vec<String>,
    pub stop_signs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleTypeDistribution {
    pub vehicle_type: VehicleType,
    pub probability: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CongestionLevel {
    Free,
    Moderate,
    Heavy,
    Gridlock,
}

impl RoadSegment {
    pub fn congestion_level(&self) -> CongestionLevel {
        let occupancy = self.current_vehicles.len() as f64 / self.capacity as f64;
        match occupancy {
            x if x < 0.3 => CongestionLevel::Free,
            x if x < 0.6 => CongestionLevel::Moderate,
            x if x < 0.9 => CongestionLevel::Heavy,
            _ => CongestionLevel::Gridlock,
        }
    }
    
    pub fn current_speed(&self) -> f64 {
        let base_speed = self.speed_limit;
        match self.congestion_level() {
            CongestionLevel::Free => base_speed,
            CongestionLevel::Moderate => base_speed * 0.7,
            CongestionLevel::Heavy => base_speed * 0.4,
            CongestionLevel::Gridlock => base_speed * 0.1,
        }
    }
}