#version 330 core
out vec4 FragColor;

in vec3 ourColor;

// Exercise 3
in vec3 ourPosition;

void main()
{
    FragColor = vec4(ourColor, 1.0f);

    // Exercise 3
    // FragColor = vec4(ourPosition, 1.0f);
}