use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Report {
    pub timestamp: String,
    pub total_vehicles_processed: u64,
    pub average_travel_time: f64,
    pub average_speed: f64,
    pub segments_with_highest_load: Vec<(String, f64)>,
    pub congestion_events: Vec<CongestionEvent>,
    pub scenario_comparisons: HashMap<String, ScenarioStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongestionEvent {
    pub start_time: f64,
    pub end_time: f64,
    pub location: String,
    pub severity: f64,
    pub vehicles_affected: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioStats {
    pub total_vehicles: u64,
    pub avg_travel_time: f64,
    pub avg_speed: f64,
    pub total_congestion_time: f64,
}

impl Report {
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Traffic Simulation Report ===\n");
        report.push_str(&format!("Generated: {}\n", self.timestamp));
        report.push_str(&format!("Total vehicles processed: {}\n", self.total_vehicles_processed));
        report.push_str(&format!("Average travel time: {:.2} seconds\n", self.average_travel_time));
        report.push_str(&format!("Average speed: {:.2} km/h\n", self.average_speed));
        
        if !self.segments_with_highest_load.is_empty() {
            report.push_str("\nMost congested segments:\n");
            for (segment, load) in &self.segments_with_highest_load {
                report.push_str(&format!("  - {}: {:.1}% load\n", segment, load));
            }
        }
        
        if !self.congestion_events.is_empty() {
            report.push_str("\nCongestion events:\n");
            for event in &self.congestion_events {
                report.push_str(&format!(
                    "  - {}: {:.0}s to {:.0}s, severity {:.1}%, {} vehicles affected\n",
                    event.location, event.start_time, event.end_time, 
                    event.severity, event.vehicles_affected
                ));
            }
        }
        
        if !self.scenario_comparisons.is_empty() {
            report.push_str("\nScenario comparisons:\n");
            for (scenario, stats) in &self.scenario_comparisons {
                report.push_str(&format!("  {}:\n", scenario));
                report.push_str(&format!("    Vehicles: {}\n", stats.total_vehicles));
                report.push_str(&format!("    Avg travel time: {:.2}s\n", stats.avg_travel_time));
                report.push_str(&format!("    Avg speed: {:.2} km/h\n", stats.avg_speed));
                report.push_str(&format!("    Total congestion time: {:.2}s\n", stats.total_congestion_time));
            }
        }
        
        report
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]  // Добавлены Serialize, Deserialize
pub struct SimulationStatistics {
    pub total_vehicles: u32,
    pub average_speed: f64,
    pub max_congestion: f64,
    pub average_wait_time: f64,
    pub throughput: f64,
    pub most_congested_roads: Vec<(String, f64)>,
    pub current_vehicles: u32,
}

impl SimulationStatistics {
    pub fn reset(&mut self) {
        self.total_vehicles = 0;
        self.average_speed = 0.0;
        self.max_congestion = 0.0;
        self.average_wait_time = 0.0;
        self.throughput = 0.0;
        self.most_congested_roads.clear();
        self.current_vehicles = 0;
    }
}