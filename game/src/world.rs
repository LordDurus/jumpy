use crate::vector2::MyVector2;
use std::collections::HashMap;

pub type EntityID = u32;

pub struct World {
    pub positions: HashMap<EntityID, MyVector2>, // Position components
    pub velocities: HashMap<EntityID, MyVector2>, // Velocity components
    next_entity_id: EntityID,                    // Keeps track of the next available entity ID
}

impl World {
    // Create a new, empty world
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            velocities: HashMap::new(),
            next_entity_id: 1,
        }
    }

    pub fn remove_entity(&mut self, entity_id: u32) {
        self.positions.remove(&entity_id);
        self.velocities.remove(&entity_id);
    }
    pub fn add_entity(&mut self, position: MyVector2, velocity: MyVector2) -> u32 {
        let entity_id = self.create_entity();
        self.positions.insert(entity_id, position);
        self.velocities.insert(entity_id, velocity);
        return entity_id;
    }

    #[allow(dead_code)]
    // Create a new entity and return its ID
    pub fn create_entity(&mut self) -> EntityID {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }

    #[allow(dead_code)]
    // Remove an entity and its components
    pub fn destroy_entity(&mut self, id: EntityID) {
        self.positions.remove(&id);
        self.velocities.remove(&id);
    }
}
