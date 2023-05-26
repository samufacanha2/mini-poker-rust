# Build the Docker image
build:
	docker build -t mini-poker .

# Run the Docker container
run:
	docker run mini-poker