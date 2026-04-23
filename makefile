run:
	# 1. Kompilyatsiya
	cargo build --target wasm32-unknown-unknown
	
	# 2. WASM faylni nusxalash
	cp ../target/wasm32-unknown-unknown/debug/project_one.wasm ./game.wasm
	
	# 3. HTML yaratish
	echo '<!DOCTYPE html><html><head><meta charset="utf-8"><style>html,body,canvas{margin:0;padding:0;width:100%;height:100%;overflow:hidden;background:blue;}</style></head><body><canvas id="glcanvas"></canvas><script src="mq_js_bundle.js"></script><script>load("game.wasm");</script></body></html>' > index.html
	
	# 4. Serverni yoqish
	python3 -c 'import http.server; m = http.server.SimpleHTTPRequestHandler.extensions_map; m[".wasm"] = "application/wasm"; http.server.test(HandlerClass=http.server.SimpleHTTPRequestHandler, port=8080)'

