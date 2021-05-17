#version 450
layout(location=0) in vec2 vTexCoord;
// `layout(location=0)` means that the value of f_color will be saved to whatever buffer is at location 0 in our application.
// In most cases, location=0 is the current texture from the swapchain aka the screen
layout(location=0) out vec4 f_color;

layout(set=0, binding = 0) uniform texture2D t_diffuse; // `set = 0` correspond to render_pass.set_bind_group(0, ..)
layout(set=0, binding = 1) uniform sampler s_diffuse;

void main() {
    f_color = texture(sampler2D(t_diffuse, s_diffuse), vTexCoord);
}