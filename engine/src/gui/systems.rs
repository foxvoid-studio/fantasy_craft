// --- IMPORTS ---
use std::collections::{HashMap, HashSet};
use hecs::Entity;
use macroquad::prelude::*;
use crate::core::context::Context; use crate::core::event::EventBus;
use crate::input::focus::InputFocus;
// Your refactored context
use crate::gui::components::{TextDisplay, GuiBox};
use crate::gui::event::UiClickEvent;
use crate::physics::components::Transform;
use crate::prelude::{
    ButtonState, FontComponent, GuiAction, GuiButton, GuiCheckbox, GuiDraggable, GuiElement, GuiImage, GuiInputField, GuiLayout, GuiLocalOffset, GuiSlider, HorizontalAlignment, HorizontalAlignmentType, Parent, VerticalAlignment, VerticalAlignmentType, Visible
};

// --- RESOURCE WRAPPER STRUCT ---
// (This is correct, keep it here or move it to your resources module)
/// Wrapper for the UI rectangle cache.
#[derive(Debug, Default)]
pub struct UiResolvedRects(pub HashMap<Entity, (Vec2, Vec2)>);

#[derive(Debug, Clone, Copy)]
pub struct PreviousMousePosition(pub Vec2);

// --- SYSTEMS ---

pub fn gui_resolve_layout_system(ctx: &mut Context) {
    let (screen_w, screen_h) = (screen_width(), screen_height());
    
    ctx.resource_mut::<UiResolvedRects>().0.clear();

    // ... (Collecte des 'entities' ... c'est correct)
    let mut entities: HashSet<Entity> = ctx.world.query::<(&Parent, &GuiElement)>()
        .iter()
        .map(|(e, _)| e)
        .collect();

    entities.extend(
        ctx.world.query::<&GuiBox>().without::<&Parent>()
            .iter()
            .map(|(e, _)| e)
    );
    let mut entities_to_process: Vec<Entity> = entities.into_iter().collect();
    
    let mut iterations = 0;
    
    while !entities_to_process.is_empty() && iterations < 10 {
        // --- PHASE 1: COLLECTER LES MODIFICATIONS ---
        let mut results_to_apply = Vec::new();
        let mut processed_this_iteration = Vec::new();
        
        // Emprunt immuable de la map pour cette phase
        let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

        entities_to_process.retain(|&entity| {
            let parent_opt = ctx.world.get::<&Parent>(entity).ok();

            let (parent_w, parent_h, parent_pos) = 
                if let Some(parent) = parent_opt.as_ref() { 
                    // Lecture depuis la map immuable
                    if let Some((pos, size)) = resolved_rects_map.get(&parent.0) {
                        (size.x, size.y, *pos)
                    } else {
                        return true; // Garder (parent pas encore traité)
                    }
                } else {
                    // ... (logique de la racine, inchangée)
                    if let Ok(layout) = ctx.world.get::<&GuiLayout>(entity) {
                        let root_x = layout.x.resolve(screen_w);
                        let root_y = layout.y.resolve(screen_h);
                        (screen_w, screen_h, vec2(root_x, root_y))
                    } else {
                        (screen_w, screen_h, Vec2::ZERO)
                    }
                };

            // 1. Résoudre Taille (lecture seule)
            let resolved_size = if let Ok(gui_box) = ctx.world.get::<&GuiBox>(entity) {
                vec2(gui_box.width.resolve(parent_w), gui_box.height.resolve(parent_h))
            } else {
                Vec2::ZERO
            };

            // 2. Résoudre Position (lecture seule)
            let mut resolved_pos;
            if parent_opt.is_some() {
                resolved_pos = parent_pos;
                if let Ok(local_offset) = ctx.world.get::<&GuiLocalOffset>(entity) {
                    resolved_pos.x += local_offset.x.resolve(parent_w);
                    resolved_pos.y += local_offset.y.resolve(parent_h);
                }
            } else {
                 if let Ok(layout) = ctx.world.get::<&GuiLayout>(entity) {
                    resolved_pos = vec2(layout.x.resolve(screen_w), layout.y.resolve(screen_h));
                } else if let Ok(transform) = ctx.world.get::<&Transform>(entity) {
                    resolved_pos = transform.position;
                } else {
                    resolved_pos = Vec2::ZERO;
                }
            }
            
            // 3. Stocker le résultat au lieu de l'insérer
            results_to_apply.push((entity, resolved_pos, resolved_size));
            
            processed_this_iteration.push(entity);
            false // Retirer de entities_to_process
        });

        // --- PHASE 2: APPLIQUER LES MODIFICATIONS ---
        
        // Failsafe
        if processed_this_iteration.is_empty() && !entities_to_process.is_empty() {
            eprintln!("Erreur de layout GUI : impossible de résoudre certaines entités.");
            break; 
        }

        let (_world, resources) = (&mut ctx.world, &mut ctx.resources);

        // 2. Obtenez l'accès mutable à la map DEPUIS 'resources'
        let rect_map_mut = &mut resources.get_mut::<UiResolvedRects>()
            .expect("Ressource UiResolvedRects manquante")
            .0;

        for (entity, pos, size) in results_to_apply {
            // 1. Insérer dans la map des ressources
            rect_map_mut.insert(entity, (pos, size));

            // 2. Mettre à jour le Transform
            let is_dragging = ctx.world.get::<&GuiDraggable>(entity)
                .map_or(false, |d| d.is_dragging);

            if !is_dragging {
                if let Ok(mut transform) = ctx.world.get::<&mut Transform>(entity) {
                    transform.position = pos;
                }
            }
        }
        
        iterations += 1;
    }
}

