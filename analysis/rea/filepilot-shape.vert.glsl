#version 330 core

uniform mat4 proj;

layout(location = 0) in vec2 p0;
layout(location = 1) in vec2 p1;
layout(location = 2) in vec2 p2;
layout(location = 3) in vec2 p3;
layout(location = 4) in vec2 vertMinUV;
layout(location = 5) in vec2 vertMaxUV;
layout(location = 6) in vec4 vertColor;
layout(location = 7) in vec4 vertGradient;
layout(location = 8) in vec4 vertCornerRadii;
layout(location = 9) in vec4 vertShapeThicknessMask;
layout(location = 10) in float vertDepth;
layout(location = 11) in float vertShapeThickness;
layout(location = 12) in float vertShapeSoftness;
layout(location = 13) in float vertDashLength;
layout(location = 14) in float vertGapLength;
layout(location = 15) in float vertDashCornerRad;

out vec2 fragShapePos;
out vec2 fragShapeDim;
smooth out vec2 fragUV;
smooth out vec4 fragColor;
smooth out vec4 fragGradient;
smooth out vec4 fragCornerRadii;
smooth out vec4 fragShapeThicknessMask;
out float fragDepth;
out float fragShapeThickness;
out float fragShapeSoftness;
out float fragDashLength;
out float fragGapLength;
out float fragDashCornerRad;

void main()
{
	vec2 positions[4] = vec2[](p0, p1, p2, p3);
	vec2 uvs[4] = vec2[](vec2(vertMinUV.x, vertMaxUV.y), vertMaxUV, vertMinUV, vec2(vertMaxUV.x, vertMinUV.y));

	vec2 vertPos = positions[gl_VertexID];
	vec2 vertUV = uvs[gl_VertexID];

	gl_Position = proj * vec4(vertPos, 0.0, 1.0);

	fragShapePos = p0;
	fragShapeDim = vec2(abs(p3.x - p0.x), abs(p3.y - p0.y));
	fragUV = vertUV;
	fragColor = vertColor;
	fragGradient = vertGradient;
	fragCornerRadii = vertCornerRadii;
	fragShapeThicknessMask = vertShapeThicknessMask;
	fragDepth = vertDepth;
	fragShapeThickness = vertShapeThickness;
	fragShapeSoftness = vertShapeSoftness;
	fragDashLength = vertDashLength;
	fragGapLength = vertGapLength;
	fragDashCornerRad = vertDashCornerRad;
}
