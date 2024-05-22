%global _cross_first_party 1
%undefine _debugsource_packages

Name: %{_cross_os}settings-aws-dev
Version: 0.0
Release: 0%{?dist}
Summary: settings-aws-dev
License: Apache-2.0 OR MIT
URL: https://github.com/bottlerocket-os/bottlerocket

BuildRequires: %{_cross_os}glibc-devel

%description
%{summary}.

%prep
%setup -T -c
%cargo_prep

%build
%cargo_build --manifest-path %{_builddir}/sources/Cargo.toml \
    -p settings-plugin-aws-dev

%install
install -d %{buildroot}%{_cross_libdir}
install -p -m 0755 \
    ${HOME}/.cache/%{__cargo_target}/release/libsettings.so \
    %{buildroot}%{_cross_libdir}

%files
%{_cross_libdir}/libsettings.so
