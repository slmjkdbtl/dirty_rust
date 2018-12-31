// wengwengweng

type Poly = Vec<Vec2>;

use crate::*;
use crate::math::*;

fn get_axis(v: Poly) -> Poly {

	let normals = vec![];

	for i in 0..v.len() {

		let p1 = v[i];
// 		let p2 = v[i + 1] or v[0];
// 		let edge = p1 - p2;
// 		let normal = math.normalize(math.normal(edge))

// 		normals.push(normal);

	}

	return normals;

}

pub fn sat(p1: Poly, p2: Poly) -> (bool, Vec2) {

// 	local function get_axis(v)

// 		local normals = {}

// 		for i = 1, #v do

// 			local p1 = v[i]
// 			local p2 = v[i + 1] or v[1]
// 			local edge = p1 - p2
// 			local normal = math.normalize(math.normal(edge))

// 			normals[#normals + 1] = normal

// 		end

// 		return normals

// 	end

// 	local function project(v, a)

// 		local min = math.dot(a, v[1])
// 		local max = min

// 		for i = 2, #v do

// 			local proj = math.dot(a, v[i])

// 			if (proj < min) then
// 				min = proj
// 			elseif (proj > max) then
// 				max = proj
// 			end

// 		end

// 		return min, max

// 	end

// 	local axis = {}

// 	table.append(axis, get_axis(v1))
// 	table.append(axis, get_axis(v2))

// 	local mtv = vec2(0)
// 	local overlap = 99999999

// 	for i = 1, #axis do

// 		local a = axis[i]
// 		local s1min, s1max = project(v1, a)
// 		local s2min, s2max = project(v2, a)

// 		if (s1min > s2max or s2min > s1max) then
// 			return false, vec2(0)
// 		end

// 		local o = s2max - s1min

// 		if (o < overlap) then

// 			overlap = o
// 			mtv = a * o

// 		end

// 	end

// 	return true, mtv

	return (false, vec2!());

}

