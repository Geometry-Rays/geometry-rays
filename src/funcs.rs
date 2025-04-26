use macroquad::prelude::*;

use crate::types::{Button, ObjectType};

pub fn draw_text_pro(
    text: &str,
    x: f32,
    y: f32,
    size: u8,
    color: Color,
    font: &Font
) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size as u16,
            color: color,
            ..Default::default()
        }
    );
}

pub fn measure_text_ex(
    text: &str,
    size: u8,
    font: &Font
) -> f32 {
    measure_text(text, Some(font), size as u16, 1.0).width
}

impl Button {
    pub fn new<Fx, Fy>(
        x: Fx,
        y: Fy,
        width: f32,
        height: f32,
        text: &str,
        font_size: i32,
        is_disabled: bool
    ) -> Self
    where
        Fx: 'static + Fn() -> f32 + Clone,
        Fy: 'static + Fn() -> f32 + Clone,
    {
        let x_clone = x.clone();
        let y_clone = y.clone();
        let rect_fn = Box::new(move || {
            Rect::new(x_clone(), y_clone().clone(), width, height)
        });

        Button {
            rect: Rect::new(x(), y(), width, height),
            rect_fn,
            text: text.to_string(),
            font_size,
            base_color: WHITE,
            hover_scale: 1.0,
            press_offset: 0.0,
            is_pressed: false,
            animation_timer: 0.0,
            is_disabled: is_disabled,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rect = (self.rect_fn)();

        let is_hovered = self.is_hovered();
        let is_pressed = is_hovered && is_mouse_button_down(MouseButton::Left);

        // Update hover animation
        let target_scale = if is_hovered { 1.1 } else { 1.0 };
        self.hover_scale += (target_scale - self.hover_scale) * (delta_time * 12.0);

        // Update press animation
        let target_offset = if is_pressed { 4.0 } else { 0.0 };
        self.press_offset += (target_offset - self.press_offset) * (delta_time * 15.0);

        // Update color animation
        if is_hovered {
            self.animation_timer = (self.animation_timer + delta_time * 8.0).min(1.0);
        } else {
            self.animation_timer = (self.animation_timer - delta_time * 8.0).max(0.0);
        }

        self.is_pressed = is_pressed;
    }

    pub fn draw(&self, use_image: bool, image: Option<&&Texture2D>, image_scale: f32, gray: bool, font: &Font) {
        let scale_offset_x = self.rect.w * (self.hover_scale - 1.0) * 0.5;
        let scale_offset_y = self.rect.h * (self.hover_scale - 1.0) * 0.5;

        let scaled_rect = Rect::new(
            self.rect.x - scale_offset_x,
            self.rect.y - scale_offset_y + self.press_offset,
            self.rect.w * self.hover_scale,
            self.rect.h * self.hover_scale,
        );

        // Draw main button body
        draw_rectangle(
            scaled_rect.x,
            scaled_rect.y,
            scaled_rect.w,
            scaled_rect.h,
            if self.is_disabled { BLACK } else if gray { GRAY } else { self.base_color },
        );

        // Old way of drawing button borders
        // d.draw_rectangle_lines(
        //     scaled_rect.x as i32,
        //     scaled_rect.y as i32,
        //     scaled_rect.width as i32,
        //     scaled_rect.height as i32,
        //     Color::new(0, 0, 0, 255),
        // );

        // Draw button borders
        draw_rectangle_lines(
            scaled_rect.x,
            scaled_rect.y,
            scaled_rect.w,
            scaled_rect.h,
            10.0,
            if self.is_disabled { WHITE } else { BLACK }
        );

        // d.draw_rectangle_lines_ex(scaled_rect, 5.0, if self.is_disabled { Color::WHITE } else { Color::BLACK });

        // Draw text with perfect centering
        let text_width = measure_text_ex(&self.text, self.font_size as u8, &font);
        let text_x = scaled_rect.x + ((scaled_rect.w - text_width) / 2.0);
        let text_y = scaled_rect.y as i32 + ((scaled_rect.h as i32 - self.font_size) / 2);

        if !use_image {
            draw_text_pro(
                &self.text,
                text_x,
                text_y as f32 + self.font_size as f32,
                self.font_size as u8,
                if self.is_disabled { WHITE } else { BLACK },
                &font
            );
        } else {
            draw_texture_ex(
                image.unwrap(),
                self.rect.x + self.rect.w / 2.0 - image.unwrap().width() as f32 * image_scale / 2.0,
                self.rect.y + self.rect.w / 2.0 - image.unwrap().height() as f32 * image_scale / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        image.unwrap().width() * image_scale,
                        image.unwrap().height() * image_scale
                    )),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None
                }
            );
        }
    }

    pub fn is_hovered(&self) -> bool {
        self.rect.contains(mouse_position().into())
    }

    pub fn is_clicked(&self) -> bool {
        self.is_hovered() && is_mouse_button_released(MouseButton::Left)
    }
}

impl ObjectType {
    pub fn new(
        id: u16,
        name: &str,
        texture: Texture2D,
        sorting: u16,
        obj_btn_offset: f32
    ) -> ObjectType {
        ObjectType {
            id,
            name: name.to_string(),
            texture,
            button: Button::new(
                move || 140.0 + (sorting as f32 * obj_btn_offset),
                || screen_height() - 190.0,
                60.0,
                60.0,
                name,
                10,
                false
            )
        }
    }
}