pub fn button_interaction_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_down(MouseButton::Left);
    let just_clicked = is_mouse_button_pressed(MouseButton::Left);

    let (world, resources) = (&mut ctx.world, &mut ctx.resources);

    // --- READ PHASE ---
    // We get the read-only resource first.
    let resolved_rects_map = &resources.get::<UiResolvedRects>()
        .expect("UiResolvedRects resource is missing")
        .0;

    // We create a local buffer to store events because we can't 
    // borrow EventBus mutably while holding resolved_rects_map.
    let mut events_to_send: Vec<UiClickEvent> = Vec::new();

    let mut query = world.query::<(
        &mut GuiButton, 
        &GuiBox, 
        Option<&GuiAction>, 
        Option<&Visible>, 
        Option<&HorizontalAlignment>, 
        Option<&VerticalAlignment>
    )>();

    for (entity, (button, gui_box, action_opt, visibility, h_align, v_align)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        button.just_clicked = false;

        // We use the read-only map here
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue; 
            };

        if !gui_box.screen_space { continue; }

        let mut x = resolved_pos.x;
        let mut y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;

        // Apply alignment
        if let Some(h_align) = h_align {
            match h_align.0 {
                HorizontalAlignmentType::Left => {},
                HorizontalAlignmentType::Center => x -= w / 2.0,
                HorizontalAlignmentType::Right => x -= w,
            }
        }
        
        if let Some(v_align) = v_align {
            match v_align.0 {
                VerticalAlignmentType::Top => {},
                VerticalAlignmentType::Center => y -= h / 2.0,
                VerticalAlignmentType::Bottom => y -= h,
            }
        }

        let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

        match button.state {
            ButtonState::Idle => {
                if is_hovered { button.state = ButtonState::Hovered; }
            }
            ButtonState::Hovered => {
                if !is_hovered { button.state = ButtonState::Idle; }
                else if just_clicked { button.state = ButtonState::Pressed; }
            }
            ButtonState::Pressed => {
                if !is_pressed {
                    if is_hovered {
                        button.just_clicked = true;
                        button.state = ButtonState::Hovered;

                        // --- COLLECT PHASE ---
                        // Instead of sending immediately, we push to the buffer.
                        if let Some(action) = action_opt {
                            events_to_send.push(UiClickEvent {
                                action_id: action.action_id.clone(),
                                entity,
                            });
                        }
                    } else {
                        button.state = ButtonState::Idle;
                    }
                }
            }
        }
    }

    // --- SEND PHASE ---
    // The loop is done, so `resolved_rects_map` borrow is dropped (or can be inferred dropped).
    // We are now free to borrow `resources` mutably to get the EventBus.
    
    if !events_to_send.is_empty() {
        let event_bus = resources.get_mut::<EventBus>()
            .expect("EventBus resource is missing");

        for event in events_to_send {
            event_bus.send(event);
        }
    }
}

