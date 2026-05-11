use crate::models::{
    TrafficNetwork, RoadSegment, Intersection, TrafficLight, EntryPoint, ExitPoint,
    Point, RoadType, PriorityRules, LightPhase, LightState, Vehicle, VehicleType,
    VehicleTypeDistribution, TrafficNetworkData // Если вы решите вынести Data структуры в models
};
use crate::validators::NetworkValidator;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

// Примечание: Структуры *Data (TrafficNetworkData и т.д.) лучше также вынести в models.rs 
// или оставить здесь, если они используются только для сериализации в этом модуле.
// Для чистоты кода я оставлю их здесь, но убедитесь, что они не конфликтуют.

#[derive(Debug, Serialize, Deserialize)]
struct TrafficNetworkData {
    roads: Vec<RoadSegmentData>,
    intersections: Vec<IntersectionData>,
    traffic_lights: Vec<TrafficLightData>,
    entry_points: Vec<EntryPointData>,
    exit_points: Vec<ExitPointData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RoadSegmentData {
    id: String,
    name: String,
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    length: f64,
    lanes: u32,
    speed_limit: f64,
    capacity: u32,
    road_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IntersectionData {
    id: String,
    x: f64,
    y: f64,
    roads_connected: Vec<String>,
    traffic_light_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrafficLightData {
    id: String,
    intersection_id: String,
    phases: Vec<LightPhaseData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LightPhaseData {
    duration: f64,
    road_states: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EntryPointData {
    id: String,
    x: f64,
    y: f64,
    road_id: String,
    spawn_rate: f64,
    vehicle_types: Vec<VehicleTypeDistData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VehicleTypeDistData {
    vehicle_type: String,
    probability: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExitPointData {
    id: String,
    x: f64,
    y: f64,
    road_id: String,
}

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
    
    // Загрузка сети из файла
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, NetworkLoadError> {
        let content = fs::read_to_string(path)?;
        
        let network_data: TrafficNetworkData = if path.as_ref().extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "yaml" || ext == "yml")
            .unwrap_or(false) 
        {
            serde_yaml::from_str(&content)?
        } else {
            serde_json::from_str(&content)?
        };
        
        let network = network_data.into_network();
        
        // Теперь типы должны совпадать, так как TrafficNetwork импортирован из models
        if let Err(e) = NetworkValidator::validate(&network) {
            return Err(NetworkLoadError::Validation(e));
        }
        
        Ok(network)
    }
    
    // Сохранение сети в файл
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), NetworkSaveError> {
        let data = TrafficNetworkData::from_network(self);
        let content = serde_json::to_string_pretty(&data)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    // Создание демонстрационной сети
    pub fn create_demo_network() -> Self {
        let mut network = TrafficNetwork::new();
        
        // Создаем дороги
        let road1 = RoadSegment {
            id: "road_1".to_string(),
            name: "Main Street East".to_string(),
            start: Point { x: 10.0, y: 50.0 },
            end: Point { x: 40.0, y: 50.0 },
            length: 30.0,
            lanes: 2,
            speed_limit: 50.0,
            capacity: 100,
            current_vehicles: Vec::new(),
            road_type: RoadType::Arterial,
        };
        
        let road2 = RoadSegment {
            id: "road_2".to_string(),
            name: "Main Street West".to_string(),
            start: Point { x: 60.0, y: 50.0 },
            end: Point { x: 90.0, y: 50.0 },
            length: 30.0,
            lanes: 2,
            speed_limit: 50.0,
            capacity: 100,
            current_vehicles: Vec::new(),
            road_type: RoadType::Arterial,
        };
        
        let road3 = RoadSegment {
            id: "road_3".to_string(),
            name: "Cross Street North".to_string(),
            start: Point { x: 50.0, y: 20.0 },
            end: Point { x: 50.0, y: 80.0 },
            length: 60.0,
            lanes: 1,
            speed_limit: 40.0,
            capacity: 50,
            current_vehicles: Vec::new(),
            road_type: RoadType::Collector,
        };
        
        network.roads.push(road1);
        network.roads.push(road2);
        network.roads.push(road3);
        
        // Создаем перекресток
        let intersection = Intersection {
            id: "cross_1".to_string(),
            position: Point { x: 50.0, y: 50.0 },
            roads_connected: vec!["road_1".to_string(), "road_2".to_string(), "road_3".to_string()],
            traffic_light_id: Some("light_1".to_string()),
            priority_rules: PriorityRules {
                main_road: Some("road_1".to_string()),
                yield_signs: vec!["road_3".to_string()],
                stop_signs: Vec::new(),
            },
        };
        
        network.intersections.push(intersection);
        
        // Создаем светофор
        let traffic_light = TrafficLight {
            id: "light_1".to_string(),
            intersection_id: "cross_1".to_string(),
            phases: vec![
                LightPhase {
                    duration: 30.0,
                    road_directions: [
                        ("road_1".to_string(), LightState::Green),
                        ("road_2".to_string(), LightState::Green),
                        ("road_3".to_string(), LightState::Red),
                    ].iter().cloned().collect(),
                },
                LightPhase {
                    duration: 5.0,
                    road_directions: [
                        ("road_1".to_string(), LightState::Yellow),
                        ("road_2".to_string(), LightState::Yellow),
                        ("road_3".to_string(), LightState::Red),
                    ].iter().cloned().collect(),
                },
                LightPhase {
                    duration: 25.0,
                    road_directions: [
                        ("road_1".to_string(), LightState::Red),
                        ("road_2".to_string(), LightState::Red),
                        ("road_3".to_string(), LightState::Green),
                    ].iter().cloned().collect(),
                },
                LightPhase {
                    duration: 5.0,
                    road_directions: [
                        ("road_1".to_string(), LightState::Red),
                        ("road_2".to_string(), LightState::Red),
                        ("road_3".to_string(), LightState::Yellow),
                    ].iter().cloned().collect(),
                },
            ],
            current_phase: 0,
            cycle_duration: 65.0,
            timer: 0.0,
        };
        
        network.traffic_lights.push(traffic_light);
        
        // Создаем точки въезда
        let entry = EntryPoint {
            id: "entry_1".to_string(),
            position: Point { x: 10.0, y: 50.0 },
            road_id: "road_1".to_string(),
            spawn_rate: 0.3,
            vehicle_types: vec![
                VehicleTypeDistribution {
                    vehicle_type: VehicleType::Car,
                    probability: 0.7,
                },
                VehicleTypeDistribution {
                    vehicle_type: VehicleType::Truck,
                    probability: 0.2,
                },
                VehicleTypeDistribution {
                    vehicle_type: VehicleType::Bus,
                    probability: 0.1,
                },
            ],
        };
        
        network.entry_points.push(entry);
        
        // Создаем точки выезда
        let exit = ExitPoint {
            id: "exit_1".to_string(),
            position: Point { x: 90.0, y: 50.0 },
            road_id: "road_2".to_string(),
        };
        
        network.exit_points.push(exit);
        
        network
    }
    
    pub fn spawn_vehicle(&self) -> Option<Vehicle> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        if let Some(entry) = self.entry_points.first() {
            let vehicle_type = Self::select_vehicle_type(&entry.vehicle_types);
            
            Some(Vehicle {
                id: uuid::Uuid::new_v4().to_string(),
                vehicle_type,
                position: entry.position.clone(),
                speed: 0.0,
                target_speed: 50.0,
                route: vec![entry.road_id.clone()],
                current_road: entry.road_id.clone(),
                distance_traveled: 0.0,
                waiting_time: 0.0,
            })
        } else {
            None
        }
    }
    
    fn select_vehicle_type(distributions: &[VehicleTypeDistribution]) -> VehicleType {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let rand_val: f64 = rng.gen();
        let mut cumulative = 0.0;
        
        for dist in distributions {
            cumulative += dist.probability;
            if rand_val <= cumulative {
                return dist.vehicle_type.clone();
            }
        }
        
        VehicleType::Car
    }
    
    pub fn update_congestion(&mut self) {
        for road in &mut self.roads {
            let congestion = road.current_vehicles.len() as f64 / road.capacity as f64;
            if congestion > 0.8 {
                // Логика замедления
            }
        }
    }
}

// Реализация конвертации для Data структур
impl TrafficNetworkData {
    fn into_network(self) -> TrafficNetwork {
        let roads: Vec<RoadSegment> = self.roads.into_iter().map(|r| RoadSegment {
            id: r.id,
            name: r.name,
            start: Point { x: r.start_x, y: r.start_y },
            end: Point { x: r.end_x, y: r.end_y },
            length: r.length,
            lanes: r.lanes,
            speed_limit: r.speed_limit,
            capacity: r.capacity,
            current_vehicles: Vec::new(),
            road_type: match r.road_type.as_str() {
                "highway" => RoadType::Highway,
                "arterial" => RoadType::Arterial,
                "collector" => RoadType::Collector,
                _ => RoadType::Local,
            },
        }).collect();
        
        let intersections: Vec<Intersection> = self.intersections.into_iter().map(|i| Intersection {
            id: i.id,
            position: Point { x: i.x, y: i.y },
            roads_connected: i.roads_connected,
            traffic_light_id: i.traffic_light_id,
            priority_rules: PriorityRules {
                main_road: None,
                yield_signs: Vec::new(),
                stop_signs: Vec::new(),
            },
        }).collect();
        
        let traffic_lights: Vec<TrafficLight> = self.traffic_lights.into_iter().map(|t| {
            let phases: Vec<LightPhase> = t.phases.into_iter().map(|p| {
                let mut road_directions = HashMap::new();
                for (road, state) in p.road_states {
                    let light_state = match state.as_str() {
                        "red" => LightState::Red,
                        "yellow" => LightState::Yellow,
                        "green" => LightState::Green,
                        _ => LightState::Red,
                    };
                    road_directions.insert(road, light_state);
                }
                LightPhase {
                    duration: p.duration,
                    road_directions,
                }
            }).collect();
            
            TrafficLight {
                id: t.id,
                intersection_id: t.intersection_id,
                phases,
                current_phase: 0,
                cycle_duration: 0.0,
                timer: 0.0,
            }
        }).collect();
        
        let entry_points: Vec<EntryPoint> = self.entry_points.into_iter().map(|e| {
            let vehicle_types: Vec<VehicleTypeDistribution> = e.vehicle_types.into_iter().map(|v| {
                let vehicle_type = match v.vehicle_type.as_str() {
                    "car" => VehicleType::Car,
                    "truck" => VehicleType::Truck,
                    "bus" => VehicleType::Bus,
                    "emergency" => VehicleType::Emergency,
                    _ => VehicleType::Car,
                };
                VehicleTypeDistribution {
                    vehicle_type,
                    probability: v.probability,
                }
            }).collect();
            
            EntryPoint {
                id: e.id,
                position: Point { x: e.x, y: e.y },
                road_id: e.road_id,
                spawn_rate: e.spawn_rate,
                vehicle_types,
            }
        }).collect();
        
        let exit_points: Vec<ExitPoint> = self.exit_points.into_iter().map(|e| ExitPoint {
            id: e.id,
            position: Point { x: e.x, y: e.y },
            road_id: e.road_id,
        }).collect();
        
        TrafficNetwork {
            roads,
            intersections,
            traffic_lights,
            entry_points,
            exit_points,
        }
    }
    
    fn from_network(network: &TrafficNetwork) -> Self {
        let roads = network.roads.iter().map(|r| RoadSegmentData {
            id: r.id.clone(),
            name: r.name.clone(),
            start_x: r.start.x,
            start_y: r.start.y,
            end_x: r.end.x,
            end_y: r.end.y,
            length: r.length,
            lanes: r.lanes,
            speed_limit: r.speed_limit,
            capacity: r.capacity,
            road_type: match r.road_type {
                RoadType::Highway => "highway",
                RoadType::Arterial => "arterial",
                RoadType::Collector => "collector",
                RoadType::Local => "local",
            }.to_string(),
        }).collect();
        
        let intersections = network.intersections.iter().map(|i| IntersectionData {
            id: i.id.clone(),
            x: i.position.x,
            y: i.position.y,
            roads_connected: i.roads_connected.clone(),
            traffic_light_id: i.traffic_light_id.clone(),
        }).collect();
        
        let traffic_lights = network.traffic_lights.iter().map(|t| {
            let phases = t.phases.iter().map(|p| {
                let mut road_states = Vec::new();
                for (road, state) in &p.road_directions {
                    let state_str = match state {
                        LightState::Red => "red",
                        LightState::Yellow => "yellow",
                        LightState::Green => "green",
                    };
                    road_states.push((road.clone(), state_str.to_string()));
                }
                LightPhaseData {
                    duration: p.duration,
                    road_states,
                }
            }).collect();
            
            TrafficLightData {
                id: t.id.clone(),
                intersection_id: t.intersection_id.clone(),
                phases,
            }
        }).collect();
        
        let entry_points = network.entry_points.iter().map(|e| {
            let vehicle_types = e.vehicle_types.iter().map(|v| {
                let type_str = match v.vehicle_type {
                    VehicleType::Car => "car",
                    VehicleType::Truck => "truck",
                    VehicleType::Bus => "bus",
                    VehicleType::Emergency => "emergency",
                };
                VehicleTypeDistData {
                    vehicle_type: type_str.to_string(),
                    probability: v.probability,
                }
            }).collect();
            
            EntryPointData {
                id: e.id.clone(),
                x: e.position.x,
                y: e.position.y,
                road_id: e.road_id.clone(),
                spawn_rate: e.spawn_rate,
                vehicle_types,
            }
        }).collect();
        
        let exit_points = network.exit_points.iter().map(|e| ExitPointData {
            id: e.id.clone(),
            x: e.position.x,
            y: e.position.y,
            road_id: e.road_id.clone(),
        }).collect();
        
        Self {
            roads,
            intersections,
            traffic_lights,
            entry_points,
            exit_points,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    
    #[error("Validation error: {0}")]
    Validation(#[from] crate::validators::ValidationError),
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkSaveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}