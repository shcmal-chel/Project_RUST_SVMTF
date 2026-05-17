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
                for entry in &mut network.entry_points {
                    entry.spawn_rate = 0.3;
                }
                for light in &mut network.traffic_lights {
                    light.phases[0].duration = 35.0;
                    light.phases[1].duration = 5.0;
                    light.phases[2].duration = 25.0;
                    light.phases[3].duration = 5.0;
                }
                Ok(())
            }
            Scenario::IncreasedIntensity => {
                for entry in &mut network.entry_points {
                    entry.spawn_rate = 0.9;
                }
                Ok(())
            }
            Scenario::RoadClosure => {
                for entry in &mut network.entry_points {
                    if entry.road_id == "road_1" {
                        entry.spawn_rate = 0.0;
                    }
                }
                Ok(())
            }
            Scenario::TrafficLightChanges => {
                for light in &mut network.traffic_lights {
                    light.phases[0].duration = 20.0;
                    light.phases[1].duration = 3.0;
                    light.phases[2].duration = 15.0;
                    light.phases[3].duration = 3.0;
                }
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