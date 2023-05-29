# Default build
	cargo build

# Build for windows
build-windows:
	cargo build --target=x86_64-pc-windows-gnu --verbose

# Build the Docker image
docker-build:
	docker build -t mini-poker .

# Run the Docker container
docker-run:
	docker run mini-poker