pub fn gui_box_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(
        &GuiBox,
        Option<&GuiButton>,
        Option<&Visible>,
        Option<&HorizontalAlignment>,
        Option<&VerticalAlignment>,
    )>();

    for (entity, (gui_box, button_opt, visibility, h_align, v_align)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible || !gui_box.screen_space {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect // Dereference the tuple (pos, size)
            } else {
                continue; // This UI element was not processed by the layout system
            };

        let mut x = resolved_pos.x;
        let mut y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;
        // --- END OF CHANGE ---

        // (Alignment logic is correct)
        if let Some(h_align) = h_align {
            match h_align.0 {
                HorizontalAlignmentType::Left => { /* Default */ }
                HorizontalAlignmentType::Center => x -= w / 2.0,
                HorizontalAlignmentType::Right => x -= w,
            }
        }
        if let Some(v_align) = v_align {
            match v_align.0 {
                VerticalAlignmentType::Top => { /* Default */ }
                VerticalAlignmentType::Center => y -= h / 2.0,
                VerticalAlignmentType::Bottom => y -= h,
            }
        }
        
        let radius = gui_box.border_radius.min(w / 2.0).min(h / 2.0);

        // Determine the final color
        let mut final_color = gui_box.color;
        if let Some(button) = button_opt {
            final_color = match button.state {
                ButtonState::Hovered => button.hovered_color,
                ButtonState::Pressed => button.pressed_color,
                ButtonState::Idle => button.normal_color
            };
        }

        // (Drawing logic is correct)
        if radius == 0.0 {
            draw_rectangle(x, y, w, h, final_color);
        } else {
            // 1. Create an opaque version
            let opaque_color = Color::new(final_color.r, final_color.g, final_color.b, 1.0);

            // 2. Create the render target
            let rt_w = w.max(1.0) as u32;
            let rt_h = h.max(1.0) as u32;
            let rt = render_target(rt_w, rt_h);
            rt.texture.set_filter(FilterMode::Nearest);

            // 3. Set up a camera
            let camera = Camera2D {
                render_target: Some(rt.clone()),
                zoom: vec2(2.0 / rt_w as f32, 2.0 / rt_h as f32),
                target: vec2(rt_w as f32 / 2.0, rt_h as f32 / 2.0),
                ..Default::default()
            };
            set_camera(&camera);

            // 4. Draw the 7 shapes (OPAQUE) at (0, 0)
            clear_background(BLANK);
            draw_rectangle(radius, 0.0, w - radius * 2.0, h, opaque_color);
            draw_rectangle(0.0, radius, radius, h - radius * 2.0, opaque_color);
            draw_rectangle(w - radius, radius, radius, h - radius * 2.0, opaque_color);
            draw_circle(radius, radius, radius, opaque_color);
            draw_circle(w - radius, radius, radius, opaque_color);
            draw_circle(radius, h - radius, radius, opaque_color);
            draw_circle(w - radius, h - radius, radius, opaque_color);

            // 5. Restore the default camera
            set_default_camera();

            // 6. Draw the RenderTarget to the screen at its final, aligned position
            draw_texture_ex(
                &rt.texture,
                x, // Use the aligned x
                y, // Use the aligned y
                final_color, // Use the original color (with alpha)
                DrawTextureParams {
                    dest_size: Some(vec2(w, h)),
                    ..Default::default()
                },
            );
        }
    }
}

