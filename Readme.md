# S3LightFixes

S3LightFixes is a descendant of [Waza-lightfixes](https://modding-openmw.com/mods/waza_lightfixes/), which itself is a descendant of [Lightfixes.pl](https://modding-openmw.com/tips/custom-shaders/#lightfixes-plugin) by vtastek. All three applications are designed to make ESP files which adjust the lighting values from *all* mods listed in one's openmw.cfg. 

In other words, make light gud. What sets this version apart is that it's a standalone binary application, instead of piggybacking off tes3cmd. The changes it makes are the same as the previous two, but with additional conveniences like automatic installation and support for portable installs of OpenMW, and itself having greater portability.

# Usage

Just download the executable for your OS and run it however's most convenient:

[Windows](https://github.com/magicaldave/motherJungle/releases/latest/files/windows-latest.zip)

[Mac](https://github.com/magicaldave/motherJungle/releases/latest/files/macos-latest.zip)

[Linux](https://github.com/magicaldave/motherJungle/releases/latest/files/ubuntu-latest.zip)

You may optionally edit the lightconfig.toml S3Lightfixes creates (next to your user openmw.cfg) to adjust its settings for your next run. Or, make your own lightconfig.toml and place it next to the S3LightFixes executable before running it. The toml schema is as follows:

```toml
# Automatically enable S3LightFixes.omwaddon
auto_install = true
# Disable flickering lights
disable_flickering = true
# Serialize S3LightFixes plugin to a text file. Don't do this unless you're asked to (or just curious)
save_log = false
# Hue multiplier for non-colored lights
standard_hue = 0.6000000238418579
# Saturation multiplier for non-colored lights
standard_saturation = 0.800000011920929
# Value multiplier for non-colored lights
standard_value = 0.5699999928474426
# Radius multiplier for non-colored lights
standard_radius = 2.0
# Hue multiplier for colored lights
colored_hue = 1.0
# Saturation multiplier for colored lights
colored_saturation = 0.8999999761581421
# Value multiplier for colored lights
colored_value = 0.699999988079071
# Radius multiplier for colored lights
colored_radius = 1.100000023841858
```


# Nitty-Gritty
More specifically, the lightfixes plugin adjusts the color and radius of colored or whitish lights for your config separately. The radius in lightConfig.toml is used as a multiplier on top of the existing radius of the light, so they'll generally be brighter with the default configuration.

S3LightFixes also supports portable installations of OpenMW by way of utilizing the `OPENMW_CONFIG` and `OPENMW_CONFIG_DIR` environment variables. Simply run it like so:
- Powershell:
  ```
  OPENMW_CONFIG="C:\Documents\My Games\OpenMW\openmw.cfg" .\s3lightfixes.exe
  OPENMW_CONFIG_DIR="C:\Documents\My Games\OpenMW\" .\s3lightfixes.exe
  ```
- Linux:
  ```
  OPENMW_CONFIG="$HOME/.config/openmw/openmw.cfg" ./s3lightfixes
  OPENMW_CONFIG_DIR="$HOME/.config/openmw/" ./s3lightfixes
  ```
- macOS:
  ```
  OPENMW_CONFIG="$HOME/Library/Application Support/openmw/openmw.cfg" ./s3lightfixes
  OPENMW_CONFIG_DIR="$HOME/Library/Application Support/openmw/" ./s3lightfixes
  ```

Additionally, S3LightFixes will perform the following:
- Automatically install itself into your `data-local` directory of openmw
- Create a config file adjacent to your openmw.cfg if one doesn't already exist
- Disable sunlight color in interiors
- Optionally remove the Flicker and FlickerSlow flags from all lights
- Nullify all negative lights

Finally, S3LightFixes supports using multiple layered configuration files. It will search for and use a `lightConfig.toml` in the following folders, in order:
1. The same folder as itself
2. The same folder as your openmw.cfg
3. The `data-local` directory of your OpenMW installation
