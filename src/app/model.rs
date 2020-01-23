// wengwengweng

use std::rc::Rc;
use std::io::Cursor;

use crate::*;
use super::*;
use super::gfx::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
	pub diffuse: f32,
	pub specular: f32,
	pub shininess: f32,
}

impl Default for Material {
	fn default() -> Self {
		return Self {
			diffuse: 0.0,
			specular: 0.0,
			shininess: 0.0,
		};
	}
}

/// mesh data
#[derive(Clone)]
pub struct MeshData {
	pub vertices: Vec<Vertex3D>,
	pub indices: Vec<u32>,
}

#[derive(Clone)]
pub struct NodeData {
	pub id: usize,
	pub children: Vec<usize>,
	pub transform: Transform,
	pub meshes: Option<Vec<MeshData>>,
}

/// model data
#[derive(Clone)]
pub struct ModelData {
	nodes: HashMap<usize, NodeData>,
	scene: Vec<usize>,
	img: Option<img::Image>,
	anims: HashMap<usize, Anim>,
}

#[derive(Clone)]
pub struct Mesh {
	gl_mesh: Rc<gl::Mesh<Vertex3D, Uniform3D>>,
	data: MeshData,
}

#[derive(Clone)]
pub struct Node {
	pub meshes: Option<Vec<Mesh>>,
	pub id: usize,
	pub children: Vec<usize>,
	pub transform: Transform,
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
	nodes: HashMap<usize, Node>,
	anims: HashMap<usize, Anim>,
	scene: Vec<usize>,
	bound: (Vec3, Vec3),
	texture: Option<Texture>,
}

// TODO: deal with non-mesh nodes
fn read_gltf_node(bin: &[u8], nodes: &mut HashMap<usize, NodeData>, node: gltf::Node) {

	let id = node.index();
	let (pos, rot, scale) = node.transform().decomposed();

	let transform = Transform {
		pos: vec3!(pos[0], pos[1], pos[2]),
		rot: vec4!(rot[0], rot[1], rot[2], rot[3]),
		scale: vec3!(scale[0], scale[1], scale[2]),
	};

	let meshes = node.mesh().map(|mesh| {

		return mesh
			.primitives()
			.map(|prim| {

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

			return MeshData {
				vertices: verts,
				indices: indices,
			};

		}).collect();

	});

	nodes.insert(id, NodeData {
		id: id,
		children: node.children().map(|c| c.index()).collect(),
		transform: transform,
		meshes: meshes,
	});

	for c in node.children() {
		read_gltf_node(bin, nodes, c);
	}

}

type Track<T> = Vec<(f32, T)>;

#[derive(Clone, Debug)]
pub struct Anim {
	pos: Option<Track<Vec3>>,
	rot: Option<Track<Vec4>>,
	scale: Option<Track<Vec3>>,
}

fn get_track_val<T: Lerpable>(track: &Track<T>, t: f32) -> Option<T> {

	if let Some((k, _)) = track.first() {
		if *k > t {
			return None;
		}
	}

	if let Some((k, val)) = track.last() {
		if *k <= t {
			return Some(*val);
		}
	}

	for i in 0..track.len() - 1 {

		let (k1, pos1) = track[i];
		let (k2, pos2) = track[i + 1];

		if t >= k1 && t <= k2 {
			let dt = t - k1;
			return Some(pos1.lerp(pos2, dt));
		}

	}

	return None;

}

impl Anim {

	pub fn len(&self) -> f32 {

		let t1 = self.pos
			.as_ref()
			.map(|track| track.last().map(|(t, _)| *t))
			.flatten()
			.unwrap_or(0.0);

		let t2 = self.rot.as_ref()
			.map(|track| track.last().map(|(t, _)| *t))
			.flatten()
			.unwrap_or(0.0);

		let t3 = self.scale.as_ref()
			.map(|track| track.last().map(|(t, _)| *t))
			.flatten()
			.unwrap_or(0.0);

		return t1.max(t2).max(t3);

	}