pub fn text_render_system(ctx: &mut Context) {
    // This system correctly uses Transform.position, which is set by the
    // gui_resolve_layout_system. No changes are needed.
    for (_, (text_display, transform, visibility, font_opt, h_align, v_align)) in ctx.world.query::<(&TextDisplay, &Transform, Option<&Visible>, Option<&FontComponent>, Option<&HorizontalAlignment>, Option<&VerticalAlignment>)>().iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible || !text_display.screen_space {
            continue;
        }

        let font = font_opt.and_then(|f| ctx.asset_server.get_font(&f.0));
        
        let text_size = measure_text(&text_display.text, font, text_display.font_size as u16, 1.0);

        // --- Alignment Logic (Correct) ---
        let mut draw_x = transform.position.x;
        if let Some(h_align) = h_align {
            match h_align.0 {
                HorizontalAlignmentType::Left => { /* Default */ }
                HorizontalAlignmentType::Center => draw_x = transform.position.x - text_size.width / 2.0,
                HorizontalAlignmentType::Right => draw_x = transform.position.x - text_size.width,
            }
        }
        
        let mut baseline_y = transform.position.y + text_size.offset_y; 
        
        if let Some(v_align) = v_align {
            match v_align.0 {
                VerticalAlignmentType::Top => { /* Default */ }
                VerticalAlignmentType::Center => baseline_y = transform.position.y - (text_size.height / 2.0) + text_size.offset_y,
                VerticalAlignmentType::Bottom => baseline_y = transform.position.y - text_size.height + text_size.offset_y,
            }
        }
        // --- End Alignment Logic ---

        if let Some(font) = font {
            draw_text_ex(
                &text_display.text,
                draw_x.round(),
                baseline_y.round(),
                TextParams {
                    font: Some(font),
                    font_size: text_display.font_size as u16,
                    color: text_display.color,
                    ..Default::default()
                }
            );
        }
        else {
            draw_text(
                &text_display.text,
                draw_x.round(),
                baseline_y.round(),
                text_display.font_size,
                text_display.color
            );
        }
    }
}

pub fn draggable_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let current_mouse_pos = vec2(mouse_x, mouse_y);
    
    // --- MODIFIED ---
    let delta = current_mouse_pos - ctx.resource::<PreviousMousePosition>().0;
    
    let is_pressed = is_mouse_button_pressed(MouseButton::Left);
    let is_down = is_mouse_button_down(MouseButton::Left);

    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&mut GuiDraggable, &mut Transform, &GuiBox, Option<&Visible>)>();

    for (entity, (draggable, transform, _gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        // --- MODIFIED ---
        let (_resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue; // Not processed by layout system
            };

        if draggable.is_dragging {
            if !is_down {
                draggable.is_dragging = false;
            } else {
                // This is correct: it modifies the Transform directly,
                // which will be used as the base pos next frame.
                transform.position.x += delta.x;
                transform.position.y += delta.y;
            }
        } else {
            // Use the transform's position for hover checking, as it's
            // the most up-to-date position.
            let x = transform.position.x;
            let y = transform.position.y;
            let w = resolved_size.x;
            let h = resolved_size.y;

            let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

            if is_hovered && is_pressed {
                draggable.is_dragging = true;
            }
        }
    }
}

pub fn slider_interaction_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_pressed(MouseButton::Left);
    let is_down = is_mouse_button_down(MouseButton::Left);

    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&mut GuiSlider, &GuiBox, Option<&Visible>)>();

    for (entity, (slider, _gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue;
            };

        let x = resolved_pos.x;
        let y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;
        
        let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

        if slider.is_dragging_handle {
            if !is_down {
                slider.is_dragging_handle = false;
            } else {
                let relative_x = mouse_x - x;
                let normalized_value = (relative_x / w).clamp(0.0, 1.0);
                slider.value = slider.min + normalized_value * (slider.max - slider.min);
            }
        } else if is_hovered && is_pressed {
            slider.is_dragging_handle = true;
            let relative_x = mouse_x - x;
            let normalized_value = (relative_x / w).clamp(0.0, 1.0);
            slider.value = slider.min + normalized_value * (slider.max - slider.min);
        }
    }
}

pub fn slider_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;
    
    let mut query = ctx.world.query::<(&GuiSlider, &GuiBox, Option<&Visible>)>();

    for (entity, (slider, _gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue;
            };

        let x = resolved_pos.x;
        let y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;

        let normalized_value = (slider.value - slider.min) / (slider.max - slider.min).max(f32::EPSILON);
        let handle_width = slider.handle_width;
        
        let handle_x = x + (normalized_value * w) - (handle_width / 2.0);

        draw_rectangle(
            handle_x.clamp(x, x + w - handle_width),
            y,
            handle_width,
            h,
            slider.handle_color
        )
    }
}

pub fn checkbox_logic_system(ctx: &mut Context) {
    // This system doesn't use the map, no changes needed.
    let mut query = ctx.world.query::<(&GuiButton, &mut GuiCheckbox, Option<&Visible>)>();

    for (_, (button, checkbox, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }
        
        if button.just_clicked {
            checkbox.is_checked = !checkbox.is_checked;
        }
    }
}

