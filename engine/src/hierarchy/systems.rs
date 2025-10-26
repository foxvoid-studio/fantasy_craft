use std::collections::HashMap;

use crate::prelude::{Context, LocalOffset, Parent, Transform};

pub fn hierarchy_update_system(ctx: &mut Context) {
    let mut world_position = HashMap::new();
    for (entity, transform) in ctx.world.query::<&Transform>().iter() {
        world_position.insert(entity, transform.position);
    }

    for (_, (child_transform, parent, local_offset)) in ctx.world.query::<(&mut Transform, &Parent, &LocalOffset)>().iter() {
        if let Some(&parent_world_pos) = world_position.get(&parent.0) {
            child_transform.position = parent_world_pos + local_offset.0;
        }
    }
}
