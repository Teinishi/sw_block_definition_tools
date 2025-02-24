in vec3 vertex_position_in;
in vec4 vertex_color_in;

out vec4 vertex_color_out;

uniform mat4 mat_view_proj;
uniform mat4 mat_world;

void main()
{
    gl_Position =  mat_view_proj * mat_world * vec4(vertex_position_in, 1);
    gl_Position.z -= 0.00001;
    vertex_color_out = vertex_color_in;
}
