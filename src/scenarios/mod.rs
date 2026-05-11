use crate::models::*;
use crate::traffic_network::TrafficNetwork;

pub enum Scenario {
    BaseFlow,
    IncreasedIntensity(String),
    RoadClosure(String),
    TrafficLightChanges(String, Vec<LightPhase>),
}

impl Scenario {
    pub fn apply(&self, network: &mut TrafficNetwork) -> Result<(), String> {
        match self {
            Scenario::BaseFlow => {
                // Базовая конфигурация
                Ok(())
            }
            Scenario::IncreasedIntensity(road_id) => {
                // Увеличение интенсивности на указанной дороге
                if let Some(road) = network.roads.iter_mut().find(|r| r.id == *road_id) {
                    // Увеличиваем spawn rate на въездах, связанных с этой дорогой
                    for entry in &mut network.entry_points {
                        if entry.road_id == *road_id {
                            entry.spawn_rate *= 3.0;
                        }
                    }
                    Ok(())
                } else {
                    Err(format!("Road {} not found", road_id))
                }
            }
            Scenario::RoadClosure(road_id) => {
                // Перекрытие дороги
                if let Some(road) = network.roads.iter_mut().find(|r| r.id == *road_id) {
                    road.capacity = 0;
                    Ok(())
                } else {
                    Err(format!("Road {} not found", road_id))
                }
            }
            Scenario::TrafficLightChanges(intersection_id, new_phases) => {
                // Изменение режимов работы светофоров
                if let Some(light) = network.traffic_lights.iter_mut()
                    .find(|l| l.intersection_id == *intersection_id) {
                    light.phases = new_phases.clone();
                    Ok(())
                } else {
                    Err(format!("Intersection {} not found", intersection_id))
                }
            }
        }
    }
}

pub fn get_demo_scenarios() -> Vec<(String, Scenario)> {
    vec![
        ("Базовое движение".to_string(), Scenario::BaseFlow),
        ("Увеличение интенсивности на главной магистрали".to_string(), 
         Scenario::IncreasedIntensity("main_road".to_string())),
        ("Перекрытие аварийного участка".to_string(),
         Scenario::RoadClosure("accident_site".to_string())),
        ("Оптимизация работы светофоров".to_string(),
         Scenario::TrafficLightChanges("central_cross".to_string(), vec![])),
    ]
}