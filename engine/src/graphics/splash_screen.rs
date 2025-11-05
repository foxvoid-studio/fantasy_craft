use macroquad::prelude::*;
use crate::prelude::{Context, GuiBox, GuiImage, Transform};

#[derive(Debug)]
pub struct SplashScreenTag;

#[derive(Debug)]
pub struct SplashAnimation {
    pub timer: f32,
    pub fade_in_time: f32,
    pub fade_out_time: f32,
    pub total_duration: f32
}

pub fn setup_splash_screen(ctx: &mut Context) {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let logo_w = 400.0;
    let logo_h = 400.0;

    let pos_x = (screen_w / 2.0) - (logo_w / 2.0);
    let pos_y = (screen_h / 2.0) - (logo_h / 2.0);

    ctx.world.spawn((
        Transform {
            position: vec2(pos_x, pos_y),
            ..Default::default()
        },
        GuiBox {
            width: logo_w,
            height: logo_h,
            color: BLACK,
            ..Default::default()
        },
        GuiImage {
            texture: Some("logo_engine".to_string()),
            col_row: uvec2(0, 0),
            tint: WHITE,
            ..Default::default()
        },
        SplashScreenTag,
        SplashAnimation {
            timer: 0.0,
            fade_in_time: 1.2,
            fade_out_time: 1.0,
            total_duration: 3.0
        }
    ));
}

pub fn animate_splash_screen(ctx: &mut Context) {
    for (_, (transform, gui_image, anim)) in ctx.world.query::<(&mut Transform, &mut GuiImage, &mut SplashAnimation)>().iter() {
        anim.timer += ctx.dt;

        if anim.timer < anim.fade_in_time {
            let t = anim.timer / anim.fade_in_time;
            gui_image.tint.a = t.clamp(0.0, 1.0);
        }
        else if anim.timer < anim.total_duration - anim.fade_out_time {
            let t = (anim.timer - anim.fade_in_time) * 2.0;
            transform.scale = vec2(1.0 + 0.02 * (t.sin()), 1.0 + 0.02 * (t.sin()));
            gui_image.tint.a = 1.0;
        }
        else {
            let t = (anim.total_duration - anim.timer) / anim.fade_out_time;
            gui_image.tint.a = t.clamp(0.0, 1.0);
        }
    }
}

pub fn despawn_splash_screen(ctx: &mut Context) {
    let mut entities_to_despawn = Vec::new();

    for (entity, _) in ctx.world.query::<&SplashScreenTag>().iter() {
        entities_to_despawn.push(entity);
    }

    for entity in entities_to_despawn {
        ctx.world.despawn(entity).expect("Failed to despawn splash entity");
    }
}