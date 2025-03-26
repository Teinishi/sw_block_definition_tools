in vec3 vertex_position_in;
in vec4 vertex_color_in;

out vec4 vertex_color_out;

uniform float z_offset_unit;
uniform float z_offset;

uniform mat4 mat_view_proj;
uniform mat4 mat_world;

void main()
{
    gl_Position =  mat_view_proj * mat_world * vec4(vertex_position_in, 1);
    gl_Position.z += z_offset_unit * z_offset;

    vertex_color_out = vertex_color_in;
}
