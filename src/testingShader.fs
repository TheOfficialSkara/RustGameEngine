#version 330 core
in vec4 color;
out vec4 FragColor;
void main() {
   FragColor = vec4(color.x, color.y, color.z, 1.0);
}
