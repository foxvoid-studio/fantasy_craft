use hecs::Entity;
use macroquad::prelude::*;
use parry2d::query;

use crate::physics::components::Transform;
use crate::core::context::Context;
use crate::physics::components::{BodyType, Collider, RigidBody, Velocity, Speed};
use crate::physics::helpers::make_isometry;
use crate::prelude::{CollisionEvent, CollisionEvents};

pub fn movement_system(ctx: &mut Context) {
    // Obtenez dt AVANT la boucle
    let dt = ctx.dt(); 

    // query_mut() est plus idiomatique que query::<...>().iter() pour des &mut
    for (_, (transform, velocity, speed)) in ctx.world.query_mut::<(&mut Transform, &mut Velocity, &Speed)>() {
        if velocity.0.length() > 0.0 {
            velocity.0 = velocity.0.normalize();
        }
        // Utilisez la variable dt locale
        transform.position += velocity.0 * speed.0 * dt;
    }
}

pub fn physics_system(ctx: &mut Context) {
    
    // --- Phase 1: Préparation et "Emprunts Disjoints" ---
    
    // Obtenez dt UNE SEULE FOIS.
    let dt = ctx.dt(); 
    
    // Séparez ctx en ses deux parties pour que le compilateur comprenne
    // que 'world' et 'resources' sont indépendants.
    let (world, resources) = (&mut ctx.world, &mut ctx.resources);

    // Obtenez les événements. Emprunte 'resources' SEULEMENT.
    let collision_events = &mut resources.get_mut::<CollisionEvents>()
        .expect("La ressource CollisionEvents est manquante")
        .0;

    // --- Phase 2: Collecte (Lecture seule de 'world') ---
    
    // Ce Vec contiendra des DONNÉES POSSÉDÉES (clonées), pas des références.
    // Notez : plus de &mut RigidBody !
    let mut entities: Vec<(Entity, Vec2, RigidBody, Velocity, Collider)> = Vec::new();

    for (entity, (transform, rigidbody, velocity, collider)) in world.query::<(&Transform, &RigidBody, &Velocity, &Collider)>().iter() {
        // CLONEZ les données. C'est la correction principale.
        entities.push((
            entity,
            transform.position, // Vec2 est Copy
            rigidbody.clone(),  // Nécessite #[derive(Clone)]
            velocity.clone(),   // Nécessite #[derive(Clone)] (ou Copy)
            collider.clone()    // Nécessite #[derive(Clone)]
        ));
    }
    // L'emprunt de 'query' sur 'world' est libéré EXACTEMENT ICI.
    // 'world' est maintenant 100% libre.

    
    // --- Phase 3: Simulation (sur le Vec local) ---
    
    // Étape 1: Intégration du mouvement
    for (_, position, rb, velocity, _) in entities.iter_mut() {
        if let BodyType::Dynamic = rb.body_type {
            *position += velocity.0 * dt; // Utilisez la variable dt locale
        }
    }

    // Étape 2: Résolution des collisions (votre boucle 'split_at_mut' est une bonne approche)
    let mut i = 0;
    while i < entities.len() {
        let (left, right) = entities.split_at_mut(i + 1);
        let entity_a = &mut left[i]; // 'A'
        
        for entity_b in right.iter_mut() { // 'B'
            
            let iso_a = make_isometry(entity_a.1); // .1 = position
            let iso_b = make_isometry(entity_b.1);

            if let Ok(Some(contact)) = query::contact(&iso_a, &*entity_a.4.shape, &iso_b, &*entity_b.4.shape, 0.0) { // .4 = collider
                let normal_vector = contact.normal1.into_inner();
                let half_correction = normal_vector * contact.dist * 0.5;

                // .2 = rigidbody
                if matches!(entity_a.2.body_type, BodyType::Dynamic) && matches!(entity_b.2.body_type, BodyType::Static) {
                    let correction = normal_vector * contact.dist;
                    entity_a.1 += vec2(correction.x, correction.y); 
                } 
                else if matches!(entity_b.2.body_type, BodyType::Dynamic) && matches!(entity_a.2.body_type, BodyType::Static) {
                    let correction = -normal_vector * contact.dist;
                    entity_b.1 += vec2(correction.x, correction.y);
                }
                else if matches!(entity_a.2.body_type, BodyType::Dynamic) && matches!(entity_b.2.body_type, BodyType::Dynamic) {
                    entity_a.1 += vec2(half_correction.x, half_correction.y);
                    entity_b.1 -= vec2(half_correction.x, half_correction.y);
                }

                // C'est OK, 'collision_events' n'emprunte que 'resources'
                collision_events.push(CollisionEvent {
                    entity_a: entity_a.0, // .0 = entity
                    entity_b: entity_b.0
                });
            }
        }
        i += 1;
    }
    // L'emprunt de 'collision_events' sur 'resources' se termine ici.

    // --- Phase 4: Écriture (dans 'world') ---
    
    // 'world' est libre, nous pouvons maintenant le ré-emprunter sans crainte.
    for (entity, new_pos, _, _, _) in entities {
        if let Ok(mut transform) = world.get::<&mut Transform>(entity) {
            transform.position = new_pos;
        }
    }
}

pub fn collider_debug_render_system(ctx: &mut Context) {
    for (_, (transform, collider)) in ctx.world.query::<(&Transform, &Collider)>().iter() {
        let position = transform.position;
        let half_extents = collider.half_extents;

        let width = half_extents.x * 2.0;
        let height = half_extents.y * 2.0;

        let x = position.x - half_extents.x;
        let y = position.y - half_extents.y;

        draw_rectangle_lines(x, y, width, height, 2.0, RED);
    }
}