pub fn checkbox_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&GuiCheckbox, &GuiBox, Option<&Visible>)>();

    for (entity, (checkbox, _gui_box, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);

        if !is_visible {
            continue;
        }

        if checkbox.is_checked {
            // --- MODIFIED ---
            let (resolved_pos, resolved_size) = 
                if let Some(rect) = resolved_rects_map.get(&entity) {
                    *rect
                } else {
                    continue;
                };

            let x = resolved_pos.x;
            let y = resolved_pos.y;
            let w = resolved_size.x;
            let h = resolved_size.y;

            let padding = w * 0.2;
            draw_line(x + padding, y + padding, x + w - padding, y + h - padding, 2.0, BLACK);
            draw_line(x + w - padding, y + padding, x + padding, y + h - padding, 2.0, BLACK);
        }
    }
}

pub fn input_field_focus_system(ctx: &mut Context) {
    let (mouse_x, mouse_y) = mouse_position();
    let is_pressed = is_mouse_button_pressed(MouseButton::Left);

    if !is_pressed {
        return;
    }

    let mut clicked_entity: Option<Entity> = None;
    
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&GuiBox, Option<&Visible>)>();

    for (entity, (gui_box, visibility)) in query.iter() {
        if ctx.world.get::<&GuiInputField>(entity).is_err() {
            continue;
        }

        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible || !gui_box.screen_space {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue;
            };

        let x = resolved_pos.x;
        let y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;

        let is_hovered = mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y + h);

        if is_hovered {
            clicked_entity = Some(entity);
            break;
        }
    }

    // (Logic for setting focus is correct)
    let mut query = ctx.world.query::<&mut GuiInputField>();
    for (e, input_field) in query.iter() {
        if Some(e) == clicked_entity {
            if !input_field.is_focused {
                while get_char_pressed().is_some() {}
                input_field.caret_position = input_field.text.chars().count()
            }

            input_field.is_focused = true;
            input_field.caret_visible = true;
            input_field.caret_blink_timer = 0.0;
        }
        else {
            input_field.is_focused = false;
        }
    }
}

