uniform mat4 matrix;

in vec2 pos;
in vec2 uv_coord;
in vec4 color;

out vec2 frag_uv;
out vec4 frag_color;

void main() {
    frag_uv = uv_coord;
    frag_color = color;

    gl_Position = matrix * vec4(pos.xy, 0, 1);
}