
## custom shader inputs

```glsl
// vertex position
varying vec3 v_pos;
// vertex normal
varying vec3 v_normal;
// vertex texture coord
varying vec2 v_uv;
// vertex texture coord
varying vec4 v_color;

// model matrix
uniform mat4 u_model;
// view matrix
uniform mat4 u_view;
// projection matrix
uniform mat4 u_proj;
// texture
uniform sampler2D u_tex;
// uniform color
uniform vec4 u_color;

// default vertex position
vec4 default_pos();
// default fragment color
vec4 default_color();
```

