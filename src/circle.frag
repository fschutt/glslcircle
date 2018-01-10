#version 130

void main()
{
    vec2 pos = mod(gl_FragCoord.xy, vec2(50.0)) - vec2(25.0);
    float dist_squared = dot(pos, pos);
    vec4 foreground_color = vec4(1.0, 1.0, 1.0, 1.0);
    vec4 background_color = vec4(0.0, 0.0, 0.0, 1.0);
	gl_FragColor = mix(foreground_color, background_color, smoothstep(380.25, 420.25, dist_squared));
}