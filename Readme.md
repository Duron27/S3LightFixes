# S3LightFixes

S3LightFixes is a descendant of [Waza-lightfixes](https://modding-openmw.com/mods/waza_lightfixes/), which itself is a descendant of [Lightfixes.pl](https://modding-openmw.com/tips/custom-shaders/#lightfixes-plugin) by vtastek. All three applications are designed to make ESP files which adjust the lighting values from *all* mods listed in one's openmw.cfg. 

In other words, make light gud. What sets this version apart is that it's a standalone binary application, instead of piggybacking off tes3cmd. It functions identically to the previous two by design, but should be more modern and portable. That's all.

More specifically, the lightfixes plugin adjusts the color and radius of colored or whitish lights for your config separately. The radius in lightConfig.toml is used as a multiplier on top of the existing radius of the light, so they'll generally be brighter with the default configuration.

Additionally, lightfixes will perform the following:
- Disable sunlight color in interiors
- Optionally remove the Flicker and FlickerSlow flags from all lights
- Nullify all negative lights
