#![allow(dead_code)]

use crate::models::*;
use rand::Rng;
use std::collections::VecDeque;

pub use crate::statistics::SimulationStatistics;
use serde::{Serialize, Deserialize};

pub struct SimulationEngine {
    network: TrafficNetwork,
    time_step: f64,
    current_time: f64,
    pub vehicle_queue: VecDeque<Vehicle>,
}

impl SimulationEngine {
    pub fn new(network: TrafficNetwork) -> Self {
        Self {
            network,
            time_step: 0.1,
            current_time: 0.0,
            vehicle_queue: VecDeque::new(),
        }
    }
    
    pub fn step(&mut self) -> SimulationResult {
        self.current_time += self.time_step;
        
        self.update_traffic_lights();
        let updates = self.update_vehicle_positions();
        let new_vehicles = self.spawn_vehicles();
        let conflicts = self.resolve_conflicts();
        
        SimulationResult {
            time: self.current_time,
            vehicle_updates: updates,
            new_vehicles,
            conflicts,
            statistics: self.calculate_statistics(),
        }
    }
    
    fn update_traffic_lights(&mut self) {
        for light in &mut self.network.traffic_lights {
            light.timer += self.time_step;
            if light.timer >= light.phases[light.current_phase].duration {
                light.timer = 0.0;
                light.current_phase = (light.current_phase + 1) % light.phases.len();
            }
        }
    }
    
    fn update_vehicle_positions(&mut self) -> Vec<VehicleUpdate> {
        Vec::new()
    }
    
    fn spawn_vehicles(&mut self) -> Vec<Vehicle> {
        let mut new_vehicles = Vec::new();
        let mut rng = rand::thread_rng();
        
        for entry in &self.network.entry_points {
            if rng.gen::<f64>() < entry.spawn_rate * self.time_step {
                if let Some(vehicle) = self.create_vehicle(entry) {
                    new_vehicles.push(vehicle);
                }
            }
        }
        
        new_vehicles
    }
    
    fn create_vehicle(&self, entry: &EntryPoint) -> Option<Vehicle> {
        let mut rng = rand::thread_rng();
        let rand_val: f64 = rng.gen();
        let mut cumulative = 0.0;
        
        for dist in &entry.vehicle_types {
            cumulative += dist.probability;
            if rand_val <= cumulative {
                return Some(Vehicle {
                    id: uuid::Uuid::new_v4().to_string(),
                    vehicle_type: dist.vehicle_type.clone(),
                    position: entry.position.clone(),
                    speed: 0.0,
                    target_speed: 50.0,
                    route: vec![entry.road_id.clone()],
                    current_road: entry.road_id.clone(),
                    distance_traveled: 0.0,
                    waiting_time: 0.0,
                });
            }
        }
        
        None
    }
    
    fn resolve_conflicts(&self) -> Vec<Conflict> {
        Vec::new()
    }

    fn calculate_statistics(&self) -> crate::statistics::SimulationStatistics {
        crate::statistics::SimulationStatistics::default()
    }

    pub fn get_statistics(&self) -> SimulationStatistics {
        self.calculate_statistics()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SimulationResult {
    pub time: f64,
    pub vehicle_updates: Vec<VehicleUpdate>,
    pub new_vehicles: Vec<Vehicle>,
    pub conflicts: Vec<Conflict>,
    pub statistics: crate::statistics::SimulationStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleUpdate {
    pub vehicle_id: String,
    pub new_position: Point,
    pub new_speed: f64,
    pub current_road: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub location: Point,
    pub vehicles_involved: Vec<String>,
    pub conflict_type: ConflictType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    Collision,
    NearMiss,
    TrafficViolation,
}