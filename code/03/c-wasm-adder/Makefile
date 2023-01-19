adder.wasm: adder.c
	clang \
		--target=wasm32-unknown-unknown -nostdlib \
		-O3 \
		-Wl,--no-entry -Wl,--export-dynamic \
		-o adder.wasm \
		adder.c

clean:
	rm adder.wasm