pub fn input_field_typing_system(ctx: &mut Context) {
    const KEY_REPEAT_INITIAL_DELAY: f32 = 0.4;
    const KEY_REPEAT_RATE: f32 = 0.05;

    // --- MODIFIED: Get dt once ---
    let dt = ctx.dt();
    
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&mut GuiInputField, &GuiBox, Option<&FontComponent>)>();

    for (entity, (input_field, _gui_box, font_opt)) in query.iter() {
        if !input_field.is_focused {
            input_field.backspace_repeat_timer = 0.0;
            input_field.left_key_repeat_timer = 0.0;
            input_field.right_key_repeat_timer = 0.0;
            continue;
        }
        
        // --- Left Arrow ---
        let left_pressed = is_key_pressed(KeyCode::Left);
        let left_down = is_key_down(KeyCode::Left);
        let mut move_left = false;

        if left_pressed {
            move_left = true;
            input_field.left_key_repeat_timer = KEY_REPEAT_INITIAL_DELAY;
        }
        else if left_down {
            // --- MODIFIED ---
            input_field.left_key_repeat_timer -= dt;
            if input_field.left_key_repeat_timer <= 0.0 {
                move_left = true;
                input_field.left_key_repeat_timer = KEY_REPEAT_RATE;
            }
        }
        else {
            input_field.left_key_repeat_timer = 0.0;
        }

        if move_left && input_field.caret_position > 0 {
            input_field.caret_position -= 1;
            input_field.caret_visible = true;
            input_field.caret_blink_timer = 0.0;
        }

        // --- Right Arrow ---
        let right_pressed = is_key_pressed(KeyCode::Right);
        let right_down = is_key_down(KeyCode::Right);
        let mut move_right = false;

        if right_pressed {
            move_right = true;
            input_field.right_key_repeat_timer = KEY_REPEAT_INITIAL_DELAY;
        }
        else if right_down {
            // --- MODIFIED ---
            input_field.right_key_repeat_timer -= dt;
            if input_field.right_key_repeat_timer <= 0.0 {
                move_right = true;
                input_field.right_key_repeat_timer = KEY_REPEAT_RATE;
            }
        }
        else {
            input_field.right_key_repeat_timer = 0.0;
        }

        if move_right {
            let text_len = input_field.text.chars().count();
            if input_field.caret_position < text_len {
                input_field.caret_position += 1;
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
            }
        }

        // --- Backspace ---
        let backspace_pressed = is_key_pressed(KeyCode::Backspace);
        let backspace_down = is_key_down(KeyCode::Backspace);
        
        let mut should_delete = false;
        if backspace_pressed {
            should_delete = true;
            input_field.backspace_repeat_timer = KEY_REPEAT_INITIAL_DELAY;
        } else if backspace_down {
            // --- MODIFIED ---
            input_field.backspace_repeat_timer -= dt;
            if input_field.backspace_repeat_timer <= 0.0 {
                should_delete = true;
                input_field.backspace_repeat_timer = KEY_REPEAT_RATE;
            }
        } else {
            input_field.backspace_repeat_timer = 0.0;
        }

        if should_delete && input_field.caret_position > 0 {
            let mut chars: Vec<char> = input_field.text.chars().collect();
            if input_field.caret_position <= chars.len() {
                chars.remove(input_field.caret_position - 1);
                input_field.text = chars.into_iter().collect();
                input_field.caret_position -= 1;
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
            }
        }
        
        // --- Delete Key ---
        if is_key_pressed(KeyCode::Delete) {
             let mut chars: Vec<char> = input_field.text.chars().collect();
             if input_field.caret_position < chars.len() {
                chars.remove(input_field.caret_position);
                input_field.text = chars.into_iter().collect();
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
             }
        }

        // --- Typing ---
        while let Some(char) = get_char_pressed() {
            if char == '\u{08}' || char == '\u{7f}' { // Backspace or Delete
                continue; 
            }

            let char_count = input_field.text.chars().count();
            let at_limit = input_field.max_chars.map_or(false, |max| char_count >= max);
        
            if !at_limit {
                let mut chars: Vec<char> = input_field.text.chars().collect();
                let insert_pos = input_field.caret_position.min(chars.len());
                chars.insert(insert_pos, char);
                input_field.text = chars.into_iter().collect();
                
                input_field.caret_position += 1;
                input_field.caret_visible = true;
                input_field.caret_blink_timer = 0.0;
            }
        }


        // --- Scroll Logic ---
        let font_to_use: Option<&Font> = font_opt.and_then(|f| ctx.asset_server.get_font(&f.0));

        let text_before_caret: String = input_field.text.chars().take(input_field.caret_position).collect();
        let caret_x_absolute = measure_text(&text_before_caret, font_to_use, input_field.font_size as u16, 1.0).width;

        // --- MODIFIED ---
        let w = if let Some((_, size)) = resolved_rects_map.get(&entity) {
            size.x
        } else {
            300.0 // Fallback
        };

        let visible_width = w - (input_field.padding.x * 2.0);

        // (Scroll logic is correct)
        if caret_x_absolute < input_field.scroll_offset {
            input_field.scroll_offset = caret_x_absolute;
        }
        if caret_x_absolute > input_field.scroll_offset + visible_width {
            input_field.scroll_offset = caret_x_absolute - visible_width;
        }
        let total_text_width = measure_text(&input_field.text, font_to_use, input_field.font_size as u16, 1.0).width;
        if total_text_width < visible_width {
             input_field.scroll_offset = 0.0;
        } else if total_text_width - input_field.scroll_offset < visible_width {
             input_field.scroll_offset = (total_text_width - visible_width).max(0.0);
        }

        // --- Caret Blink ---
        // --- MODIFIED ---
        input_field.caret_blink_timer += dt;
        if input_field.caret_blink_timer >= 0.5 {
            input_field.caret_visible = !input_field.caret_visible;
            input_field.caret_blink_timer = 0.0;
        }
    }
}

