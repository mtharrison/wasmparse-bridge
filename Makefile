.PHONY: build

build:
	bash build.sh
clean:
	rm -rf target
	rm -rf pkg/wasmparse_bridge*