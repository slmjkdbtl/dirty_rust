// wengwengweng

use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;

use serde::Serialize;
use serde::Deserialize;

use crate::*;
use gfx::*;
use geom::*;

type NodeID = usize;

// TODO: rework anim system

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct NodeData {
	pub id: NodeID,
	pub children: Vec<NodeID>,
	pub transform: Transform,
	pub meshes: Vec<MeshData>,
	pub name: Option<String>,
}

/// Data for Creating [`Model`](`struct.Model.html`)
#[derive(Clone, Serialize, Deserialize)]
pub struct ModelData {
	nodes: HashMap<NodeID, NodeData>,
	root_nodes: Vec<NodeID>,
	img: Option<img::Image>,
	anims: HashMap<NodeID, Anim>,
	anim_len: f32,
}

#[derive(Clone)]
pub(super) struct Node {
	meshes: Vec<Mesh>,
	id: NodeID,
	name: Option<String>,
	children: Vec<NodeID>,
	transform: Transform,
}

impl Node {
	pub fn transform(&self) -> Transform {
		return self.transform;
	}
	pub fn meshes(&self) -> &[Mesh] {
		return &self.meshes;
	}
	pub fn children(&self) -> &[NodeID] {
		return &self.children;
	}
}

type Track<T> = Vec<(f32, T)>;

/// 3D Animation Data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Anim {
	pos: Track<Vec3>,
	rot: Track<Vec4>,
	scale: Track<Vec3>,
}

fn get_track_val<T: Lerp>(track: &Track<T>, t: f32) -> Option<T> {

	if track.is_empty() {
		return None;
	}

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
			.last()
			.map(|(t, _)| *t)
			.unwrap_or(0.0);

		let t2 = self.rot
			.last()
			.map(|(t, _)| *t)
			.unwrap_or(0.0);

		let t3 = self.scale
			.last()
			.map(|(t, _)| *t)
			.unwrap_or(0.0);

		return t1
			.max(t2)
			.max(t3)
			;

	}

	pub fn get_transform(&self, t: f32) -> (Option<Vec3>, Option<Vec4>, Option<Vec3>) {

		return (
			get_track_val(&self.pos, t),
			get_track_val(&self.rot, t),
			get_track_val(&self.scale, t),
		);

	}

}

/// 3D Model
#[derive(Clone)]
pub struct Model {
	nodes: HashMap<NodeID, Node>,
	anims: HashMap<NodeID, Anim>,
	anim_len: f32,
	root_nodes: Vec<NodeID>,
	bbox: BBox,
	texture: Option<Texture>,
}

fn read_gltf_node(bin: &[u8], nodes: &mut HashMap<NodeID, NodeData>, node: gltf::Node) {

	let id = node.index();
	let name = node.name();
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
				}).unwrap_or_default();

			let indices = reader
				.read_indices()
				.map(|indices| {
					return indices
						.into_u32()
						.collect::<Vec<u32>>();
				}).unwrap_or_default();

			let normals = reader
				.read_normals()
				.map(|normals| {
					return normals
						.map(|v| vec3!(v[0], v[1], v[2]))
						.collect::<Vec<Vec3>>();
				}).unwrap_or_default();

			let normals = if normals.len() != positions.len() {
				ops::gen_normals(&positions, &indices)
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
				}).unwrap_or_default();

			let texcoords = reader
				.read_tex_coords(0)
				.map(|texcoords| {
					return texcoords
						.into_f32()
						.map(|t| vec2!(t[0], t[1]))
						.collect::<Vec<Vec2>>();
				}).unwrap_or_default();

			let mut verts = Vec::with_capacity(positions.len());

			for i in 0..positions.len() {

				let v = Vertex {
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

	}).unwrap_or_default();

	nodes.insert(id, NodeData {
		id: id,
		name: name.map(String::from),
		children: node.children().map(|c| c.index()).collect(),
		transform: transform,
		meshes: meshes,
	});

	for c in node.children() {
		read_gltf_node(bin, nodes, c);
	}

}

impl Model {

	/// load [`ModelData`](struct.ModelData.html) from a file
	pub fn load_file(path: impl AsRef<Path>) -> Result<ModelData> {

		let mut path = path.as_ref().to_owned();

		match fs::extname(&path)?.as_ref() {

			"obj" => {

				let obj_src = fs::read_str(&path)?;

				path.set_extension("mtl");

				let mtl_src = fs::read_str(&path).ok();
				let mtl_src = mtl_src.as_deref();

				path.set_extension("png");

				let img_src = fs::read(&path).ok();
				let img_src = img_src.as_deref();

				let data = gfx::Model::load_obj(&obj_src, mtl_src, img_src)?;

				return Ok(data);

			},

			"glb" => {

				let bytes = fs::read(&path)?;
				let data = gfx::Model::load_glb(&bytes)?;

				return Ok(data);

			},

			_ => {
				return Err(format!("unsupported 3d format"));
			},

		}

	}

