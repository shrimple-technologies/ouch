Name: ouch
Version: 0.4.1
Release: 1%{?dist}
Summary: Focus on your browsing
License: GPL-3.0-or-later
URL: https://git.sr.ht/~shrimple/ouch
Source0: %{URL}/archive/%{version}.tar.gz


BuildRequires: rust
BuildRequires: cargo
BuildRequires: blueprint-compiler
BuildRequires: libadwaita-devel
BuildRequires: gtk4-devel
BuildRequires: webkitgtk6.0-devel
# BuildRequires: lua-devel

Requires: libadwaita
Requires: gtk4
Requires: webkitgtk6.0
# I have no idea if this is actually required at runtime.
# Requires: lua


%description

Ouch Browser isn't just another web browser, it's a whole new look on the web,
with a modern look and feel, and a emphasis on productivity.


%prep
%autosetup


%build
blueprint-compiler batch-compile src/ui src/ui src/ui/window.blp src/ui/about.blp src/ui/about-shrimple.blp src/ui/help-overlay.blp src/ui/preferences.blp
cargo build --release


%install
# It really doesn't allow installing translations
# mkdir -p %{_prefix}/share/locale/fr/LC_MESSAGES
# mkdir -p %{_prefix}/share/locale/pt_BR/LC_MESSAGES
# mkdir -p %{_prefix}/share/locale/nb_NO/LC_MESSAGES
# msgfmt -o %{_prefix}/share/locale/fr/LC_MESSAGES/ouch.mo po/fr.po
# msgfmt -o %{_prefix}/share/locale/pt_BR/LC_MESSAGES/ouch.mo po/pt_BR.po
# msgfmt -o %{_prefix}/share/locale/nb_NO/LC_MESSAGES/ouch.mo po/nb_NO.po

# install -Dm 644 res/site.srht.shrimple.ouch.gschema.xml --target-directory %{_prefix}/share/glib-2.0/schemas
install -Dm 644 res/site.srht.shrimple.ouch.desktop --target-directory %{_prefix}/share/applications
install -Dm 644 res/site.srht.shrimple.ouch.svg --target-directory %{_prefix}/share/icons/hicolor/scalable/apps
install -Dm 644 res/site.srht.shrimple.ouch-symbolic.svg --target-directory %{_prefix}/share/icons/hicolor/symbolic/apps
install -Dm 644 res/site.srht.shrimple.svg --target-directory %{_prefix}/share/icons/hicolor/scalable/apps
install -Dm755 target/release/ouch --target-directory %{buildroot}%{_bindir}


# %pre
# glib-compile-schemas %{_prefix}/share/glib-2.0/schemas


%files
%license licenses/GPL-3.0-or-later.txt
%{_bindir}/ouch
%{_prefix}/share/applications/site.srht.shrimple.ouch.desktop
%{_prefix}/share/icons/hicolor/scalable/apps/site.srht.shrimple.ouch.svg
%{_prefix}/share/icons/hicolor/scalable/apps/site.srht.shrimple.svg
%{_prefix}/share/icons/hicolor/symbolic/apps/site.srht.shrimple.ouch-symbolic.svg
%{_prefix}/share/locale/fr/LC_MESSAGES/ouch.mo
%{_prefix}/share/locale/pt_BR/LC_MESSAGES/ouch.mo
%{_prefix}/share/locale/nb_NO/LC_MESSAGES/ouch.mo
%{_prefix}/share/glib-2.0/schemas/site.srht.shrimple.ouch.gschema.xml


%changelog
* Sun Sep 15 2024 Max Walters <mdwalters.pm@proton.me>
- Remove Ouch Browser from being a handler for XPI files and exported mail archives
