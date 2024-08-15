#version 100
precision lowp float;
varying lowp vec4 color;
varying lowp vec2 uv;
varying lowp vec3 fragPos;
varying lowp vec3 fragNormal;
varying lowp vec3 fragUp;
varying lowp vec3 fragRight;

uniform lowp vec3 lightPos;
uniform sampler2D Texture;
uniform sampler2D lightTex;

vec3 pointLight(float a, float b) {
    // Light Infi
    vec3 lightOff = (lightPos - fragPos);
    vec3 lightDir = normalize(lightOff);
    float lightDist = length(lightOff);
    vec3 NfragNormal = normalize(fragNormal);

    // How did we hit the fragment
    float Diff = clamp(dot(NfragNormal, lightDir), 0.0, 1.0);

    // Light Intensity
    float Intense = 1.0 / (1.0 + a * lightDist + b * lightDist * lightDist);

    float res = 1.0;
    res *= Diff;
    res *= Intense;
    return vec3(res);
}

vec3 spotLight(float a, float b) {
    // cfg
    vec3 spotPos = vec3(0.0, 2.0, 1.5);
    spotPos = lightPos;
    spotPos.y = 1.0;
    spotPos.y = 1.2;


    vec3 v = vec3(0.0, -1.0, 0.0);
    vec3 p = fragPos - spotPos;

    vec3 lightOff = spotPos - fragPos;
    vec3 NfragNormal = normalize(fragNormal);
    float lightDist = length(lightOff);
    float ac = 0.3;
    // float ac = 0.6;

    // compute
    float intense = 0.0;
    if (dot(normalize(p), v) > ac) {
        float Diff = clamp(dot(NfragNormal, -p), 0.0, 1.0);

        intense = 1.0 / (1.0 + a * lightDist + b * lightDist * lightDist);

        vec3 h = (dot(p, v) / dot(v, v)) * v;
        vec3 r = p - h;

        float bc = sqrt(1 - ac*ac);
        float b = (bc / ac) * length(h);
        float k = length(r) / b;

        float x = dot(normalize(r), fragRight) * k / 2.0 + 0.5;
        float y = dot(normalize(r), fragUp) * k / 2.0 + 0.5;

        x = (normalize(r) * k).x / 2.0 + 0.5;
        y = 1.0 - ((normalize(r) * k).z / 2.0 + 0.5);
        // return vec3(y);
        // return vec3(x, y, 0.0);
        // return vec3(1.0, 1.0, 0.0);
        // return vec3(x, y, 0.0);
        return texture2D(lightTex, vec2(x, y)).xyz * intense;
    }

    float res = 0.0;
    return vec3(res);
}

vec3 global_shade(float a, float b) {
    vec3 projLight = lightPos;
    projLight.y = 0.0;

    vec3 lightOff = (projLight - fragPos);
    vec3 lightDir = normalize(lightOff);
    float lightDist = length(lightOff);
    vec3 NfragNormal = normalize(fragNormal);

    float Diff = dot(NfragNormal, lightDir);
    // float neg = float(Diff <= 0.1);
    // Diff = (1 - neg) * clamp(Diff, 0.0, 1.0) + neg;
    float Intense = 1.0 / (1.0 + a * lightDist + b * lightDist * lightDist);

    return vec3(Diff);
}

void main() {
    gl_FragColor = vec4(spotLight(0.1, 0.01), 1.0) * texture2D(Texture, uv);
    // float l = length(texture2D(lightTex, lightP).xyz);
    // vec2 ll = vec2(l, length(lightOff.y));
    // float lightDist = length(ll);
    // float dist = 1.0 - clamp(lightDist, 0.0, 1.0);
    // gl_FragColor = bigLight();
    // gl_FragColor = vec4(pointLight(0.5, 0.5), 1.0);
    // gl_FragColor = vec4(global_shade(0.5, 0.5), 1.0);
}