	/// load [`ModelData`](struct.ModelData.html) from [`MeshData`](struct.MeshData.html)
	pub fn load_meshdata(data: MeshData) -> ModelData {
		return Self::load_raw(data.vertices, data.indices);
	}

	/// load [`ModelData`](struct.ModelData.html) from raw vertices & indices
	pub fn load_raw(verts: Vec<Vertex>, indices: Vec<u32>) -> ModelData {

		let node = NodeData {
			id: 0,
			name: None,
			children: vec![],
			transform: Transform::new(),
			meshes: vec![MeshData {
				vertices: verts,
				indices: indices,
			}],
		};

		return ModelData {
			nodes: hmap![
				0 => node,
			],
			root_nodes: vec![0],
			img: None,
			anims: hmap![],
			anim_len: 0.0,
		};

	}

	/// load [`ModelData`](struct.ModelData.html) from glb bytes
	pub fn load_glb(bytes: &[u8]) -> Result<ModelData> {

		use gltf::Glb;
		use gltf::Gltf;

		// init
		let glb = Glb::from_slice(bytes)
			.map_err(|_| format!("failed to parse glb"))?;
		let document = Gltf::from_slice(&glb.json)
			.map_err(|_| format!("failed to parse document from glb"))?;
		let bin = glb.bin
			.ok_or_else(|| format!("failed to parse bin from glb"))?;

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
		let mut anims: HashMap<NodeID, Anim> = hmap![];

		for a in document.animations() {

			for c in a.channels() {

				let reader = c.reader(|_| Some(&bin));
				let node_id = c.target().node().index();
				let sampler = c.sampler();

				let mut anim = anims.entry(node_id).or_insert(Anim {
					pos: vec![],
					rot: vec![],
					scale: vec![],
				});

				// TODO
				use gltf::animation::Interpolation;

				match sampler.interpolation() {
					Interpolation::Linear => {},
					Interpolation::Step => {},
					Interpolation::CubicSpline => {},
				};

				let frames: Vec<f32> = reader
					.read_inputs()
					.ok_or_else(|| format!("failed to read anim"))?
					.collect();

				use gltf::animation::util::ReadOutputs;

				match reader
					.read_outputs()
					.ok_or_else(|| format!("failed to read anim"))? {

					ReadOutputs::Translations(translations) => {
						let mut values = Vec::with_capacity(frames.len());
						for (i, v) in translations.enumerate() {
							let t = frames
								.get(i)
								.ok_or_else(|| format!("failed to read anim from glb"))?;
							values.push((*t, vec3!(v[0], v[1], v[2])));
						}
						anim.pos = values;
					}

					ReadOutputs::Rotations(rotations) => {
						let mut values = Vec::with_capacity(frames.len());
						for (i, v) in rotations.into_f32().enumerate() {
							let t = frames
								.get(i)
								.ok_or_else(|| format!("failed to read anim from glb"))?;
							values.push((*t, vec4!(v[0], v[1], v[2], v[3])));
						}
						anim.rot = values;
					}

					ReadOutputs::Scales(scales) => {
						let mut values = Vec::with_capacity(frames.len());
						for (i, v) in scales.enumerate() {
							let t = frames
								.get(i)
								.ok_or_else(|| format!("failed to read anim from glb"))?;
							values.push((*t, vec3!(v[0], v[1], v[2])));
						}
						anim.scale = values;
					}

					_ => {}

				};

			}

		}

		let mut anim_len = 0.0f32;

		for a in anims.values() {
			anim_len = anim_len.max(a.len());
		}

		// mesh
		let mut nodes = HashMap::with_capacity(document.nodes().len());
		let mut root_nodes = vec![];

		for s in document.scenes() {
			for n in s.nodes() {
				root_nodes.push(n.index());
				read_gltf_node(&bin, &mut nodes, n);
			}
		}

		return Ok(ModelData {
			nodes,
			root_nodes,
			img,
			anims,
			anim_len,
		});

	}

	/// load [`ModelData`](struct.ModelData.html) from obj file
	pub fn load_obj(obj: &str, mtl: Option<&str>, img: Option<&[u8]>) -> Result<ModelData> {

		let (models, materials) = tobj::load_obj_buf(&mut Cursor::new(obj), true, |_| {
			return mtl
				.map(|m| tobj::load_mtl_buf(&mut Cursor::new(m)))
				.unwrap_or(Ok((vec![], hmap![])));
		}).map_err(|_| format!("failed to parse obj"))?;

		let mut root_nodes = Vec::with_capacity(models.len());
		let mut nodes = HashMap::with_capacity(models.len());

		for (i, m) in models.into_iter().enumerate() {

			root_nodes.push(i);

			let m = m.mesh;
			let positions = m.positions
				.chunks(3)
				.map(|n| vec3!(n[0], n[1], n[2]))
				.collect::<Vec<Vec3>>();

			let vert_count = positions.len();
			let mut verts = Vec::with_capacity(vert_count);

			let normals = if m.normals.len() != vert_count * 3 {
				ops::gen_normals(&positions, &m.indices)
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

				verts.push(Vertex {
					pos: vec3!(vx, vy, vz),
					normal: normals[i],
					uv: vec2!(tx, 1.0 - ty),
					color: mcolor,
				});

			}

			nodes.insert(i, NodeData {
				id: i,
				name: None,
				children: vec![],
				transform: gfx::Transform::new(),
				meshes: vec![MeshData {
					vertices: verts,
					indices: m.indices,
				}],
			});

		}

		let img = if let Some(bytes) = img {
			Some(img::Image::from_bytes(bytes)?)
		} else {
			None
		};

		return Ok(ModelData {
			nodes,
			root_nodes,
			img,
			anims: hmap![],
			anim_len: 0.0,
		});

	}

