---
title: Plugins
---

Plugins are Ouch Browser's replacement for WebExtensions (e.g. extensions
installed via Firefox Add-ons or Chrome Web Store). Compared to WebExtensions,
Ouch Browser plugins are written in Lua, while WebExtensions are written in
JavaScript; Ouch Browser plugins have more control over the browser
window itself, while WebExtensions can only control the web view, etc. While
there are mant pros about Ouch Browser extensions, it also comes with it's fair
share of cons too, for example, because of being able to control the browser
itself, malicious extensions could be used to steal the user's data, such as
passwords, and other senstitive material. This could be circumvented by
introducing the (currently non-existant) plugin store from where plugins can
only be installed from there, unless if it is being used for development, in
which case, Ouch Browser would warn the user before installing the plugin.



As previously mentioned, Ouch Browser plugins are written in Lua, because of
the programming language being very extensible, and many parsers exist for it.
The simplest form of a plugin is this:

```lua
ouch.config(
    {
        "id" = "hello_world",
        "name" = "\"Hello, world!\" Plugin Demo"
    }
)

print("Hello, world!")
```

This plugin runs the `ouch.config()` function, which sets up the metadata of the
plugin, and is therefore required to have in your plugin, then runs the
built-in `print()` function, and outputs "Hello, world!" This is a good start
for the plugin, however, unless if the user invokes Ouch Browser directly
(using the `ouch` command or `flatpak run site.srht.shrimple.ouch`), the user
will not be able to see the message. However, a function is availiable that
displays the message to the user, no matter how they invoked Ouch Browser:

```diff
ouch.config(
    {
        "id" = "hello_world",
        "name" = "\"Hello, world!\" Plugin Demo"
    }
)

-print("Hello, world!")
+ouch.dialog(
+   "\"Hello, world!\" Plugin Demo",
+   "Hello, world!"
+)
```

Now, the "Hello, world!" message is displayed to the user as a dialog. As this
dialog resembles the dialogs used for JavaScript `alert()` functions, we
prohibit using messages, such as, "example.org says" as the title, to ease confusion.
