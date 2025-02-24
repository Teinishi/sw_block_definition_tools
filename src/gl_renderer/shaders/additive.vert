in vec3 vertex_position_in;
in vec4 vertex_color_in;

out vec4 vertex_color_out;

uniform mat4 mat_world;
uniform mat4 mat_view_proj;
uniform vec4 additive_color;

void main()
{
	gl_Position = mat_view_proj * mat_world * vec4(vertex_position_in, 1);
    gl_Position.z -= 0.00001;

	/*vertex_color_out = vertex_color_in;
    vertex_color_out.r = pow(additive_color.r, 2.2);
    vertex_color_out.g = pow(additive_color.g, 2.2);
    vertex_color_out.b = pow(additive_color.b, 2.2);*/
    vertex_color_out = additive_color;
}
