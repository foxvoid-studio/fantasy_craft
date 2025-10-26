use hecs::{Entity, World};
use macroquad::math::Vec2;

#[derive(Debug)]
pub struct Parent(pub Entity);

pub fn find_children(world: &World, parent_id: Entity) -> Vec<Entity> {
    world.query::<&Parent>()
        .iter()
        .filter_map(|(child_entity, parent_component)| {
            if parent_component.0 == parent_id {
                Some(child_entity)
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LocalOffset(pub Vec2);
