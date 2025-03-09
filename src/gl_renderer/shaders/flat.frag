#ifdef GL_ES
precision highp float;
#endif

in vec4 vertex_color_out;

out vec4 color_out;

void main()
{
    color_out = vertex_color_out;
}