	pub fn get_transform(&self, t: f32) -> (Option<Vec3>, Option<Vec4>, Option<Vec3>) {

		return (
			self.pos
				.as_ref()
				.map(|track| get_track_val(&track, t))
				.flatten(),
			self.rot
				.as_ref()
				.map(|track| get_track_val(&track, t))
				.flatten(),
			self.scale
				.as_ref()
				.map(|track| get_track_val(&track, t))
				.flatten(),
		);

	}

}

impl Model {

	pub fn load_glb(bytes: &[u8]) -> Result<ModelData> {

		use gltf::Glb;
		use gltf::Gltf;

		// init
		let glb = Glb::from_slice(bytes)?;
		let document = Gltf::from_slice(&glb.json)?;
		let bin = glb.bin.ok_or_else(|| Error::Gltf(format!("no bin")))?;

		// image
		use gltf::image::Source;

		let mut img = None;

		for i in document.images() {

			match i.source() {

				Source::View { view, .. } => {

					let offset = view.offset();
					let len = view.length();
					let buf = &bin[offset..offset + len];

					img = Some(img::Image::from_bytes(buf)?);

				},

				_ => {},

			}

		}

		// anims
		let mut anims: HashMap<usize, Anim> = hmap![];

		for a in document.animations() {

			for c in a.channels() {

				let reader = c.reader(|_| Some(&bin));
				let node_id = c.target().node().index();
				let sampler = c.sampler();

				let mut anim = anims.entry(node_id).or_insert(Anim {
					pos: None,
					rot: None,
					scale: None,
				});

				// TODO
				use gltf::animation::Interpolation;

				match sampler.interpolation() {
					Interpolation::Linear => {},
					Interpolation::Step => {},
					Interpolation::CubicSpline => {},
				};

				let times: Vec<f32> = reader
					.read_inputs()
					.ok_or(Error::Gltf(format!("failed to read anim")))?
					.collect();

				use gltf::animation::util::ReadOutputs;

				match reader
					.read_outputs()
					.ok_or(Error::Gltf(format!("failed to read anim")))? {

					ReadOutputs::Translations(translations) => {
						let mut values = Vec::with_capacity(times.len());
						for (i, v) in translations.enumerate() {
							let t = times
								.get(i)
								.ok_or(Error::Gltf(format!("failed to read anim")))?;
							values.push((*t, vec3!(v[0], v[1], v[2])));
						}
						anim.pos = Some(values);
					}

					ReadOutputs::Rotations(rotations) => {
						let mut values = Vec::with_capacity(times.len());
						for (i, v) in rotations.into_f32().enumerate() {
							let t = times
								.get(i)
								.ok_or(Error::Gltf(format!("failed to read anim")))?;
							values.push((*t, vec4!(v[0], v[1], v[2], v[3])));
						}
						anim.rot = Some(values);
					}

					ReadOutputs::Scales(scales) => {
						let mut values = Vec::with_capacity(times.len());
						for (i, v) in scales.enumerate() {
							let t = times
								.get(i)
								.ok_or(Error::Gltf(format!("failed to read anim")))?;
							values.push((*t, vec3!(v[0], v[1], v[2])));
						}
						anim.scale = Some(values);
					}

					_ => {}

				};

			}

		}

		// mesh
		let mut nodes = HashMap::with_capacity(document.nodes().len());
		let mut scene = vec![];

		for s in document.scenes() {
			for n in s.nodes() {
				scene.push(n.index());
				read_gltf_node(&bin, &mut nodes, n);
			}
		}

		return Ok(ModelData {
			nodes: nodes,
			scene: scene,
			img: img,
			anims: anims,
		});

	}

