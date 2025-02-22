#ifdef GL_ES
precision highp float;
#endif

in vec4 vertexColor_out;

out vec4 color_out;

void main()
{
    color_out = vertexColor_out;
}