	/// create model from [`ModelData`](struct.ModelData.html)
	pub fn from_data(ctx: &impl GLCtx, data: ModelData) -> Result<Self> {

		let bbox = get_bbox(&data);

		let tex = if let Some(img) = data.img {
			Some(Texture::from_img(ctx, img)?)
		} else {
			None
		};

		let anims = data.anims;
		let anim_len = data.anim_len;
		let root_nodes = data.root_nodes;

		let nodes = data.nodes
			.into_iter()
			.map(|(id, node)| {

				let meshes = node.meshes
					.into_iter()
					// TODO: don't unwrap here
					.map(|m| Mesh::from_meshdata(ctx, &m).unwrap())
					.collect::<Vec<Mesh>>();

				return (id, Node {
					id: node.id,
					name: node.name,
					children: node.children,
					transform: node.transform,
					meshes: meshes,
				});

			})
			.collect::<HashMap<NodeID, Node>>();

		return Ok(Self {
			bbox: bbox,
			nodes,
			anim_len,
			anims,
			root_nodes,
			texture: tex,
		});

	}

	/// create model from a file
	pub fn from_file(ctx: &impl GLCtx, path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_data(ctx, Self::load_file(path)?);
	}

	/// create model from a [`MeshData`](struct.MeshData.html)
	pub fn from_meshdata(ctx: &impl GLCtx, data: MeshData) -> Result<Self> {
		return Self::from_data(ctx, Self::load_meshdata(data));
	}

	/// create model from raw vertices & indices
	pub fn from_raw(ctx: &impl GLCtx, verts: Vec<Vertex>, indices: Vec<u32>) -> Result<Self> {
		return Self::from_data(ctx, Self::load_raw(verts, indices));
	}

	/// create model from obj file
	pub fn from_obj(ctx: &impl GLCtx, obj: &str, mtl: Option<&str>, img: Option<&[u8]>) -> Result<Self> {
		return Self::from_data(ctx, Self::load_obj(obj, mtl, img)?);
	}

	/// create model from glb file
	pub fn from_glb(ctx: &impl GLCtx, bytes: &[u8]) -> Result<Self> {
		return Self::from_data(ctx, Self::load_glb(bytes)?);
	}

	pub(super) fn get_node(&self, id: NodeID) -> Option<&Node> {
		return self.nodes.get(&id);
	}

	pub fn get_anim(&self, id: NodeID) -> Option<&Anim> {
		return self.anims.get(&id);
	}

	pub fn anim_len(&self) -> f32 {
		return self.anim_len;
	}

	pub fn root_nodes(&self) -> &[NodeID] {
		return &self.root_nodes;
	}

	pub fn texture(&self) -> Option<&Texture> {
		return self.texture.as_ref();
	}

	/// get center position
	pub fn center(&self) -> Vec3 {
		return (self.bbox.min + self.bbox.max) / 2.0;
	}

	/// get bounding box
	pub fn bbox(&self) -> BBox {
		return self.bbox;
	}

}

fn get_bbox_inner(
	min: &mut Vec3,
	max: &mut Vec3,
	transform: Mat4,
	nodes: &HashMap<NodeID, NodeData>,
	list: &[NodeID],
) {

	for id in list {

		if let Some(node) = nodes.get(id) {

			let tr = transform * node.transform.as_mat4();

			for m in &node.meshes {

				for v in &m.vertices {

					let pos = tr * v.pos;

					min.x = f32::min(pos.x, min.x);
					min.y = f32::min(pos.y, min.y);
					min.z = f32::min(pos.z, min.z);
					max.x = f32::max(pos.x, max.x);
					max.y = f32::max(pos.y, max.y);
					max.z = f32::max(pos.z, max.z);

				}

			}

			get_bbox_inner(min, max, tr, nodes, &node.children);

		}

	}

}

fn get_bbox(model: &ModelData) -> BBox {

	let mut min = vec3!();
	let mut max = vec3!();

	get_bbox_inner(&mut min, &mut max, mat4!(), &model.nodes, &model.root_nodes);

	return BBox::new(min, max);

}

