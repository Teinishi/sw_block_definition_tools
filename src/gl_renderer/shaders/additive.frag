in vec4 vertex_color_out;

out vec4 color_out;

void main()
{
    color_out = vec4(vertex_color_out.rgb, 1.0);
}
