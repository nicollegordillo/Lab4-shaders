use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader_urano, fragment_shader_neptune, fragment_shader_jupiter, fragment_shader_saturn_with_ring, fragment_shader_venus, fragment_shader_mars, fragment_shader_earth, fragment_shader_mercury, fragment_shader_sun, fragment_shader_moon, fragment_shader_ring};
use camera::Camera;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_x, -sin_x, 0.0,
        0.0, sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0, 0.0, translation.x,
        0.0, scale, 0.0, translation.y,
        0.0, 0.0, scale, translation.z,
        0.0, 0.0, 0.0, 1.0,
    );
    
    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], shader_type: u8){
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i+2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i+1].clone(),
                transformed_vertices[i+2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            //let shaded_color = fragment_shader_saturn_with_ring(&fragment, &uniforms);
            let shaded_color = match shader_type {
                1 => fragment_shader_jupiter(&fragment, uniforms),
                2 => fragment_shader_saturn_with_ring(&fragment, uniforms),
                3 => fragment_shader_urano(&fragment, uniforms),
                4 => fragment_shader_venus(&fragment, uniforms),
                5 => fragment_shader_mars(&fragment, uniforms),
                6 => fragment_shader_earth(&fragment, uniforms),
                7 => fragment_shader_mercury(&fragment, uniforms),
                8 => fragment_shader_sun(&fragment, uniforms),
                9 => fragment_shader_moon(&fragment, uniforms),
                10 => fragment_shader_ring(&fragment, uniforms),
                _ => fragment_shader_neptune(&fragment, uniforms),
            };
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x , y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = 600;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust 3D model",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(0,0);
    window.update();

    framebuffer.set_background_color(0x333355);

    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let mut rotation = Vec3::new(0.0,0.0,0.0);
    let mut scale = 1.0f32;

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let obj = Obj::load("./sphere.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();

    let moon_obj = Obj::load("./moon.obj").expect("Failed to load moon.obj");
    let moon_vertex_array = moon_obj.get_vertex_array();

    let ring_obj = Obj::load("./ring.obj").expect("Failed to load ring.obj");
    let ring_vertex_array = ring_obj.get_vertex_array();

    let mut time = 0;
    let mut shader_type = 0;

    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        
        time += 1;

        if window.is_key_down(Key::NumPad1) { shader_type = 1; } 
        if window.is_key_down(Key::NumPad2) { shader_type = 2; } 
        if window.is_key_down(Key::NumPad3) { shader_type = 3; } 
        if window.is_key_down(Key::NumPad4) { shader_type = 4; } 
        if window.is_key_down(Key::NumPad5) { shader_type = 5; } 
        if window.is_key_down(Key::NumPad6) { shader_type = 6; }
        if window.is_key_down(Key::NumPad7) { shader_type = 7; }
        if window.is_key_down(Key::NumPad8) { shader_type = 8; }
        if window.is_key_down(Key::NumPad0) { shader_type = 0; }

        framebuffer.clear();

        handle_input(&window, &mut camera);

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let uniforms= Uniforms { 
            model_matrix, 
            view_matrix, 
            projection_matrix, 
            viewport_matrix,
            time,
        };

        framebuffer.set_current_color(0xFFDDDD);
        //render(&mut framebuffer, &uniforms, &vertex_arrays, shader_type);
        if shader_type == 2 {
            // Render Saturn
            render(&mut framebuffer, &uniforms, &vertex_arrays, shader_type);

            // Adjust ring's transformation matrix
            let ring_translation = Vec3::new(0.0, 0.0, 0.0); // Centered on Saturn
            let ring_scale = 0.6; // Adjust scale to fit around Saturn
            let ring_rotation = Vec3::new(0.0, rotation.y, 0.0); // Rotate with Saturn
            let ring_model_matrix = create_model_matrix(ring_translation, ring_scale, ring_rotation);

            let ring_uniforms = Uniforms {
                model_matrix: ring_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
            };

            render(&mut framebuffer, &ring_uniforms, &ring_vertex_array, 10); // Use shader 10 for the ring

        } else if shader_type == 6 {
            render(&mut framebuffer, &uniforms, &vertex_arrays, shader_type); // Render Earth
            // Calculate moon's orbital angle based on time for circular motion
            let orbit_radius = 1.0;
            let angle = (uniforms.time as f32) * 0.05;//djust speed by modifying the multiplier

            // Position the moon in orbit around Earth
            let moon_translation = Vec3::new(orbit_radius * angle.cos(), 0.0, orbit_radius * angle.sin());
            let moon_rotation = Vec3::new(0.0, angle, 0.0); // Rotate the moon to face outward in orbit
            let moon_model_matrix = create_model_matrix(moon_translation, 0.3, moon_rotation);

            let moon_uniforms = Uniforms {
                model_matrix: moon_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
            };
            render(&mut framebuffer, &moon_uniforms, &vertex_arrays, 9); // Render Moon with shader 9 (example)
        } else {
            render(&mut framebuffer, &uniforms, &vertex_arrays, shader_type);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, camera: &mut Camera){
    let movement_speed= 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;

    if window.is_key_down(Key::Left){
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right){
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W){
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S){
        camera.orbit(0.0, rotation_speed);
    }

    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A){
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D){
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q){
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E){
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    if window.is_key_down(Key::Up){
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down){
        camera.zoom(-zoom_speed);
    }
}
