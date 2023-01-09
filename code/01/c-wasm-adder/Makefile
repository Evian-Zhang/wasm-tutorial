adder.wasm: adder.c
	clang --target=wasm32-unknown-unknown -O3 -nostdlib -Wl,--no-entry -Wl,--export-all -o adder.wasm adder.c

clean:
	rm adder.wasm
