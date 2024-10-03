# Ouch Browser

Focus on your browsing.

## Features

- Arc Browser-like workflow (UX, keyboard shortcuts, etc.)
- Intergration with the GNOME platform
- Privacy respecting (we do not collect **any** data)
- Mobile support
- Extremely lightweight (the binary is just a little under a megabyte large!)

## Installing

As Ouch Browser is beta software, it's installation options are limited.

### Binary

> **Warning**
>
> Ouch Browser is only available on x84_64 architectures. You may need to [build Ouch Browser](#building) for your architecture before continuing.

1. Download [the latest release](https://git.sr.ht/~shrimple/ouch/refs/download/0.4.1/ouch-0.4.1.tar.gz)

2. Extract the file. This will result in a new `ouch-0.4.1` folder in where you downloaded the file, depending on your method of extraction.

3. Open the directory, then execute the `install.sh` file.

Wait a few seconds, and Ouch Browser should appear in your launcher. If it doesn't by a minute, you may need to log out, and log back in.

### Flatpak

1. Ensure you have Shrimple Technologies' Flatpak repository installed

```sh
flatpak remote-add --if-not-exists shrimple https://shrimple.srht.site/repo/flatpak/shrimple.flatpakrepo
```

2. Install Ouch Browser

```sh
flatpak install shrimple site.srht.shrimple.ouch
```

## Building

### Binary

To build Ouch Browser, you need [GTK 4](https://gitlab.gnome.org/GNOME/gtk), [libadwaita](https://gitlab.gnome.org/GNOME/libadwaita), [WebKitGTK](https://webkitgtk.org/), [Blueprint](https://gitlab.gnome.org/jwestman/blueprint-compiler), and [just](https://github.com/casey/just) installed on your system. You **might** be able to build Ouch Browser without `just`, by using the `Justfile` as a `Makefile` by replacing the `just` command in these instructions with `make -f=Justfile`.

If you are on Fedora, you can install the GNOME Software Development group. Check your Linux distribution's package manager for a similar meta-package.

```sh
sudo dnf group install gnome-software-development
```

Then, you can build Ouch Browser.

```sh
just build-schemas
just build
```

### Flatpak

#### GNOME Builder

Building Ouch Browser as a Flatpak with GNOME Builder is recommended due to it's simplicity

0. Open GNOME Builder, then click on "Clone Repository..." then enter `https://git.sr.ht/~shrimple/ouch` into the "Repository URL" field.

1. Click on the little hammer icon on the title bar. This will start the build.

2. After the build has finished, click on the dropdown menu beside the hammer icon, and click on "Export," and a GNOME Files window will open with the exported file selected.

#### Flatpak Builder

Building Ouch Browser as a Flatpak with Flatpak Builder may be a better way than using GNOME Builder, due to being run as a single command, and is editor-agonistic. Before building, you must remove GPG signing in the `Justfile` located in the source folder:

```diff
build-flatpak:
    @flatpak-builder \
        --force-clean \
        --user \
-       --gpg-sign=22C359EDF1E87959D2DAD548E4BE7E015E072434 \
        --repo=.build/repo \
        .build \
        build-aux/flatpak/{{ ID }}.json
    @flatpak build-bundle \
        .build/repo \
        {{ ID }}.flatpak \
        {{ ID }} \
        --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo
```

Then, you can build the Flatpak:

```sh
just build-flatpak
```

The exported Flatpak file will be in the Ouch Browser source folder.

## Development

The maintainers of Ouch Browser use [Visual Studio Code](https://code.visualstudio.com/) to develop Ouch Browser, but however, [GNOME Builder](https://apps.gnome.org/Builder/) is recommended, due to being easy to use, and it's simplicity. Despite whichever one you choose, we provide configurations for both of the editors (`.vscode/settings.json` and `.editorconfig`). If you plan on contributing, we do not recommend changing any of these settings.



For Visual Studio Code users, we also provide a file (`.vscode/extensions.json`) containing the needed extensions to develop Ouch Browser. We also include tasks, for example, you can press <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>B</kbd> to build Ouch Browser.

## Contributing Translations

Translations are more than welcome, please go to our [Weblate project](https://hosted.weblate.org/projects/ouch/ouch/) to get started.

## Frequently Asked Questions

<!-- I probably need to reword this for accessiability. -->

Please see [this file](FAQ.md).

## Credits

- [Jakub Steiner](http://jimmac.eu/) for designing the app icon.
- [Lo](https://github.com/lo2dev) for [the tabs code](https://github.com/lo2dev/zenith/blob/9758de563b2317c05a774317be02ef60cdd4b8e3/src/window.blp#L126-L149) and [the URL matching code](https://github.com/lo2dev/zenith/blob/530dc0fc69620d46fe78fba80919644bd99c722e/src/window.py#L75-L94) from Zenith.
- All of the translation contributors.
