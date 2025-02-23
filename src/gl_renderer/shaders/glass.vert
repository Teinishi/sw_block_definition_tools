in vec3 vertex_position_in;
in vec4 vertex_color_in;
in vec3 vertex_normal_in;

out vec3 vertex_world_position_out;
out vec3 vertex_normal_out;

uniform mat4 mat_world;
uniform mat4 mat_view_proj;

void main()
{
	vertex_normal_out = (mat_world * vec4(vertex_normal_in, 0.0)).xyz;

	vec4 world_pos = mat_world * vec4(vertex_position_in, 1);
	vertex_world_position_out = world_pos.xyz;
	gl_Position = mat_view_proj * world_pos;
}
