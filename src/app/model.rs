// wengwengweng

use std::rc::Rc;
use std::io::Cursor;
use std::path::Path;

use crate::*;
use super::*;
use super::gfx::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
	pub ambient: Color,
	pub diffuse: Color,
	pub specular: Color,
	pub shininess: f32,
}

impl Material {

	fn from_tobj(m: &tobj::Material) -> Self {

		let am = m.ambient;
		let df = m.diffuse;
		let sp = m.specular;

		return Self {
			ambient: rgba!(am[0], am[1], am[2], 1.0),
			diffuse: rgba!(df[0], df[1], df[2], 1.0),
			specular: rgba!(sp[0], sp[1], sp[2], 1.0),
			shininess: m.shininess,
		};

	}

}

impl Default for Material {
	fn default() -> Self {
		return Self {
			ambient: rgba!(),
			diffuse: rgba!(),
			specular: rgba!(),
			shininess: 0.0,
		};
	}
}

/// mesh data
#[derive(Clone)]
pub struct MeshData {
	pub vertices: Vec<Vertex3D>,
	pub indices: Vec<u32>,
	pub transform: gfx::Transform,
}

/// model data
#[derive(Clone)]
pub struct ModelData {
	meshes: Vec<MeshData>,
	img: Option<img::Image>,
}

#[derive(Clone)]
pub struct Mesh {
	gl_mesh: Rc<gl::Mesh<Vertex3D, Uniform3D>>,
	data: MeshData,
}

impl Mesh {
	pub(super) fn gl_mesh(&self) -> &gl::Mesh<Vertex3D, Uniform3D> {
		return &self.gl_mesh;
	}
	pub fn data(&self) -> &MeshData {
		return &self.data;
	}
}

/// 3d model
#[derive(Clone)]
pub struct Model {
	meshes: Vec<Mesh>,
	bound: (Vec3, Vec3),
	center: Vec3,
	texture: Option<Texture>,
}

fn handle_gltf_node(bin: &[u8], meshes: &mut Vec<MeshData>, ptransform: gfx::Transform, node: gltf::Node) {

	let mat = node.transform().matrix();

	let transform = gfx::Transform::from_mat4(mat4!(
		mat[0][0], mat[0][1], mat[0][2], mat[0][3],
		mat[1][0], mat[1][1], mat[1][2], mat[1][3],
		mat[2][0], mat[2][1], mat[2][2], mat[2][3],
		mat[3][0], mat[3][1], mat[3][2], mat[3][3],
	));

	let transform = ptransform.apply(&transform);

	if let Some(mesh) = node.mesh() {

		for prim in mesh.primitives() {

			let reader = prim.reader(|_| Some(&bin));

			let positions = reader
				.read_positions()
				.map(|positions| {
					return positions
						.map(|v| vec3!(v[0], v[1], v[2]))
						.collect::<Vec<Vec3>>();
				})
				.unwrap_or(vec![]);

			let indices = reader
				.read_indices()
				.map(|indices| {
					return indices
						.into_u32()
						.collect::<Vec<u32>>();
				})
				.unwrap_or(vec![]);

			let normals = reader
				.read_normals()
				.map(|normals| {
					return normals
						.map(|v| vec3!(v[0], v[1], v[2]))
						.collect::<Vec<Vec3>>();
				})
				.unwrap_or(vec![]);

			let normals = if normals.len() != positions.len() {
				gen_normals(&positions, &indices)
			} else {
				normals
			};

			let colors = reader
				.read_colors(0)
				.map(|colors| {
					return colors
						.into_rgba_f32()
						// gltf/glb exported by blender uses linear color space
						.map(|c| rgba!(c[0], c[1], c[2], c[3]).to_srgb())
						.collect::<Vec<Color>>();
				})
				.unwrap_or(vec![]);

			let texcoords = reader
				.read_tex_coords(0)
				.map(|texcoords| {
					return texcoords
						.into_f32()
						.map(|t| vec2!(t[0], t[1]))
						.collect::<Vec<Vec2>>();
				})
				.unwrap_or(vec![]);

			let mut verts = Vec::with_capacity(positions.len());

			for i in 0..positions.len() {

				let v = Vertex3D {
					pos: positions[i],
					normal: normals[i],
					color: colors.get(i).cloned().unwrap_or(rgba!(1)),
					uv: texcoords.get(i).cloned().unwrap_or(vec2!(0)),
				};

				verts.push(v);

			}

			meshes.push(MeshData {
				vertices: verts,
				indices: indices,
				transform: transform,
			});

		}

	}

	for cnode in node.children() {
		handle_gltf_node(bin, meshes, transform, cnode);
	}

}

impl Model {

	pub fn load_glb(bytes: &[u8]) -> Result<ModelData> {

		use gltf::Glb;
		use gltf::Gltf;

		let glb = Glb::from_slice(bytes)?;
		let document = Gltf::from_slice(&glb.json)?;
		let bin = glb.bin.ok_or_else(|| Error::Gltf(format!("no bin")))?;

		use gltf::image::Source;

		let mut img = None;

		for image in document.images() {

			match image.source() {

				Source::View { view, .. } => {

					let offset = view.offset();
					let len = view.length();
					let buf = &bin[offset..offset + len];

					img = Some(img::Image::from_bytes(buf)?);

				},

				_ => {},

			}

		}

		let mut meshes = Vec::with_capacity(document.meshes().len());

		for scene in document.scenes() {
			for node in scene.nodes() {
				handle_gltf_node(&bin, &mut meshes, gfx::Transform::new(), node);
			}
		}

		return Ok(ModelData {
			meshes: meshes,
			img: img,
		});

	}

