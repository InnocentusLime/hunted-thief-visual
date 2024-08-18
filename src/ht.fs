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
uniform sampler2D noise;
uniform float time;
uniform float lightPo;

float samplePeriodicNoise(vec2 uv) {
    return texture2D(noise, mod(uv, 1.0)).r;
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

vec3 sampleLightTex(vec2 uv, bool shif) {
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
    return wideK * texture2D(lightTex, uv).xyz + innK * texture2D(lightTex, uv2).xyz;
}

vec3 spotLight(float a, float b, float ac, vec3 spotPos) {
    vec3 v = vec3(0.0, -1.0, 0.0);
    vec3 p = fragPos - spotPos;

    vec3 lightOff = spotPos - fragPos;
    vec3 NfragNormal = normalize(fragNormal);
    float lightDist = length(lightOff);

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
        // return texture2D(lightTex, vec2(x, y)).xyz * intense;
        return sampleLightTex(vec2(x, y), ac > 0.3) * intense;
    }

    float res = 0.0;
    return vec3(res);
}

vec3 darken(vec3 tint, vec3 target) {
    vec3 inv = vec3(1.0) - target;
    return clamp(tint / inv, vec3(0.0), vec3(1.0));
}

vec3 blend(vec3 a, vec3 b, float k) {
    return k * a + (1 - k) * b;
}

vec3 erase(vec3 a, vec3 b) {
    return a * (1 - length(b)) + vec3(1.0) * length(b);
}

void main() {
    vec3 light = vec3(255.0 / 255.0, 95.0 / 255.0, 0.0);
    vec3 dark = vec3(7.0 / 255.0, 12.0 / 255.0, 17 / 255.0);

    float shadowMix = 0.5;
    vec3 darkLightPos = vec3(0.0, 1.2, 1.5);
    vec2 darkLightDelta = (lightPos.xz - darkLightPos.xz) * 0.3;

    float k = pow(lightPo, 3.0);
    vec3 darkLight = spotLight(0.1, 0.1, k * 0.7 + 0.3, darkLightPos + vec3(darkLightDelta.x, 0.0, darkLightDelta.y));
    vec3 lantern = spotLight(0.1, 0.01, k * 0.58 + 0.42, vec3(lightPos.x, 0.3, lightPos.z - 0.05));

    vec4 final = vec4(darkLight, 1.0) * texture2D(Texture, uv);
    vec3 biglit = blend(darken(dark, final.rgb), final.rgb, min(0.65, 1 - length(lantern)));
    // TODO: improve the blending stuff
    biglit = blend(light * lantern * biglit.rgb, biglit.rgb, min(0.33, length(lantern * lantern)));
    gl_FragColor = vec4(biglit, final.w);
}