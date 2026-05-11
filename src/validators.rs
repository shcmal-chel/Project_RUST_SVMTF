use crate::models::*;
use std::collections::HashSet;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid road geometry: {0}")]
    InvalidGeometry(String),
    
    #[error("Duplicate ID: {0}")]
    DuplicateId(String),
    
    #[error("Referenced object not found: {0}")]
    ReferenceNotFound(String),
    
    #[error("Invalid capacity value: {0}")]
    InvalidCapacity(String),
    
    #[error("Invalid spawn rate: {0}")]
    InvalidSpawnRate(String),
}

pub struct NetworkValidator;

impl NetworkValidator {
    pub fn validate(network: &TrafficNetwork) -> Result<(), ValidationError> {
        Self::check_unique_ids(network)?;
        Self::check_references(network)?;
        Self::check_road_connections(network)?;
        Self::check_geometry(network)?;
        Self::check_parameters(network)?;
        Ok(())
    }
    
    fn check_unique_ids(network: &TrafficNetwork) -> Result<(), ValidationError> {
        let mut ids = HashSet::new();
        
        for road in &network.roads {
            if !ids.insert(&road.id) {
                return Err(ValidationError::DuplicateId(road.id.clone()));
            }
        }
        
        for intersection in &network.intersections {
            if !ids.insert(&intersection.id) {
                return Err(ValidationError::DuplicateId(intersection.id.clone()));
            }
        }
        
        Ok(())
    }
    
    fn check_references(network: &TrafficNetwork) -> Result<(), ValidationError> {
        for intersection in &network.intersections {
            for road_id in &intersection.roads_connected {
                if !network.roads.iter().any(|r| &r.id == road_id) {
                    return Err(ValidationError::ReferenceNotFound(
                        format!("Road {} referenced in intersection {}", road_id, intersection.id)
                    ));
                }
            }
        }
        Ok(())
    }
    
    fn check_road_connections(network: &TrafficNetwork) -> Result<(), ValidationError> {
        for road in &network.roads {
            let start_connected = network.intersections.iter()
                .any(|i| i.roads_connected.contains(&road.id));
            
            let end_connected = network.intersections.iter()
                .any(|i| i.roads_connected.contains(&road.id));
            
            if !start_connected && !end_connected && network.entry_points.is_empty() {
                return Err(ValidationError::InvalidGeometry(
                    format!("Road {} has no connections", road.id)
                ));
            }
        }
        Ok(())
    }
    
    fn check_geometry(network: &TrafficNetwork) -> Result<(), ValidationError> {
        for road in &network.roads {
            let distance = ((road.end.x - road.start.x).powi(2) + 
                           (road.end.y - road.start.y).powi(2)).sqrt();
            
            if distance <= 0.0 {
                return Err(ValidationError::InvalidGeometry(
                    format!("Road {} has zero length", road.id)
                ));
            }
        }
        Ok(())
    }
    
    fn check_parameters(network: &TrafficNetwork) -> Result<(), ValidationError> {
        for road in &network.roads {
            if road.capacity == 0 {
                return Err(ValidationError::InvalidCapacity(
                    format!("Road {} has zero capacity", road.id)
                ));
            }
            
            if road.speed_limit <= 0.0 {
                return Err(ValidationError::InvalidCapacity(
                    format!("Road {} has invalid speed limit", road.id)
                ));
            }
        }
        
        for entry in &network.entry_points {
            if entry.spawn_rate < 0.0 || entry.spawn_rate > 1.0 {
                return Err(ValidationError::InvalidSpawnRate(
                    format!("Entry {} has invalid spawn rate", entry.id)
                ));
            }
        }
        
        Ok(())
    }
}