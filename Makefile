build-lib:
	wasm-pack build --target web
	cp ./pkg/factors.js ./web/lib/
	cp ./pkg/factors_bg.wasm ./web/lib/
