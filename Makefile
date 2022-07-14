docker-release:
	cross build --target x86_64-unknown-linux-musl --release

docker: docker-release
	docker build --network host -t dwd:latest .

docker-test:
	docker run --rm -it --name dwd \
		-v "${PWD}/config.yaml":/app/config.yaml \
		dwd:latest

docker-publish:
	docker image tag dwd:latest xieaolin/dwd:latest
	docker image push xieaolin/dwd:latest
