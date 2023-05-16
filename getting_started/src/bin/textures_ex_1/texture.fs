#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

// texture sampler
uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    // flip texture horizontally
    vec2 FlipH = vec2(-TexCoord.x, TexCoord.y);
    // flip texture vertically
    vec2 FlipV = vec2(TexCoord.x, -TexCoord.y);
    // take flipped
    vec2 Flipped = vec2(FlipH.x, FlipV.y);
    FragColor = mix(texture(texture1, TexCoord), texture(texture2, Flipped), 0.2);
}