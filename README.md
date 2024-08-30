# Ouch Browser

Focus on your browsing.

## Installing

As Ouch Browser is beta software, it's installation options are limited.

### Binary

1. Download [the latest release](https://git.sr.ht/~shrimple/ouch/refs/download/0.3.3/ouch-0.3.3.tar.gz)

2. Extract the file. This will result in a new `ouch-x.y.z` folder in where you downloaded the file.

3. Open the directory, then run this command in the directory:

```sh
sudo install -Dm 755 ouch --target-directory /usr/local/bin
sudo install -Dm 644 res/site.srht.shrimple.ouch.desktop --target-directory /usr/share/applications
sudo install -Dm 644 res/site.srht.shrimple.ouch.svg --target-directory /usr/share/icons/hicolor/scalable/apps/
sudo install -Dm 644 res/site.srht.shrimple.ouch-symbolic.svg --target-directory /usr/share/icons/hicolor/symbolic/apps/
sudo install -Dm 644 res/site.srht.shrimple.svg --target-directory /usr/share/icons/hicolor/scalable/apps/
```

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

To build Ouch Browser, you need [GTK 4](https://gitlab.gnome.org/GNOME/gtk), [libadwaita](https://gitlab.gnome.org/GNOME/libadwaita), [WebKitGTK](https://webkitgtk.org/), [Blueprint](https://gitlab.gnome.org/jwestman/blueprint-compiler), and [just](https://github.com/casey/just) installed on your system.

```sh
just build
```

### Flatpak

<!--
Before doing any of these instructions, you must edit Ouch Browser's sources list in `build-aux/flatpak/site.srht.shrimple.Ouch.json`:

```diff
"sources": [
-    {
-        "type": "archive",
-        "sha256": "86e66dd7ea8b06f414bac29311b31860da2af164b4b1884ee83150fa5383525e",
-        "url": "https://git.sr.ht/~shrimple/ouch/archive/x.x.x.tar.gz"
-    }
+    "devel.json"
]
```
-->

#### GNOME Builder

Building Ouch Browser as a Flatpak with GNOME Builder is recommended due to it's simplicity

0. Open GNOME Builder, then click on "Clone Repository..." then enter `https://git.sr.ht/~shrimple/ouch` into the "Repository URL" field.

1. Click on the little hammer icon on the title bar. This will start the build.

2. After the build has finished, click on the dropdown menu beside the hammer icon, and click on "Export," and a GNOME Files window will open with the exported file selected.

#### Flatpak Builder

Building Ouch Browser as a Flatpak with Flatpak Builder may be a better way than using GNOME Builder, due to being run as a single command, and is editor-agonistic.

```sh
just build-flatpak
```

The exported Flatpak file will be in the Ouch Browser source folder.

## Development

The maintainers of Ouch Browser use [Visual Studio Code](https://code.visualstudio.com/) to develop Ouch Browser, but however, [GNOME Builder](https://apps.gnome.org/Builder/) is recommended, due to being easy to use, and it's simplicity. Despite whichever one you choose, we provide configurations for both of the editors (`.vscode/settings.json` and `.editorconfig`). If you plan on contributing, we do not recommend changing any of these settings.



For Visual Studio Code users, we also provide a file (`.vscode/extensions.json`) containing the needed extensions to develop Ouch Browser. We also include tasks, for example, you can press <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>B</kbd> to build Ouch Browser.

## Frequently Asked Questions

<!-- I probably need to reword this for accessiability. -->

Please see [this file](FAQ.md).
