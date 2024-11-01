Name: ouch
Version: 0.5.0
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
Requires: libadwaita
Requires: gtk4
Requires: webkitgtk6.0

%description
Ouch Browser isn't just another web browser, it's a whole new look on the web,
with a modern look and feel, and a emphasis on productivity.

%prep
%autosetup

%build
blueprint-compiler batch-compile src/ui src/ui src/ui/window.blp src/ui/about.blp src/ui/about-shrimple.blp src/ui/help-overlay.blp src/ui/preferences.blp src/ui/oobe.blp src/ui/plugin-manager.blp
cargo build --release

%install
mkdir -p %{buildroot}%{_datadir}/locale/fr/LC_MESSAGES
mkdir -p %{buildroot}%{_datadir}/locale/pt_BR/LC_MESSAGES
mkdir -p %{buildroot}%{_datadir}/locale/nb_NO/LC_MESSAGES
mkdir -p %{buildroot}%{_datadir}/locale/eo/LC_MESSAGES
msgfmt -o %{buildroot}%{_datadir}/locale/fr/LC_MESSAGES/ouch.mo po/fr.po
msgfmt -o %{buildroot}%{_datadir}/locale/pt_BR/LC_MESSAGES/ouch.mo po/pt_BR.po
msgfmt -o %{buildroot}%{_datadir}/locale/nb_NO/LC_MESSAGES/ouch.mo po/nb_NO.po
msgfmt -o %{buildroot}%{_datadir}/locale/eo/LC_MESSAGES/ouch.mo po/eo.po

install -Dm 644 res/site.srht.shrimple.ouch.gschema.xml --target-directory %{buildroot}%{_datadir}/glib-2.0/schemas
install -Dm 644 res/site.srht.shrimple.ouch.desktop --target-directory %{buildroot}%{_datadir}/applications
install -Dm 644 res/site.srht.shrimple.ouch.svg --target-directory %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
install -Dm 644 res/site.srht.shrimple.ouch-symbolic.svg --target-directory %{buildroot}%{_datadir}/icons/hicolor/symbolic/apps
install -Dm 644 res/site.srht.shrimple.svg --target-directory %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
install -Dm755 target/release/ouch --target-directory %{buildroot}%{_bindir}


%post
glib-compile-schemas %{buildroot}%{_datadir}/share/glib-2.0/schemas

%files
%license licenses/GPL-3.0-or-later.txt
%{_bindir}/ouch
%{_datadir}/applications/site.srht.shrimple.ouch.desktop
%{_datadir}/icons/hicolor/scalable/apps/site.srht.shrimple.ouch.svg
%{_datadir}/icons/hicolor/scalable/apps/site.srht.shrimple.svg
%{_datadir}/icons/hicolor/symbolic/apps/site.srht.shrimple.ouch-symbolic.svg
%{_datadir}/locale/fr/LC_MESSAGES/ouch.mo
%{_datadir}/locale/pt_BR/LC_MESSAGES/ouch.mo
%{_datadir}/locale/nb_NO/LC_MESSAGES/ouch.mo
%{_datadir}/locale/eo/LC_MESSAGES/ouch.mo
%{_datadir}/glib-2.0/schemas/site.srht.shrimple.ouch.gschema.xml


%changelog
* Thu Oct 31 2024 Max Walters <mdwalters.pm@proton.me>
- Adds OOBE
- Adds support for plugins via Lua
- Adds support for more search engines and homepages!
- Fixes multiple bugs
- Now uses libadwaita 1.6, GTK 4.16, and WebKitGTK 2.46
