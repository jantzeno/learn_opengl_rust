#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

// texture sampler
uniform sampler2D texture1;
uniform sampler2D texture2;
uniform float mixValue; // vary the amount the two textures are visible

void main()
{
	// linearly interpolate between both textures
    FragColor = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), mixValue); 
}