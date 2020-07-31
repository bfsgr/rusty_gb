#version 330 core
in vec2 vtex;
out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, vtex);
}