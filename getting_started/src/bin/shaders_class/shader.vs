#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 ourColor;

// Exercise 2
uniform float xOffset;

// Exercise 3
out vec3 ourPosition;

void main()
{
    gl_Position = vec4(aPos, 1.0);
    ourColor = aColor; 

    // Exercise 1
    // adjust y position for upside down triangle
    // gl_Position = vec4(aPos.x, -aPos.y, aPos.z, 1.0);

    // Exercise 2
    // add xOffset to the vertix x position
    // gl_Position = vec4(aPos.x + xOffset, aPos.y, aPos.z, 1.0);

    // Exercise 3
    // gl_Position = vec4(aPos, 1.0f);
    // ourPosition = aPos; 

    /* 
    Answer to the question: Do you know why the bottom-left side is black?
    ----------------------------------------------------------------------
    Think about this for a second: the output of our fragment's color is equal to the (interpolated) coordinate of 
    the triangle. What is the coordinate of the bottom-left point of our triangle? This is (-0.5f, -0.5f, 0.0f). Since the
    xy values are negative they are clamped to a value of 0.0f. This happens all the way to the center sides of the 
    triangle since from that point on the values will be interpolated positively again. Values of 0.0f are of course black
a   nd that explains the black side of the triangle.
*/
}