	/// load mesh data with materials that's safe to send between threads
	pub fn load_obj(obj: &str, mtl: Option<&str>, img: Option<&[u8]>) -> Result<ModelData> {

		let (models, materials) = tobj::load_obj_buf(&mut Cursor::new(obj), |_| {
			return mtl
				.map(|m| tobj::load_mtl_buf(&mut Cursor::new(m)))
				.unwrap_or(Ok((vec![], hmap![])));
		})?;

		let mut meshes = Vec::with_capacity(models.len());

		for m in models {

			let m = m.mesh;
			let positions = m.positions
				.chunks(3)
				.map(|n| vec3!(n[0], n[1], n[2]))
				.collect::<Vec<Vec3>>();

			let vert_count = positions.len();
			let mut verts = Vec::with_capacity(vert_count);

			let normals = if m.normals.len() != vert_count * 3 {
				gen_normals(&positions, &m.indices)
			} else {
				m.normals
					.chunks(3)
					.map(|n| vec3!(n[0], n[1], n[2]))
					.collect()
			};

			let mtl = m.material_id
				.map(|id| materials.get(id))
				.flatten()
				.map(|m| Material::from_tobj(m))
				.unwrap_or_default();

			for i in 0..vert_count {

				let vx = m.positions[i * 3 + 0];
				let vy = m.positions[i * 3 + 1];
				let vz = m.positions[i * 3 + 2];

				let tx = m.texcoords.get(i * 2 + 0).cloned().unwrap_or(0.0);
				let ty = m.texcoords.get(i * 2 + 1).cloned().unwrap_or(0.0);

				verts.push(Vertex3D {
					pos: vec3!(vx, vy, vz),
					normal: normals[i],
					uv: vec2!(tx, 1.0 - ty),
					color: mtl.diffuse.to_srgb(),
				});

			}

			meshes.push(MeshData {
				vertices: verts,
				indices: m.indices,
				transform: gfx::Transform::new(),
			});

		}

		let img = if let Some(bytes) = img {
			Some(img::Image::from_bytes(bytes)?)
		} else {
			None
		};

		return Ok(ModelData {
			meshes: meshes,
			img: img,
		});

	}

	/// create model with mesh data
	pub fn from_data(ctx: &Ctx, data: ModelData) -> Result<Self> {

		let meshdata = data.meshes;
		let mut meshes = Vec::with_capacity(meshdata.len());

		let (min, max) = get_bound(&meshdata);

		let tex = if let Some(img) = data.img {
			Some(Texture::from_img(ctx, img)?)
		} else {
			None
		};

		for m in meshdata {
			meshes.push(Mesh {
				gl_mesh: Rc::new(gl::Mesh::from2(&ctx.gl, &m.vertices, &m.indices)?),
				data: m,
			});
		}

		let center = (min + max) / 2.0;

		return Ok(Self {
			meshes: meshes,
			bound: (min, max),
			center: center,
			texture: tex,
		});

	}

	/// create model from obj
	pub fn from_obj(ctx: &Ctx, obj: &str, mtl: Option<&str>, img: Option<&[u8]>) -> Result<Self> {
		return Self::from_data(ctx, Self::load_obj(obj, mtl, img)?);
	}

	/// create model from glb
	pub fn from_glb(ctx: &Ctx, bytes: &[u8]) -> Result<Self> {
		return Self::from_data(ctx, Self::load_glb(bytes)?);
	}

	pub fn meshes(&self) -> &[Mesh] {
		return &self.meshes;
	}

	pub fn set_texture(&mut self, tex: Texture) {
		self.texture = Some(tex);
	}

	pub fn texture(&self) -> Option<&Texture> {
		return self.texture.as_ref();
	}

	// TODO
	pub fn update(&mut self, f: impl Fn(&mut MeshData)) {

		use gl::VertexLayout;

		for m in &mut self.meshes {
			f(&mut m.data);
		}

// 		for (i, m) in self.meshes.iter().enumerate() {

// 			if let Some(mesh) = self.meshes.get(i) {

// 				let mut queue = Vec::with_capacity(m.vertices.len() * Vertex3D::STRIDE);

// 				for v in &m.vertices {
// 					v.push(&mut queue);
// 				}

// 				mesh.vbuf().data(0, &queue);
// 				mesh.ibuf().data(0, &m.indices);

// 			}

// 		}

	}

	pub fn center(&self) -> Vec3 {
		return self.center;
	}

	pub fn bound(&self) -> (Vec3, Vec3) {
		return self.bound;
	}

}

fn get_bound(meshes: &[MeshData]) -> (Vec3, Vec3) {

	let mut min = vec3!();
	let mut max = vec3!();

	for m in meshes {

		for v in &m.vertices {

			let pos = m.transform * v.pos;

			min.x = f32::min(pos.x, min.x);
			min.y = f32::min(pos.y, min.y);
			min.z = f32::min(pos.z, min.z);
			max.x = f32::max(pos.x, max.x);
			max.y = f32::max(pos.y, max.y);
			max.z = f32::max(pos.z, max.z);

		}

	}

	return (min, max);

}

fn gen_normals(pos: &[Vec3], indices: &[u32]) -> Vec<Vec3> {

	let vert_count = pos.len();
	let mut normals = vec![vec3!(0); vert_count];

	indices
		.chunks(3)
		.for_each(|tri| {

			let i1 = tri[0] as usize;
			let i2 = tri[1] as usize;
			let i3 = tri[2] as usize;
			let v1 = pos[i1];
			let v2 = pos[i2];
			let v3 = pos[i3];
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

