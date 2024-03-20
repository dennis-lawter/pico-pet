build:
	cargo build --bin pico-pet --release
run: build
	echo "Searching for Raspberry Pi Pico..."
	@while [ ! -d "/media/$$USER/RPI-RP2" ]; do \
        sleep 1; \
    done; \
    echo "Raspberry Pi Pico mounted."
	cargo run --bin pico-pet --release
sprites:
	rgba8888-to-rgb332 -m 0b111_000_11 -i ./sprite_png/ -o ./sprite_raw/
fonts:
	rgba8888-to-rgb332 --monochrome -i ./font_png/ -o ./font_raw/
tracks:
	peat_track_compiler -i ./sound_peat/ -o ./sound_raw/
