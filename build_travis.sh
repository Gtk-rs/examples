#!/bin/sh

set -x
set -e

if [ "$GTK" = latest -o "$GTK" = "3.22.30" ]; then
	BUNDLE="gtk-3.22.30-1"
	if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
		FEATURES=gtk_3_22_30,futures-stable
	else
		FEATURES=gtk_3_22_30,futures-stable
	fi
elif [ "$GTK" = "3.18" ]; then
	BUNDLE="gtk-3.18.1-2"
	if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
		FEATURES=gtk_3_18,futures-stable
	else
		FEATURES=gtk_3_18,futures-stable
	fi
fi

if [ -n "$BUNDLE" ]; then
	WD="$PWD"
	cd "$HOME"
	curl -LO "https://github.com/EPashkin/gtk-bootstrap/releases/download/$BUNDLE/deps.txz"
	tar xf deps.txz
	cd "$WD"
	export PKG_CONFIG_PATH="$HOME/local/lib/pkgconfig"
fi

cargo build --features "$FEATURES" --jobs 1 "$@"
