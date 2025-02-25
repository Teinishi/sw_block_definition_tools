#ifdef GL_ES
precision highp float;
#endif

const float pi = 3.14159265;
const float light_factor = 0.9;
const vec3 sun_light_direction = normalize(vec3(0.0, -1.0, 0.5));
const vec3 sun_light_color = vec3(0.95, 0.95, 1.0);
const vec3 light_color = vec3(0.8, 0.8, 0.8);

in vec3 vertex_position_out;
in vec4 vertex_color_out;
in vec3 vertex_normal_out;

out vec4 color_out;

uniform vec3 camera_position;
uniform vec3 ambient_color_low;
uniform vec3 ambient_color_high;

vec3 fresnel(vec3 spec_color, float intensity)
{
    return spec_color + (vec3(1.0) - spec_color) * pow(1.0 - intensity, 5.0);
}

float ndf_ggx(float roughness, float n_dot_h)
{
	float a = roughness * roughness;
	float a2 = a * a;
	float d = (a2 - 1.0) * n_dot_h * n_dot_h + 1.0;
	return a2 / (pi * d * d);
}

vec3 brdf(vec3 light_color, vec3 light_direction, vec3 albedo, vec3 spec_color, vec3 normal, vec3 eye, float roughness, float specular_factor)
{
    vec3 half_vec = normalize(-eye - light_direction);

    float l_dot_n = max(0.0, -dot(light_direction, normal));
    float l_dot_h = max(0.0, -dot(light_direction, half_vec));
    float n_dot_h = max(0.0, dot(normal, half_vec));

    vec3 specular = fresnel(spec_color, l_dot_h) * ndf_ggx(roughness, n_dot_h) * light_color * l_dot_n;

    return light_color * albedo + pi * specular;
}

void main()
{
    vec3 color = vertex_color_out.rgb;
    color.r = pow(color.r, 2.2);
    color.g = pow(color.g, 2.2);
    color.b = pow(color.b, 2.2);
    vec3 normal = normalize(vertex_normal_out);
    vec3 camera_to_fragment = vertex_position_out - camera_position;
    vec3 eye = normalize(camera_to_fragment);

    float angle_factor = dot(normal, vec3(0, 1, 0)) * 0.5 + 0.5;
    vec3 ambient = mix(ambient_color_low, ambient_color_high, angle_factor).rgb;

    float sun_light_intensity = max(0.0, -dot(sun_light_direction, normal));
    sun_light_intensity = mix(1.0, sun_light_intensity, light_factor) * 0.75;
    vec3 sun_contribution = brdf(sun_light_color, sun_light_direction, color, vec3(0.02), normal, eye, 0.8, light_factor) * sun_light_intensity;

    float dist = length(camera_to_fragment);
    float incidence_factor = max(0.0, -dot(camera_to_fragment / (0.01 + dist), normal));
    float distance_factor = 0.05 * ((1.0 / max(0.01, dist)) - (1.0 / 100.0));

    vec3 surface_color = max(vec3(0), color * ambient + sun_contribution);
    surface_color += brdf(light_color * 0.2, camera_to_fragment / (0.01 + dist), color, vec3(0.02), normal, eye, 0.8, light_factor) * incidence_factor * distance_factor * 16;
    surface_color.r = pow(surface_color.r, 1 / 2.2);
    surface_color.g = pow(surface_color.g, 1 / 2.2);
    surface_color.b = pow(surface_color.b, 1 / 2.2);
    color_out = vec4(surface_color, 1.0);
}
