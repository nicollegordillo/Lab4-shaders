use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::color::Color;
use crate::fragment::Fragment;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
	let position = Vec4::new(
		vertex.position.x,
		vertex.position.y,
		vertex.position.z,
		1.0
	);
	let transformed =  uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

	let W = transformed.w;
	let ndc_position = Vec4::new(
		transformed.x / W,
		transformed.y / W,
		transformed.z / W,
		1.0
	);

	let screen_position = uniforms.viewport_matrix * ndc_position;

	let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
	let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

	let transformed_normal = normal_matrix * vertex.normal;

	Vertex {
		position: vertex.position,
		normal: vertex.normal,
		tex_coords: vertex.tex_coords,
		color: vertex.color,
		transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
		transformed_normal: vertex.normal,
	}
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colors for the sun's core and its glow
    let sun_core_color = Color::new(255.0, 204.0, 0.0); // Bright yellow core
    let glow_color = Color::new(255.0, 100.0, 0.0);     // Red-orange glow

    // Adjust the core and glow radii to increase the sun's visible area
    let core_radius = 0.3;   // Adjust this to make the core larger
    let glow_radius = 1.0;   // Adjust this for a wider glow

    // Calculate the distance from the center (assuming the sun is at 0.5, 0.5)
    let center_x = 0.0;
    let center_y = 0.0;
    let dist_x = fragment.vertex_position.x - center_x;
    let dist_y = fragment.vertex_position.y - center_y;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

    // Choose color based on distance
    let base_color = if distance < core_radius {
        sun_core_color
    } else if distance < glow_radius {
        // Apply the blend effect for the glowing part
        sun_core_color.blend_color_burn(&glow_color)
    } else {
        Color::black() // Background color outside the glow radius
    };

    // Apply intensity to the final color
    base_color * fragment.intensity
}

pub fn fragment_shader_urano(fragment: &Fragment, uniforms: &Uniforms) -> Color {
   let color = Color::new(189.0, 219.0, 208.0); 
   color * fragment.intensity
}

// Saturn Shader
pub fn fragment_shader_saturn(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let inner_color = Color::new(210.0, 180.0, 140.0); // Light tan for the core
    let outer_color = Color::new(245.0, 230.0, 210.0); // Pale yellow for the outer edges

    // Calculate distance from the center
    let center_x = 0.0;
    let center_y = 0.0;
    let dist_x = fragment.vertex_position.x - center_x;
    let dist_y = fragment.vertex_position.y - center_y;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

    // Lerp based on distance
    let blended_color = inner_color.lerp(&outer_color, distance.min(1.0));
    blended_color * fragment.intensity
}

// Jupiter Shader
pub fn fragment_shader_jupiter(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let inner_color = Color::new(255.0, 178.0, 102.0); // Light orange-brown
    let mid_color = Color::new(255.0, 255.0, 255.0);   // White for the mid-bands
    let outer_color = Color::new(178.0, 125.0, 102.0); // Brown for outer regions

    let center_x = 0.0;
    let center_y = 0.0;
    let dist_x = fragment.vertex_position.x - center_x;
    let dist_y = fragment.vertex_position.y - center_y;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

    let blended_color = if distance < 0.5 {
        inner_color.lerp(&mid_color, distance * 2.0)
    } else {
        mid_color.lerp(&outer_color, (distance - 0.5) * 2.0)
    };
    blended_color * fragment.intensity
}

// Neptune Shader
pub fn fragment_shader_neptune(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let inner_color = Color::new(70.0, 130.0, 180.0); // Deep blue for the core
    let outer_color = Color::new(173.0, 216.0, 230.0); // Light blue for outer edges

    let center_x = 0.0;
    let center_y = 0.0;
    let dist_x = fragment.vertex_position.x - center_x;
    let dist_y = fragment.vertex_position.y - center_y;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

    let blended_color = inner_color.lerp(&outer_color, distance.min(1.0));
    blended_color * fragment.intensity
}

pub fn fragment_shader_saturn_with_ring(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let inner_color = Color::new(210.0, 180.0, 140.0); // Light tan for Saturn's core
    let outer_color = Color::new(245.0, 230.0, 210.0); // Pale yellow for the outer edges
    let ring_color = Color::new(220.0, 220.0, 220.0); // Gray for the ring

    // Calculate distance from the center
    let center_x = 0.0;
    let center_y = 0.0;
    let dist_x = fragment.vertex_position.x - center_x;
    let dist_y = fragment.vertex_position.y - center_y;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

    // Define radii for Saturn's core, outer edge, and ring
    let core_radius = 0.8;       // Adjusted for larger scale
    let outer_radius = 1.4;
    let ring_inner_radius = 2.5;
    let ring_outer_radius = 3.0;

    // Determine color based on distance
    let base_color = if distance < core_radius {
        inner_color
    } else if distance < outer_radius {
        // Blend between inner and outer color within Saturn's main body
        inner_color.lerp(&outer_color, (distance - core_radius) / (outer_radius - core_radius))
    } else if distance >= ring_inner_radius && distance <= ring_outer_radius {
        // Ring effect: Make it partially transparent for a faint look
        ring_color * 0.8
    } else {
        Color::black() // Background color outside the planet and ring
    };

    // Apply intensity to the final color
    base_color * fragment.intensity
}

pub fn fragment_shader_mars(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let mars_color = Color::new(210.0, 80.0, 0.0); // Rusty red-orange color for Mars
    mars_color * fragment.intensity
}

pub fn fragment_shader_venus(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let venus_color = Color::new(255.0, 223.0, 160.0); // Pale yellowish color for Venus
    venus_color * fragment.intensity
}

pub fn fragment_shader_earth(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base colors for land and sea
    let land_color = Color::new(34.0, 139.0, 34.0); // Greenish land color
    let sea_color = Color::new(0.0, 105.0, 148.0);   // Blue sea color

    // Generate a blotchy pattern for land and sea
    let blotch_size = 8.0; // Size of the blotches
    let noise_scale = 5.0; // Adjust this value for more or less detail in the noise

    // Calculate noise values for a blotchy effect
    let noise_value = ((fragment.vertex_position.x * noise_scale).sin() +
                       (fragment.vertex_position.y * noise_scale).cos()).abs();

    // Distinguish between sea and land using the noise pattern
    let base_color = if noise_value > 0.5 {
        land_color // Use land color for higher noise values
    } else {
        sea_color // Use sea color for lower noise values
    };

    // Add moving clouds (white) overlay based on time
    let cloud_color = Color::new(255.0, 255.0, 255.0); // White clouds
    let cloud_speed = 0.1; // Adjust speed as desired
    let cloud_pattern = ((fragment.vertex_position.x * 10.0 + uniforms.time as f32 * cloud_speed).sin() *
                         (fragment.vertex_position.y * 10.0 + uniforms.time as f32 * cloud_speed).cos()).abs();

    

   base_color * fragment.intensity
}







