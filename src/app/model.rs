// wengwengweng

use std::rc::Rc;
use std::io::Cursor;

use crate::*;
use super::*;
use super::gfx::*;

/// 3d model
#[derive(Clone)]
pub struct Model {
	meshes: Vec<Rc<gl::Mesh<Vertex3D, Uniform3D>>>,
}

/// 3d model mesh data
#[derive(Clone)]
pub struct ModelData(Vec<(Vec<f32>, Vec<u32>)>);

impl Model {

	// TODO: combine this with the one below
	/// load mesh data with materials that's safe to send between threads
	pub fn prepare_obj_mtl(obj: &str, mtl: &str) -> Result<ModelData> {

		use gl::VertexLayout;

		let (models, materials) = tobj::load_obj_buf(&mut Cursor::new(obj), |_| {
			return tobj::load_mtl_buf(&mut Cursor::new(mtl));
		})?;

		let mut meshes = Vec::with_capacity(models.len());

		for m in models {

			let m = m.mesh;
			let vert_count = m.positions.len() / 3;
			let mut verts = Vec::with_capacity(vert_count * Vertex3D::STRIDE);

			let normals = if m.normals.is_empty() {
				gen_vertex_normals(&m.positions, &m.indices)
			} else {
				m.normals
					.chunks(3)
					.map(|n| vec3!(n[0], n[1], n[2]))
					.collect()
			};

			let mtl = match m.material_id {
				Some(id) => materials.get(id),
				None => None,
			};

			let color = mtl
				.map(|m| m.diffuse)
				.map(|d| color!(d[0], d[1], d[2], 1.0))
				.unwrap_or(color!(rand!(), rand!(), rand!(), 1));

			for i in 0..vert_count {

				let vx = m.positions[i * 3 + 0];
				let vy = m.positions[i * 3 + 1];
				let vz = m.positions[i * 3 + 2];

				let vert = Vertex3D {
					pos: vec3!(vx, vy, vz),
					normal: normals[i],
					uv: vec2!(),
					color: color,
				};

				vert.push(&mut verts);

			}

			meshes.push((verts, m.indices));

		}

		return Ok(ModelData(meshes));

	}

	/// load mesh data that's safe to send between threads
	pub fn prepare_obj(obj: &str) -> Result<ModelData> {

		use gl::VertexLayout;

		let (models, _) = tobj::load_obj_buf(&mut Cursor::new(obj), |_| {
			return Err(tobj::LoadError::GenericFailure);
		})?;

		let mut meshes = Vec::with_capacity(models.len());

		for m in models {

			let m = m.mesh;
			let vert_count = m.positions.len() / 3;
// 			let normals = gen_vertex_normals(&m.positions, &m.indices);
			let mut verts = Vec::with_capacity(vert_count * Vertex3D::STRIDE);

			let normals = if m.normals.is_empty() {
				gen_vertex_normals(&m.positions, &m.indices)
			} else {
				m.normals
					.chunks(3)
					.map(|n| vec3!(n[0], n[1], n[2]))
					.collect()
			};

			for i in 0..vert_count {

				let vx = m.positions[i * 3 + 0];
				let vy = m.positions[i * 3 + 1];
				let vz = m.positions[i * 3 + 2];

				let vert = Vertex3D {
					pos: vec3!(vx, vy, vz),
					normal: normals[i],
					uv: vec2!(),
					color: color!(rand!(), rand!(), rand!(), 1),
				};

				vert.push(&mut verts);

			}

			meshes.push((verts, m.indices));

		}

		return Ok(ModelData(meshes));

	}

	/// create model with mesh data
	pub fn from(ctx: &Ctx, models: ModelData) -> Result<Self> {

// 		let meshes = models
// 			.into_iter()
// 			.map(|m| Rc::new(gl::Mesh::new(&ctx.gl, &m.verts, &m.indices)?))
// 			.collect();

		let mut meshes = Vec::with_capacity(models.0.len());

		for m in models.0 {
			meshes.push(Rc::new(gl::Mesh::new(&ctx.gl, &m.0, &m.1)?));
		}

		return Ok(Self {
			meshes: meshes,
		});

	}

	/// create model from obj
	pub fn from_obj(ctx: &Ctx, obj: &str) -> Result<Self> {
		return Self::from(ctx, Self::prepare_obj(obj)?);
	}

	/// create model from obj with materials
	pub fn from_obj_mtl(ctx: &Ctx, obj: &str, mtl: &str) -> Result<Self> {
		return Self::from(ctx, Self::prepare_obj_mtl(obj, mtl)?);
	}

	pub(super) fn meshes(&self) -> &[Rc<gl::Mesh<Vertex3D, Uniform3D>>] {
		return &self.meshes;
	}

}

// TODO
fn gen_surface_normals(pos: &[f32], indices: &[u32]) -> Vec<(Vec3, Vec3)> {

	let mut verts = Vec::with_capacity(pos.len());

	indices
		.chunks(3)
		.for_each(|tri| {

			let i1 = tri[0] as usize;
			let i2 = tri[1] as usize;
			let i3 = tri[2] as usize;
			let v1 = vec3!(pos[i1 * 3], pos[i1 * 3 + 1], pos[i1 * 3 + 2]);
			let v2 = vec3!(pos[i2 * 3], pos[i2 * 3 + 1], pos[i2 * 3 + 2]);
			let v3 = vec3!(pos[i3 * 3], pos[i3 * 3 + 1], pos[i3 * 3 + 2]);
			let normal = Vec3::cross((v2 - v1), (v3 - v1)).normalize();

			verts.push((v1, normal));
			verts.push((v2, normal));
			verts.push((v3, normal));

		});

	return verts;

}

fn gen_vertex_normals(pos: &[f32], indices: &[u32]) -> Vec<Vec3> {

	let vert_count = pos.len() / 3;
	let mut normals = vec![vec3!(0); vert_count];

	indices
		.chunks(3)
		.for_each(|tri| {

			let i1 = tri[0] as usize;
			let i2 = tri[1] as usize;
			let i3 = tri[2] as usize;
			let v1 = vec3!(pos[i1 * 3], pos[i1 * 3 + 1], pos[i1 * 3 + 2]);
			let v2 = vec3!(pos[i2 * 3], pos[i2 * 3 + 1], pos[i2 * 3 + 2]);
			let v3 = vec3!(pos[i3 * 3], pos[i3 * 3 + 1], pos[i3 * 3 + 2]);
			let normal = Vec3::cross((v2 - v1), (v3 - v1));

			normals[i1] += normal;
			normals[i2] += normal;
			normals[i3] += normal;

		});

	return normals
		.into_iter()
		.map(|p| p.normalize())
		.collect();

}

