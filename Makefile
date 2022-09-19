docker-release:
	cross build --target x86_64-unknown-linux-musl --release

docker: docker-release
	docker build --network host -t xieaolin/dwd:latest .

docker-test:
	docker run --rm -it --name dwd \
		-v "${PWD}/config.yaml":/app/config.yaml \
		xieaolin/dwd:latest

docker-publish:
	docker image push xieaolin/dwd:latest
