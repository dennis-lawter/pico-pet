submodules:
	git submodule update --init --recursive

install-deps:
	cd helpers/peat_track_compiler && cargo build --release
	cargo install --path helpers/peat_track_compiler
	# cd helpers/peat_track_player && cargo build --release
	# cargo install --path helpers/peat_track_player
	cd helpers/rgba8888-to-rgb332 && cargo build --release
	cargo install --path helpers/rgba8888-to-rgb332

setup-rust:
	rustup target add thumbv6m-none-eabi

init: submodules install-deps setup-rust

# Root-level targets for the embedded project
pico-build:
	cd embedded && make build

pico-run:
	cd embedded && make run
