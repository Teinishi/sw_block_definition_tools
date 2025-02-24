#ifdef GL_ES
precision highp float;
#endif

in vec3 vertex_world_position_out;
in vec3 vertex_normal_out;

out vec4 color_out;

uniform vec3 camera_position;
uniform vec3 sky_color_up;
uniform vec3 sky_color_down;

void main()
{
    vec3 camera_to_fragment = vertex_world_position_out - camera_position;
    camera_to_fragment = camera_to_fragment / length(camera_to_fragment);
    vec3 normal_vector = normalize(vertex_normal_out);
    vec3 reflected_dir = reflect(camera_to_fragment, normal_vector);

	float reflection_factor = max(0.0, -dot(normal_vector, camera_to_fragment));
	reflection_factor = pow(1.0 - reflection_factor, 5.0);
	reflection_factor = clamp(reflection_factor, 0.0, 1.0);

    float angle_factor = dot(reflected_dir, vec3(0, 1, 0)) * 0.5 + 0.5;
    vec3 sky_color = mix(sky_color_down, sky_color_up, angle_factor).rgb;

    color_out = vec4((sky_color * reflection_factor) * 0.823, 0.0);
}
