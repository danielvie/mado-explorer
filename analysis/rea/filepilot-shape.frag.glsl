#version 330 core

uniform sampler2DArray textureSampler;
uniform bool textureIsMono;

in vec2 fragShapePos; // Pixel space location of the shape
in vec2 fragShapeDim; // Pixel space scale of the shape
smooth in vec2 fragUV;
smooth in vec4 fragColor;
smooth in vec4 fragGradient;
smooth in vec4 fragCornerRadii; // Radius of the shape corners
smooth in vec4 fragShapeThicknessMask; // Exclude borders
in float fragDepth; // Texture array index
in float fragShapeThickness; // Shape outline
in float fragShapeSoftness; // Shape edge softness
in float fragDashLength;
in float fragGapLength;
in float fragDashCornerRad;

out vec4 outColor;

float RoundedBoxSDF(vec2 centerPos, vec2 dim, vec4 radii)
{
	radii.xy = (centerPos.x > 0.0) ? radii.xy : radii.zw;
	radii.x  = (centerPos.y > 0.0) ? radii.x  : radii.y;
	vec2 q = abs(centerPos) - dim + radii.x;
	return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - radii.x;
}

bool IsDashedOutline(vec2 fragCoord, vec2 shapePos, vec2 shapeDim, float thickness, vec4 radii)
{
	float totalLength = fragDashLength + fragGapLength;
	float dist = 0.0;

	if (fragCoord.x < shapePos.x + thickness || fragCoord.x > shapePos.x + shapeDim.x - thickness)
	{
		dist = mod(fragCoord.y - shapePos.y, totalLength);
	}
	else if (fragCoord.y < shapePos.y + thickness || fragCoord.y > shapePos.y + shapeDim.y - thickness)
	{
		dist = mod(fragCoord.x - shapePos.x, totalLength);
	}

	vec2 centerPos = fragCoord - shapePos - shapeDim / 2.0;
	float distance = RoundedBoxSDF(centerPos, shapeDim / 2.0, radii);

	return dist < fragDashLength || distance > 0.0;
}

void main()
{
	vec4 sampled = texture(textureSampler, vec3(fragUV, fragDepth));
	vec4 tintColor = fragColor;

	vec4 mono = vec4(sampled.r);
	vec4 pixel = tintColor * mix(sampled, mono, vec4(textureIsMono));

	if (fragDashLength > 0)
	{
		vec4 dashCornerRadii = vec4(fragDashCornerRad);
		bool isDashed = IsDashedOutline(gl_FragCoord.xy, fragShapePos, fragShapeDim, fragShapeThickness, dashCornerRadii);
		if (!isDashed) discard;
	}

	// We don't support rotation with extra params
	float cornerRad = max(fragCornerRadii.x, max(fragCornerRadii.y, max(fragCornerRadii.z, fragCornerRadii.w)));
	if (cornerRad > 0 || fragShapeThickness > 0 || fragShapeSoftness > 0)
	{
		// Determine edge conditions
		float leftEdge   = step(gl_FragCoord.x, fragShapePos.x + fragShapeThickness);
		float rightEdge  = step(fragShapePos.x + fragShapeDim.x - fragShapeThickness, gl_FragCoord.x);
		float topEdge    = step(fragShapePos.y + fragShapeDim.y - fragShapeThickness, gl_FragCoord.y);
		float bottomEdge = step(gl_FragCoord.y, fragShapePos.y + fragShapeThickness);

		// Mask to skip edges
		float skipLeftEdge   = step(0.5, fragShapeThicknessMask.x);
		float skipRightEdge  = step(0.5, fragShapeThicknessMask.y);
		float skipTopEdge    = step(0.5, fragShapeThicknessMask.z);
		float skipBottomEdge = step(0.5, fragShapeThicknessMask.w);

		// Combine conditions to determine if the current fragment should be skipped
		float skip = max(max(leftEdge * skipLeftEdge, rightEdge * skipRightEdge),
										 max(topEdge * skipTopEdge, bottomEdge * skipBottomEdge));

		// Special cases for corners
		float cornerLL = leftEdge * bottomEdge * abs(skipLeftEdge - skipBottomEdge);
		float cornerLT = leftEdge * topEdge * abs(skipLeftEdge - skipTopEdge);
		float cornerRT = rightEdge * topEdge * abs(skipRightEdge - skipTopEdge);
		float cornerRL = rightEdge * bottomEdge * abs(skipRightEdge - skipBottomEdge);

		// Adjust skipping based on corner conditions
		float skipAdjustment = max(max(cornerLL, cornerLT), max(cornerRT, cornerRL));
		skip = max(0.0, skip - skipAdjustment);

		float smoothedAlpha = 0.0f;

		if (skip < 0.5)
		{
			float softness = max(fragShapeSoftness, float(cornerRad > 0 && fragShapeThickness > 0) * fragShapeThickness);
			vec2 halfDim = (fragShapeDim / 2.0f);
			vec2 centerPos = fragShapePos - gl_FragCoord.xy + halfDim;

			if (gl_FragCoord.x < (fragShapePos.x + halfDim.x)) centerPos.x = ceil(centerPos.x);
			else                                               centerPos.x = floor(centerPos.x);
			if (gl_FragCoord.y < (fragShapePos.y + halfDim.y)) centerPos.y = ceil(centerPos.y);
			else                                               centerPos.y = floor(centerPos.y);

			vec2 softPad = vec2(fragShapeSoftness * 4.0f); // Shadows that use big value for softness
			vec4 radii = max(fragCornerRadii, vec4(fragShapeThickness));
			float distance = RoundedBoxSDF(centerPos, floor((fragShapeDim - softPad) / 2.0f), radii);

			smoothedAlpha = (fragShapeThickness > 0.0f) ?
				1.0f - smoothstep(-softness * 0.5f, softness * 0.5f, abs(distance) - (fragShapeThickness * 0.5f)) :
				1.0f - smoothstep(0.0f, max(1.0f, softness * 2.0f), distance);
		}

		pixel = pixel * smoothedAlpha;
	}

	float gradient = max(fragGradient.x, max(fragGradient.y, max(fragGradient.z, fragGradient.w)));
	if (gradient > 0)
	{
		// Bilinear interpolation of the gradient values based on pixel position
		vec2 pixelPos = (gl_FragCoord.xy - fragShapePos) / fragShapeDim;
		float topInterp = mix(fragGradient.x, fragGradient.y, pixelPos.x);  // Top edge (left-right interpolation)
		float bottomInterp = mix(fragGradient.w, fragGradient.z, pixelPos.x);  // Bottom edge (left-right interpolation)
		float finalGradientAlpha = mix(topInterp, bottomInterp, pixelPos.y);  // Interpolate between top and bottom edges
		pixel = pixel * finalGradientAlpha;
	}

	outColor = pixel;
}
