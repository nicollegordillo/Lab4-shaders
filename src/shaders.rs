use nalgebra_glm::{Vec3, Vec4};
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
	let transformed = uniforms.viewport_matrix * uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

	let W = transformed.w;
	let transformed_position = Vec3::new(
		transformed.x / W,
		transformed.y / W,
		transformed.z / W
	);

	Vertex {
		position: vertex.position,
		normal: vertex.normal,
		tex_coords: vertex.tex_coords,
		color: vertex.color,
		transformed_position,
		transformed_normal: vertex.normal,
	}
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms)->Color {
	fragment.color * fragment.intensity
}