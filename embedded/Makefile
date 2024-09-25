build: sprites fonts tracks
	cargo build --bin pico-pet --release
run: build
	echo "Searching for Raspberry Pi Pico..."
	@while [ ! -d "/run/media/$$USER/RPI-RP2" ]; do \
        sleep 1; \
    done; \
    echo "Raspberry Pi Pico mounted."
	cargo run --bin pico-pet --release
sprites:
	../helpers/rgba8888-to-rgb332/target/release/rgba8888-to-rgb332 -m 0b111_000_11 -i assets/sprite_png/ -o assets/sprite_raw/
fonts:
	../helpers/rgba8888-to-rgb332/target/release/rgba8888-to-rgb332 --monochrome -i assets/font_png/ -o assets/font_raw/
tracks:
	../helpers/peat_track_compiler/target/release/peat_track_compiler -i assets/sound_peat/ -o assets/sound_raw/
