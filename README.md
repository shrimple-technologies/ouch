# Ouch Browser

Focus on your browsing.

## Installing

As Ouch Browser is beta software, it's installation options are limited.

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
just
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

Building Ouch Browser as a Flatpak with Flatpak Builder may be a better way than using GNOME Builder, due to being run as a single command, but may use more of your RAM.

```sh
just build-flatpak
```

The exported Flatpak file will be in the Ouch Browser source folder.
