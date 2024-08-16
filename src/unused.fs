

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