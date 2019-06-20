build:
	cargo build --release
build_linux: src/main.rs
	cargo build --release --target x86_64-unknown-linux-musl

package: build_linux build
	target/release/rpm-builder \
		--exec-file "target/x86_64-unknown-linux-musl/release/rpm-builder:/usr/bin/rpm-builder" \
		--config-file "target/rpm-builder.bash:/etc/bash_completion.d/rpm-builder.bash" \
		--version ${VERSION} \
		--compression gzip \
		rpm-builder