	/// load mesh data with materials that's safe to send between threads
	pub fn load_obj(obj: &str, mtl: Option<&str>, img: Option<&[u8]>) -> Result<ModelData> {

		let (models, materials) = tobj::load_obj_buf(&mut Cursor::new(obj), |_| {
			return mtl
				.map(|m| tobj::load_mtl_buf(&mut Cursor::new(m)))
				.unwrap_or(Ok((vec![], hmap![])));
		})?;

		let mut scene = Vec::with_capacity(models.len());
		let mut nodes = HashMap::with_capacity(models.len());

		for (i, m) in models.into_iter().enumerate() {

			scene.push(i);

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

			let mcolor = m.material_id
				.map(|id| materials.get(id))
				.flatten()
				.map(|m| m.diffuse)
				.map(|c| rgba!(c[0], c[1], c[2], 1))
				.unwrap_or(rgba!(1))
				;

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
					color: mcolor,
				});

			}

			nodes.insert(i, NodeData {
				id: i,
				children: vec![],
				transform: gfx::Transform::new(),
				meshes: Some(vec![MeshData {
					vertices: verts,
					indices: m.indices,
				}]),
			});

		}

		let img = if let Some(bytes) = img {
			Some(img::Image::from_bytes(bytes)?)
		} else {
			None
		};

		return Ok(ModelData {
			nodes: nodes,
			scene: scene,
			img: img,
			anims: hmap![],
		});

	}

	/// create model with mesh data
	pub fn from_data(ctx: &Ctx, data: ModelData) -> Result<Self> {

		let tex = if let Some(img) = data.img {
			Some(Texture::from_img(ctx, img)?)
		} else {
			None
		};

		let anims = data.anims;
		let scene = data.scene;

		let nodes = data.nodes.into_iter().map(|(id, node)| {
			let meshes = node.meshes.map(|meshes| {
				return meshes.into_iter().map(|m| {
					return Mesh {
						// TODO: no unwrap
						gl_mesh: Rc::new(gl::Mesh::from2(&ctx.gl, &m.vertices, &m.indices).unwrap()),
						data: m,
					};
				}).collect::<Vec<Mesh>>();
			});
			return (id, Node {
				id: node.id,
				children: node.children,
				transform: node.transform,
				meshes: meshes,
			});
		}).collect::<HashMap<usize, Node>>();

		return Ok(Self {
			bound: get_bound(&nodes, &scene),
			nodes: nodes,
			anims: anims,
			scene: scene,
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

	pub fn get_node(&self, id: usize) -> Option<&Node> {
		return self.nodes.get(&id);
	}

	pub fn get_anim(&self, id: usize) -> Option<&Anim> {
		return self.anims.get(&id);
	}

	pub fn scene(&self) -> &[usize] {
		return &self.scene;
	}

	pub fn set_texture(&mut self, tex: Texture) {
		self.texture = Some(tex);
	}

	pub fn texture(&self) -> Option<&Texture> {
		return self.texture.as_ref();
	}

	pub fn center(&self) -> Vec3 {
		let (min, max) = self.bound();
		return (min + max) / 2.0
	}

	pub fn bound(&self) -> (Vec3, Vec3) {
		return self.bound;
	}

}

fn get_bound_inner(
	min: &mut Vec3,
	max: &mut Vec3,
	transform: Mat4,
	nodes: &HashMap<usize, Node>,
	scene: &[usize],
) {

	for id in scene {

		if let Some(node) = nodes.get(id) {

			let tr = transform * node.transform.as_mat4();

			if let Some(meshes) = &node.meshes {

				for m in meshes {

					for v in &m.data.vertices {

						let pos = tr * v.pos;

						min.x = f32::min(pos.x, min.x);
						min.y = f32::min(pos.y, min.y);
						min.z = f32::min(pos.z, min.z);
						max.x = f32::max(pos.x, max.x);
						max.y = f32::max(pos.y, max.y);
						max.z = f32::max(pos.z, max.z);

					}

				}

			}

			get_bound_inner(min, max, tr, nodes, &node.children);

		}

	}

}

fn get_bound(nodes: &HashMap<usize, Node>, scene: &[usize]) -> (Vec3, Vec3) {

	let mut min = vec3!();
	let mut max = vec3!();

	get_bound_inner(&mut min, &mut max, mat4!(), nodes, scene);

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

