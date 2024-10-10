Name: ouch
Version: 0.5.0-rc.1
Release: 1%{?dist}
Summary: Focus on your browsing
License: GPL-3.0-or-later
URL: https://git.sr.ht/~shrimple/ouch
Source0: %{URL}/archive/%{version}.tar.gz


BuildRequires: rust
BuildRequires: cargo
BuildRequires: libadwaita-devel >= 1.5.0
BuildRequires: gtk4-devel >= 4.15.0
BuildRequires: webkitgtk6.0-devel >= 2.46.0
BuildRequires: lua-devel >= 5.4.0

Requires: libadwaita >= 1.5.0
Requires: gtk4 >= 4.15.0
Requires: webkitgtk6.0 >= 2.46.0
# I have no idea if this is actually required at runtime.
Requires: lua >= 5.4.0 


%description

Ouch Browser isn't just another web browser, it's a whole new look on the web,
with a modern look and feel, and a emphasis on productivity.


%prep
%autosetup


%build
cargo build --release --verbose


%install
install -Dm755 target/release/ouch --target-directory %{buildroot}%{_bindir}


%files
%license licenses/GPL-3.0-or-later.txt
%{_bindir}/ouch


%changelog
