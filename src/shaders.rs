use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::color::Color;
use crate::fragment::Fragment;
use std::f32::consts::PI;

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

pub fn fragment_shader_urano(fragment: &Fragment, uniforms: &Uniforms) -> Color {
   let color = Color::new(189.0, 219.0, 208.0); 
   color * fragment.intensity
}

pub fn fragment_shader_sun(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Define the colors
    let white = Color::new(255.0, 255.0, 255.0);  // White center
    let yellow = Color::new(255.0, 255.0, 0.0);  // Yellow outline

    // Define the radius of the sun (the size of the sun's edge)
    let sun_radius = 0.8;  // You can adjust this as needed

    // Calculate distance from the center of the sun in the X-Y plane (ignore Z)
    let distance_from_center = (fragment.vertex_position.x.powi(2) + fragment.vertex_position.y.powi(2)).sqrt();

    // Ensure that the distance from the center is within the sun's radius
    let t = distance_from_center / sun_radius;

    // Lerp between white (center) and yellow (outline) based on the distance
    let color = if t < 1.0 {
        // Interpolate between white and yellow based on distance
        white.lerp(&yellow, t)
    } else {
        yellow  // Beyond the sun radius, we just use yellow
    };

    // Return the interpolated color
    color
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

// fragment_shader_saturn_with_ring function
pub fn fragment_shader_saturn_with_ring(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base color for Saturn
    let saturn_color = Color::new(210.0, 180.0, 140.0); 
    // Ring color (make it bright to distinguish)
    let ring_color = Color::new(255.0, 255.0, 255.0); 

    // Define ring parameters
    let ring_inner_radius = 0.48;  // Inner radius of the ring
    let ring_outer_radius = 0.50; // Outer radius of the ring
    let ring_z_offset = 0.55;     // Z offset for the flat ring (make sure this is small)

    // Calculate distance from the center in the X-Y plane (ignore Z)
    let distance_from_center = (fragment.vertex_position.x.powi(2) + fragment.vertex_position.z.powi(2)).sqrt();

    // Check if we are in the ring's radius range
    if distance_from_center >= ring_inner_radius && distance_from_center <= ring_outer_radius {
        // Calculate the LERP factor based on the distance
        let lerp_factor = (distance_from_center - ring_inner_radius) / (ring_outer_radius - ring_inner_radius);

        // Interpolate between Saturn color and ring color using the lerp factor
        let ring_color_lerp = saturn_color.lerp(&ring_color, lerp_factor);

        // Return the interpolated color with fragment intensity
        return ring_color_lerp * fragment.intensity;
    }

    // Return base Saturn color for other parts of the planet
    saturn_color * fragment.intensity
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0); // Clamp x between edge0 and edge1
    t * t * (3.0 - 2.0 * t) // Smooth interpolation
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

    // Blend clouds on top of the base color
    let final_color = if cloud_pattern > 0.7 {
        cloud_color// Blend clouds with land/sea
    } else {
        base_color
    };

    final_color * fragment.intensity
}

pub fn fragment_shader_mercury(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    // Base color for Mercury's surface (dark gray)
    let mercury_base_color = Color::new(169.0, 169.0, 169.0); // Dark gray

    // Color for the craters (lighter gray)
    let crater_color = Color::new(200.0, 200.0, 200.0); // Lighter gray for craters

    // Parameters to control the crater pattern
    let crater_size = 0.1;  // Size of the craters
    let crater_freq = 3.0;  // Frequency of craters (lower means larger, fewer craters)
    let randomness_factor = 0.6; // Control randomness level of craters
    let noise_scale = 10.0; // Scaling factor for noise

    // Combine multiple sine waves to simulate irregular patterns for craters
    let noise1 = (fragment.vertex_position.x * crater_freq).sin();
    let noise2 = (fragment.vertex_position.y * crater_freq).cos();
    
    // Random factor based on position for a more irregular pattern
    let random_factor = (fragment.vertex_position.x * noise_scale).sin() * (fragment.vertex_position.y * noise_scale).cos();

    // Combined crater pattern with randomness
    let crater_pattern = (noise1 + noise2 + random_factor).abs();

    // Threshold for determining if a crater exists
    let final_color = if crater_pattern > randomness_factor {
        crater_color // Crater color
    } else {
        mercury_base_color // Base color for surface
    };

    // Apply fragment intensity for lighting effects
    final_color * fragment.intensity
}

pub fn fragment_shader_moon(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_color = Color::new(200.0, 200.0, 200.0); // Gray base color for the moon
    let crater_color = Color::new(100.0, 100.0, 100.0); // Darker gray for craters

    // Simple crater effect based on fragment position
    let crater_intensity = ((fragment.vertex_position.x * 10.0).sin() * 
                            (fragment.vertex_position.y * 10.0).sin()).abs();
    
    // Blend colors based on intensity
    base_color.lerp(&crater_color, crater_intensity)
}

pub fn fragment_shader_ring(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base color for Saturn's ring
    let ring_color = Color::new(255.0, 255.0, 255.0); // White for the ring's primary color

    // Define ring parameters
    let ring_inner_radius = 0.48;  // Inner radius of the ring
    let ring_outer_radius = 0.50;  // Outer radius of the ring
    let ring_z_offset = 0.55;      // Z offset for the flat ring

    // Calculate distance from the center in the X-Y plane (ignore Z)
    let distance_from_center = (fragment.vertex_position.x.powi(2) + fragment.vertex_position.z.powi(2)).sqrt();

    // Check if we are in the ring's radius range
    if distance_from_center >= ring_inner_radius && distance_from_center <= ring_outer_radius {
        // Use a sine function to create periodic bands in the ring, giving it a more textured appearance
        let band_frequency = 15.0;  // Number of bands in the ring
        let band_intensity = (fragment.vertex_position.x * band_frequency).sin() * 0.5 + 0.5; // Sine wave pattern

        // Smooth interpolation based on distance from the center to create a soft edge for the ring
        let lerp_factor = (distance_from_center - ring_inner_radius) / (ring_outer_radius - ring_inner_radius);

        // Combine the band intensity and smooth transition to create the final color effect
        return ring_color * band_intensity * lerp_factor * fragment.intensity;
    }

    // Return black color for areas outside the ring
    Color::new(0.0, 0.0, 0.0) * fragment.intensity
}











