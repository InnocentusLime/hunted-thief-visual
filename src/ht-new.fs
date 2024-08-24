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
uniform sampler2D roughNoise;
uniform sampler2D noise;
uniform float time;
uniform float lightPo;

float samplePeriodicNoise(vec2 uv) {
    return texture2D(noise, mod(uv, 1.0)).r;
}

float samplePeriodicNoise3(vec2 uv) {
    return texture2D(roughNoise, mod(uv, 1.0)).r;
}

float samplePeriodicNoise2(vec2 uv) {
    return texture2D(lightTex, mod(uv, 1.0)).r;
}

float candleNoiseChan(float x, float y) {
    float lfha = 2.0 * (samplePeriodicNoise(vec2(x * 0.67, y)) - 0.5) * 1.1;
    float hfla = 2.0 * (samplePeriodicNoise(vec2(x * 4.0, y)) - 0.5) * 0.3;
    float extra = 2.0 * (samplePeriodicNoise(vec2(x * 4.8, y)) - 0.5) * 0.1;

    return lfha + hfla + extra;
}

vec4 candleWiggleNoise(float x) {
    float n1 = candleNoiseChan(x, 0.2);
    float n2 = candleNoiseChan(x + 0.2, 0.3);
    float n3 = candleNoiseChan(x - 0.1, 0.4);
    float n4 = candleNoiseChan(x + 0.8, 0.5);

    return vec4(n1, n2, n3, n4) * vec4(0.06);
}

float sampleLightTex(vec2 uv, bool shif) {
    float innK = 0.4;
    float wideK = 0.6;
    float brightSc = 0.55;

    vec2 uv2 = (uv - vec2(0.5)) / brightSc + vec2(0.5);
    uv2 = uv2.yx;

    vec4 cn = candleWiggleNoise(time);
    vec2 uvShear = cn.xy;
    vec2 uvShear2 = cn.zw;
    if (shif) {
        innK += (samplePeriodicNoise(vec2(time * 2.0, 0.45)) - 0.5) * 0.22;
        wideK += (samplePeriodicNoise(vec2(time * 2.0, 0.65)) - 0.5) * 0.08;
    }

    uv = uv + uvShear * (uv.yx - vec2(0.5));
    uv2 = uv2 + uvShear2 * (uv2.yx - vec2(0.5));
    return innK + wideK;
    // return 1.0;
    // return wideK * texture2D(lightTex, uv).xyz + innK * texture2D(lightTex, uv2).xyz;
}

mat2 rotate(float a) {
    return mat2(cos(a), -sin(a),
                sin(a), cos(a));
}

float spotLight(float a, float b, float ac, vec3 spotPos) {
    vec3 v = vec3(0.0, -1.0, 0.0);
    vec3 p = fragPos - spotPos;

    vec3 lightOff = spotPos - fragPos;
    vec3 NfragNormal = normalize(fragNormal);
    float lightDist = length(lightOff);

    // vectors
    vec3 h = (dot(p, v) / dot(v, v)) * v;
    vec3 r = p - h;

    // compute
    float intense = 0.0;

    /* The noise nudge */
    float rk = samplePeriodicNoise(vec2(time / 4.0, time / 4.0));
    float rotK = samplePeriodicNoise3(vec2(time / 5.9, time / 5.9));
    vec2 noiseVect = rotate((rk * 0.6 + rotK * 0.4) * 2 * 3.14) * normalize(r.xz) * (rk * 0.4);
    float dac = (samplePeriodicNoise(noiseVect) - 0.5) * 0.05;

    /* "drawn" nudge */
    float bc = sqrt(1 - (ac + dac)*(ac + dac));
    float fb = (bc / (ac + dac)) * length(h);
    float k = length(r) / fb;

    float nudgeV = ((1 - k) + 0.43) / 1.1;
    float r_ang = atan(normalize(r).z, normalize(r).x);
    float nudgeU = mod((r_ang / 3.14 + 1.0) / 2.0 * 2.0, 1.0);
    vec2 lVec = vec2(nudgeU, nudgeV);
    float dac2 = texture2D(lightTex, lVec).r;
    // dac2 = 0.0;

    vec3 np = normalize(p);
    if (dot(np, v) > ac + dac + dac2) {
        float Diff = clamp(dot(NfragNormal, -p), 0.0, 1.0);

        intense = 1.0 / (1.0 + a * lightDist + b * lightDist * lightDist);
        intense *= pow(smoothstep(ac + dac + dac2, 1.0, dot(np, v)), 0.3);

        // return sampleLightTex(vec2(x, y), ac > 0.3) * intense;
        return intense;
    }

    return 0.0;
}

vec3 darken(vec3 tint, vec3 target) {
    vec3 inv = vec3(1.0) - target;
    return clamp(tint / inv, vec3(0.0), vec3(1.0));
}

vec3 blend(vec3 a, vec3 b, float k) {
    return k * a + (1 - k) * b;
}

vec3 erase(vec3 a, float b) {
    return a * (1 - length(b)) + vec3(1.0) * length(b);
}

void main() {
    vec3 light = vec3(255.0 / 255.0, 95.0 / 255.0, 0.0);
    vec3 dark = vec3(7.0 / 255.0, 12.0 / 255.0, 17 / 255.0);

    float shadowMix = 0.5;
    vec3 darkLightPos = vec3(0.0, 1.2, 1.5);
    vec2 darkLightDelta = (lightPos.xz - darkLightPos.xz) * 0.3;

    float k = pow(lightPo, 3.0);
    float darkLight = spotLight(0.1, 0.1, k * 0.7 + 0.3, darkLightPos + vec3(darkLightDelta.x, 0.0, darkLightDelta.y));
    float lantern = spotLight(0.1, 0.1, k * 0.58 + 0.42, vec3(lightPos.x, 0.3, lightPos.z - 0.05));

    vec4 tex = texture2D(Texture, uv);
    vec3 col = tex.rgb;
    vec3 final = blend(darken(dark, col * darkLight), col * darkLight, 0.65) * (1.0 - lantern) +
                 blend(light * lantern * col, col * lantern, 0.33) * float(lantern > 0.0);
    // vec4 final = vec4(vec3(darkLight), 1.0) * ;
    // vec3 biglit = blend(darken(dark, final.rgb), final.rgb, float(lantern == 0.0));
    // // TODO: improve the blending stuff
    // biglit = blend(light * lantern * biglit.rgb, biglit.rgb, float(lantern > 0));
    gl_FragColor = vec4(final, tex.w);
}