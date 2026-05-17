#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


pub use crate::models::{
    TrafficNetwork, RoadSegment, Intersection, TrafficLight, EntryPoint, ExitPoint,
    Point, RoadType, PriorityRules, LightPhase, LightState, Vehicle, VehicleType,
    VehicleTypeDistribution
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

impl TrafficNetwork {
    pub fn new() -> Self {
        Self {
            roads: Vec::new(),
            intersections: Vec::new(),
            traffic_lights: Vec::new(),
            entry_points: Vec::new(),
            exit_points: Vec::new(),
        }
    }
    
    pub fn create_demo_network() -> Self {
    let mut network = TrafficNetwork::new();
    
    // Дорога 1: Запад → Восток (10,50) → (90,50)
    let road1 = RoadSegment {
        id: "road_1".to_string(),
        name: "West-East Road".to_string(),
        start: Point { x: 10.0, y: 50.0 },
        end: Point { x: 90.0, y: 50.0 },
        length: 80.0,
        lanes: 2,
        speed_limit: 50.0,
        capacity: 100,
        current_vehicles: Vec::new(),
        road_type: RoadType::Arterial,
    };
    
    // Дорога 2: Север → Юг (50,10) → (50,90)
    let road2 = RoadSegment {
        id: "road_2".to_string(),
        name: "North-South Road".to_string(),
        start: Point { x: 50.0, y: 10.0 },
        end: Point { x: 50.0, y: 90.0 },
        length: 80.0,
        lanes: 2,
        speed_limit: 50.0,
        capacity: 100,
        current_vehicles: Vec::new(),
        road_type: RoadType::Arterial,
    };
    
    network.roads.push(road1);
    network.roads.push(road2);
    
    // Перекресток в центре (50,50)
    let intersection = Intersection {
        id: "cross_1".to_string(),
        position: Point { x: 50.0, y: 50.0 },
        roads_connected: vec!["road_1".to_string(), "road_2".to_string()],
        traffic_light_id: Some("light_1".to_string()),
        priority_rules: PriorityRules {
            main_road: Some("road_1".to_string()),
            yield_signs: vec![],
            stop_signs: vec![],
        },
    };
    
    network.intersections.push(intersection);
    
    // Светофор
    let traffic_light = TrafficLight {
        id: "light_1".to_string(),
        intersection_id: "cross_1".to_string(),
        phases: vec![
            LightPhase {
                duration: 35.0,
                road_directions: [
                    ("road_1".to_string(), LightState::Green),
                    ("road_2".to_string(), LightState::Red),
                ].iter().cloned().collect(),
            },
            LightPhase {
                duration: 5.0,
                road_directions: [
                    ("road_1".to_string(), LightState::Yellow),
                    ("road_2".to_string(), LightState::Red),
                ].iter().cloned().collect(),
            },
            LightPhase {
                duration: 25.0,
                road_directions: [
                    ("road_1".to_string(), LightState::Red),
                    ("road_2".to_string(), LightState::Green),
                ].iter().cloned().collect(),
            },
            LightPhase {
                duration: 5.0,
                road_directions: [
                    ("road_1".to_string(), LightState::Red),
                    ("road_2".to_string(), LightState::Yellow),
                ].iter().cloned().collect(),
            },
        ],
        current_phase: 0,
        cycle_duration: 70.0,
        timer: 0.0,
    };
    
    network.traffic_lights.push(traffic_light);
    
    // Западный въезд (начало road_1)
    let entry_west = EntryPoint {
        id: "entry_west".to_string(),
        position: Point { x: 10.0, y: 50.0 },
        road_id: "road_1".to_string(),
        spawn_rate: 0.3,
        vehicle_types: vec![
            VehicleTypeDistribution { vehicle_type: VehicleType::Car, probability: 0.7 },
            VehicleTypeDistribution { vehicle_type: VehicleType::Truck, probability: 0.2 },
            VehicleTypeDistribution { vehicle_type: VehicleType::Bus, probability: 0.1 },
        ],
    };
    
    // Северный въезд (начало road_2)
    let entry_north = EntryPoint {
        id: "entry_north".to_string(),
        position: Point { x: 50.0, y: 10.0 },
        road_id: "road_2".to_string(),
        spawn_rate: 0.3,
        vehicle_types: vec![
            VehicleTypeDistribution { vehicle_type: VehicleType::Car, probability: 0.7 },
            VehicleTypeDistribution { vehicle_type: VehicleType::Truck, probability: 0.2 },
            VehicleTypeDistribution { vehicle_type: VehicleType::Bus, probability: 0.1 },
        ],
    };
    
    network.entry_points.push(entry_west);
    network.entry_points.push(entry_north);
    
    // Восточный выезд
    let exit_east = ExitPoint {
        id: "exit_east".to_string(),
        position: Point { x: 90.0, y: 50.0 },
        road_id: "road_1".to_string(),
    };
    
    // Южный выезд
    let exit_south = ExitPoint {
        id: "exit_south".to_string(),
        position: Point { x: 50.0, y: 90.0 },
        road_id: "road_2".to_string(),
    };
    
    network.exit_points.push(exit_east);
    network.exit_points.push(exit_south);
    
    network
}
    
    pub fn spawn_vehicle(&self) -> Option<Vehicle> {
        None
    }
    
    pub fn update_congestion(&mut self) {}
}