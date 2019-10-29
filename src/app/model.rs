// wengwengweng

use std::rc::Rc;
use std::io::Cursor;
use std::path::Path;

use crate::*;
use super::*;
use super::gfx::*;

/// mesh data
pub type MeshData = gl::MeshData<Vertex3D>;

/// model data
pub struct ModelData {
	meshes: Vec<MeshData>,
	img: Option<img::Image>,
}

/// 3d model
#[derive(Clone)]
pub struct Model {
	meshdata: Vec<MeshData>,
	bound: (Vec3, Vec3),
	center: Vec3,
	meshes: Vec<Rc<gl::Mesh<Vertex3D, Uniform3D>>>,
	texture: Option<Texture>,
}

impl Model {

	// TODO
	pub fn from_shape(ctx: &Ctx, s: impl gl::Shape<Vertex = Vertex3D>) -> Result<Self> {
		let mesh = gl::Mesh::from_shape(&ctx.gl, s)?;
		return Ok(Self {
			meshdata: vec![],
			meshes: vec![Rc::new(mesh)],
			center: vec3!(),
			bound: (vec3!(), vec3!()),
			texture: None,
		});
	}

	pub fn load_file(path: impl AsRef<Path>) -> Result<ModelData> {

		let path = path.as_ref();

		match fs::extname(path)?.as_ref() {
			"obj" => {},
			"gltf" => {
				return Self::load_gltf(path);
			},
			_ => {},
		}

		todo!();

	}

	pub fn from_file(ctx: &Ctx, path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_data(ctx, Self::load_file(path)?);
	}

	// TODO: reuse code with load_glb
	pub fn load_gltf(path: impl AsRef<Path>) -> Result<ModelData> {

		let (document, buffers, images) = gltf::import(path)?;
		let dmeshes = document.meshes();
		let mut meshes = Vec::with_capacity(dmeshes.len());
		let mut img = None;

		// TODO: not correct
		for data in images {
			img = Some(img::Image::from_pixels(data.width as i32, data.height as i32, data.pixels)?);
		}

		for mesh in dmeshes {

			for prim in mesh.primitives() {

				let reader = prim.reader(|b| Some(&buffers[b.index()]));

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
							.map(|c| color!(c[0], c[1], c[2], c[3]))
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
						color: colors.get(i).cloned().unwrap_or(color!(1)),
						uv: texcoords.get(i).cloned().unwrap_or(vec2!(0)),
					};

					verts.push(v);

				}

				meshes.push(gl::MeshData {
					vertices: verts,
					indices: indices,
				});

			}

		}

		return Ok(ModelData {
			meshes: meshes,
			img: img,
		});

	}

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

		let dmeshes = document.meshes();
		let mut meshes = Vec::with_capacity(dmeshes.len());

		for mesh in dmeshes {

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
							.map(|c| color!(c[0], c[1], c[2], c[3]))
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
						color: colors.get(i).cloned().unwrap_or(color!(1)),
						uv: texcoords.get(i).cloned().unwrap_or(vec2!(0)),
					};

					verts.push(v);

				}

				meshes.push(gl::MeshData {
					vertices: verts,
					indices: indices,
				});

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
				.unwrap_or(Ok((vec![], hashmap![])));
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
				.flatten();

			let color = mtl
				.map(|m| m.diffuse)
				.map(|d| color!(d[0], d[1], d[2], 1.0))
				.unwrap_or(color!());

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
					color: color,
				});

			}

			meshes.push(gl::MeshData {
				vertices: verts,
				indices: m.indices,
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

		let tex = if let Some(img) = data.img {
			Some(Texture::from_img(ctx, img)?)
		} else {
			None
		};

		for m in &meshdata {
			meshes.push(Rc::new(gl::Mesh::from_meshdata(&ctx.gl, m.clone())?));
		}

		let (min, max) = get_bound(&meshdata);
		let center = (min + max) / 2.0;

		return Ok(Self {
			meshdata: meshdata,
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

	/// create model from gltf
	pub fn from_gltf(ctx: &Ctx, path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_data(ctx, Self::load_gltf(path)?);
	}

	pub(super) fn meshes(&self) -> &[Rc<gl::Mesh<Vertex3D, Uniform3D>>] {
		return &self.meshes;
	}

	pub fn set_texture(&mut self, tex: Texture) {
		self.texture = Some(tex);
	}

	pub fn texture(&self) -> Option<&Texture> {
		return self.texture.as_ref();
	}

	pub fn update(&mut self, f: impl FnOnce(&mut [MeshData])) {

		use gl::VertexLayout;

		f(&mut self.meshdata);

		let (min, max) = get_bound(&self.meshdata);

		self.center = (min + max) / 2.0;
		self.bound = (min, max);

		for (i, m) in self.meshdata.iter().enumerate() {

			if let Some(mesh) = self.meshes.get(i) {

				let mut queue = Vec::with_capacity(m.vertices.len() * Vertex3D::STRIDE);

				for v in &m.vertices {
					v.push(&mut queue);
				}

				mesh.vbuf().data(0, &queue);
				mesh.ibuf().data(0, &m.indices);

			}

		}

	}

	pub fn meshdata(&self) -> &[MeshData] {
		return &self.meshdata;
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

			let pos = v.pos;

			if pos.x < min.x {
				min.x = pos.x;
			}

			if pos.y < min.y {
				min.y = pos.y;
			}

			if pos.z < min.z {
				min.z = pos.z;
			}

			if pos.x > max.x {
				max.x = pos.x;
			}

			if pos.y > max.y {
				max.y = pos.y;
			}

			if pos.z > max.z {
				max.z = pos.z;
			}

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

