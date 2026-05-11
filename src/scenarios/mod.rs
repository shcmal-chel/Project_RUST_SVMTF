#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::models::*;


pub enum Scenario {
    BaseFlow,
    IncreasedIntensity,
    RoadClosure,
    TrafficLightChanges,
}

impl Scenario {
    pub fn apply(&self, network: &mut TrafficNetwork) -> Result<(), String> {
        match self {
            Scenario::BaseFlow => {
                // Стандартные параметры
                for entry in &mut network.entry_points {
                    entry.spawn_rate = 0.3;
                }
                for road in &mut network.roads {
                    road.capacity = 100;
                }
                println!("✅ Сценарий: Базовое движение");
                Ok(())
            }
            Scenario::IncreasedIntensity => {
                // Увеличение интенсивности в 3 раза
                for entry in &mut network.entry_points {
                    entry.spawn_rate = 0.9;
                }
                println!("✅ Сценарий: Увеличение интенсивности (поток увеличен в 3 раза)");
                Ok(())
            }
            Scenario::RoadClosure => {
                // Перекрытие центральной дороги
                if let Some(road) = network.roads.iter_mut().find(|r| r.id == "road_1") {
                    road.capacity = 10;
                    println!("✅ Сценарий: Перекрытие дороги Main Street East");
                }
                Ok(())
            }
            Scenario::TrafficLightChanges => {
                // Оптимизация светофоров - меняем фазы
                for light in &mut network.traffic_lights {
                    for phase in &mut light.phases {
                        if phase.road_directions.values().any(|s| *s == LightState::Green) {
                            phase.duration = 20.0; // Уменьшаем время для оптимизации
                        }
                    }
                }
                println!("✅ Сценарий: Оптимизация светофоров");
                Ok(())
            }
        }
    }
}

pub fn get_demo_scenarios() -> Vec<(String, Scenario)> {
    vec![
        ("Базовое движение".to_string(), Scenario::BaseFlow),
        ("Увеличение интенсивности".to_string(), Scenario::IncreasedIntensity),
        ("Перекрытие дороги".to_string(), Scenario::RoadClosure),
        ("Оптимизация светофоров".to_string(), Scenario::TrafficLightChanges),
    ]
}