pub fn input_field_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&GuiInputField, &GuiBox, Option<&Visible>, Option<&FontComponent>)>();

    for (entity, (input_field, gui_box, visibility, font_opt)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible { continue; }

        if !gui_box.screen_space {
            continue;
        }

        // --- MODIFIED ---
        let (resolved_pos, resolved_size) = 
            if let Some(rect) = resolved_rects_map.get(&entity) {
                *rect
            } else {
                continue;
            };

        let x = resolved_pos.x;
        let y = resolved_pos.y;
        let w = resolved_size.x;
        let h = resolved_size.y;

        let rt_w = (w.max(1.0)) as u32;
        let rt_h = (h.max(1.0)) as u32;
        let rt = render_target(rt_w, rt_h);

        let camera = Camera2D {
            render_target: Some(rt.clone()),
            viewport: None,
            zoom: vec2(2.0 / rt_w as f32, 2.0 / rt_h as f32),
            target: vec2(rt_w as f32 / 2.0, rt_h as f32 / 2.0),
            ..Default::default()
        };

        set_camera(&camera);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        let content_x = input_field.padding.x;
        let text_y_top = (rt_h as f32 - input_field.font_size) / 2.0;
        let baseline_y = text_y_top + input_field.font_size * 0.8; 
        let draw_x = content_x - input_field.scroll_offset;

        let font_to_use: Option<&Font> = font_opt.and_then(|f| ctx.asset_server.get_font(&f.0));

        // (Text drawing logic is correct)
        if let Some(font) = font_to_use {
            draw_text_ex(
                &input_field.text,
                draw_x,
                baseline_y,
                TextParams {
                    font: Some(font),
                    font_size: input_field.font_size as u16,
                    color: input_field.color,
                    ..Default::default()
                }
            );
        } else {
            draw_text(
                &input_field.text,
                draw_x,
                baseline_y,
                input_field.font_size,
                input_field.color
            );
        }

        // (Caret drawing logic is correct)
        if input_field.is_focused && input_field.caret_visible {
            let text_before_caret: String = input_field.text.chars().take(input_field.caret_position).collect();
            let caret_offset = measure_text(&text_before_caret, font_to_use, input_field.font_size as u16, 1.0).width;
            let caret_x = draw_x + caret_offset;

            draw_rectangle(
                caret_x,
                text_y_top,
                2.0, 
                input_field.font_size,
                input_field.color
            );
        }

        set_default_camera();

        let draw_params = DrawTextureParams {
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        };

        draw_texture_ex(&rt.texture, x, y, WHITE, draw_params);
    }
}

pub fn input_focus_update_system(ctx: &mut Context) {
    // --- MODIFIED ---
    // Get the resource mutably once.
    let (_world, resources) = (&ctx.world, &mut ctx.resources);

    let input_focus = resources.get_mut::<InputFocus>()
        .expect("Ressource InputFocus manquante");
    input_focus.is_captured_by_ui = false;

    for (_, input_field) in ctx.world.query::<&GuiInputField>().iter() {
        if input_field.is_focused {
            // --- MODIFIED ---
            input_focus.is_captured_by_ui = true;
            break;
        }
    }
}

pub fn gui_image_render_system(ctx: &mut Context) {
    // --- MODIFIED: Get map once ---
    let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

    let mut query = ctx.world.query::<(&GuiImage, &Transform, Option<&GuiBox>, Option<&Visible>)>();

    for (entity, (gui_image, transform, gui_box_opt, visibility)) in query.iter() {
        let is_visible = visibility.map_or(true, |v| v.0);
        if !is_visible {
            continue;
        }

        if !gui_image.screen_space {
            continue;
        }

        if let Some(spritesheet_name) = &gui_image.texture {
            if let Some(spritesheet) = ctx.asset_server.get_spritesheet(&spritesheet_name) {
                let texture = &spritesheet.texture;
                let source = spritesheet.get_source_rect(gui_image.col_row.x, gui_image.col_row.y);

                let (draw_x, draw_y, dest_size) = 
                    if gui_box_opt.is_some() {
                        // This element has a GuiBox, use the resolved rect
                        // --- MODIFIED ---
                        if let Some((pos, size)) = resolved_rects_map.get(&entity) {
                            (pos.x, pos.y, *size)
                        } else {
                            continue; // Not laid out
                        }
                    } else {
                        // No GuiBox (e.g., simple icon). Use transform data.
                        let dest = vec2(
                            spritesheet.sprite_width * transform.scale.x, 
                            spritesheet.sprite_height * transform.scale.y
                        );
                        (transform.position.x, transform.position.y, dest)
                    };

                let draw_params = DrawTextureParams {
                    dest_size: Some(dest_size),
                    source,
                    ..Default::default()
                };

                draw_texture_ex(texture, draw_x, draw_y, gui_image.tint, draw_params);
            }
        }
    }
}
