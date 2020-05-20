
## custom shader inputs

### global
```glsl
// vertex position
varying vec3 v_pos;
// vertex normal
varying vec3 v_normal;
// vertex texture coord
varying vec2 v_uv;
// vertex texture coord
varying vec4 v_color;
```

### vert

```glsl
// model matrix
uniform mat4 u_model;
// view matrix
uniform mat4 u_view;
// projection matrix
uniform mat4 u_proj;

// default vertex position
vec4 default_pos();
```

### frag

```glsl
// texture
uniform sampler2D u_tex;
// uniform color
uniform vec4 u_color;

// default fragment color
vec4 default_color();
```

