use hecs::Entity;

#[derive(Debug)]
pub struct CollisionEvents(pub Vec<CollisionEvent>);

#[derive(Debug, Clone, Copy)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity
}
