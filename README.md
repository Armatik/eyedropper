[![Part of GNOME Circle](https://circle.gnome.org/assets/button/badge.svg)](https://apps.gnome.org/Eyedropper/)
![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

![Eyedropper](data/icons/com.github.finefindus.eyedropper.svg)

# Eyedropper

Pick and format colors.

![Main Page](data/resources/screenshots/main_default.png)

<details>
  <summary>More screenshots</summary>

![Status page](data/resources/screenshots/status_page.png)

![Customize the shown formats](data/resources/screenshots/main_customize.png)

![Edit Colors](data/resources/screenshots/edit_sheet.png)


</details>


## Features

- Pick a Color
- Enter a color in Hex-Format
- Parse RGB/RGBA/ARGB Hex-Colors
- View colors in formats
- Customize which formats appear as well as their order
- Generate a palette of different shades

### Available formats

- Name (includes W3C color sets, GNOME Palette and xkcd color survey)
- Hex
- RGB
- HSL
- HSV
- CMYK
- XYZ
- CIELAB
- HWB
- CIELCh/HCL
- LMS
- Hunter Lab

### When should I use this? And when not?

This application is geared towards advanced users (developers, designers, etc…), who not only need to pick a color but also modify or view it in different formats. For simply picking a color on Gnome, the [color-picker extension](https://github.com/tuberry/color-picker) is far better suited.

## Installation

### Flatpak
<a href='https://flathub.org/apps/details/com.github.finefindus.eyedropper'><img width='240' alt='Get it on Flathub' src='https://flathub.org/api/badge?locale=en&light'/></a>

#### Nightly Flatpak
> :warning: The nightly flatpak may contain bugs or unexpected behaviour.

Download the latest artifact from the [CI](https://github.com/FineFindus/eyedropper/actions/workflows/ci.yml).

## Contributing

Contributors are expected to follow the [GNOME Code of Conduct](https://conduct.gnome.org/).

Any contributions you make are greatly appreciated. For major changes, please open an issue first to discuss what you would like to change.

To contribute:

1. [Fork the Project](https://github.com/FineFindus/eyedropper/fork)
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'feat: add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Translation

Translations are a great way to contribute. This project uses the [GNU gettext](https://www.gnu.org/software/gettext/manual/html_node/index.html#SEC_Contents) for translations. If you want to learn more, visit the [translator section](https://www.gnu.org/software/gettext/manual/html_node/Translators.html#Translators).

You can either contribute translations using [Weblate](https://hosted.weblate.org/projects/eyedropper/eyedropper/),
or manually using programs like [Poedit](https://poedit.net) or [Gtranslator](https://gitlab.gnome.org/GNOME/gtranslator/).

After finishing the translations, add the translated language code into the [LINGUAS](po/LINGUAS) file. Then follow the above steps to create a pull request. Please also state in the description if you are willing to maintain the translation.

## Building

See this [general guide](https://wiki.gnome.org/Newcomers/BuildProject) for building the project using GNOME Builder.

Alternatively use this [VS Code Extension](https://marketplace.visualstudio.com/items?itemName=bilelmoussaoui.flatpak-vscode#:~:text=VSCode%20%2B%20Flatpak%20Integration,run%2C%20and%20export%20a%20bundle) for working inside VS Code.

## Credits

A huge thanks to these projects who served either as an inspiration or as code examples on how to use gtk-rs.

- [GTK Rust Template](https://gitlab.gnome.org/World/Rust/gtk-rust-template)
- [Contrast](https://gitlab.gnome.org/World/design/contrast)
- [Microsoft Color Picker Utility](https://docs.microsoft.com/en-us/windows/powertoys/color-picker) - Inspirations on the design
- All the other FOSS-GTK apps
