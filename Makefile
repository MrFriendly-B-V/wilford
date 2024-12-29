all: help
.SILENT:

SHELL := /bin/bash

.PHONY: help
help:
	echo "Wilford OAuth2 Server"
	echo "Commands: "
	echo "- up				: Start all Docker containers required for a local installation"
	echo "- upload-all		: Build and upload all Docker images"
	echo "- upload-server 	: Build and upload the server Docker image"
	echo "- upload-docs		: Build and upload the docs Docker image"
	echo "- upload-ui	 	: Build and upload the ui Docker image"

test_oidc_key.pem:
	openssl genrsa -out ./test_oidc_key.pem 4096
	
test_oidc_key.pem.pub: test_oidc_key.pem
	openssl rsa -in ./test_oidc_key.pem -pubout -outform PEM -out ./test_oidc_key.pem.pub

config.json: sample_config.json
	cp sample_config.json config.json

config_docker.json: sample_config_docker.json
	cp sample_config_docker.json config_docker.json


.PHONY: up
up: test_oidc_key.pem test_oidc_key.pem.pub config_docker.json
	docker compose up -d
	echo "Wilford UI available at 	http://localhost:2522"
	echo "Wilford Docs available at	http://localhost:2523"
	echo "EspoCRM UI availabel at 	http://localhost:2524"
	echo "If this is the first run, please configure EspoCRM and Wilford."

.PHONY: dev
dev: test_oidc_key.pem test_oidc_key.pem.pub config.json ui/node_modules
	# Database
	docker compose up -d mariadb-wilford
	echo "Waiting for Database to start..."
	#sleep 5

	# Server
	echo "Starting server"

	cd server && \
		RUST_LOG=INFO,wilford=TRACE \
		CONFIG_PATH=$(shell pwd)/config.json \
		cargo run -p wilford & \
		export SERVER_PID=$$!;

	# Start UI
	echo "Starting frontend"
	cd ui && yarn run dev --clearScreen false & \
		export UI_PID=$$!;

	# Wait until user does Ctrl+C
	sleep 2
	echo "Server and UI running. Ctrl+C to exit"
	read -r -d '' _ </dev/tty

	# Kill UI and server
	echo "Killing programs"
	kill $(SERVER_PID)
	kill $(UI_PID)
ui/node_modules: ui/package.json ui/yarn.lock
	cd ui && yarn

.PHONY: upload-all
upload-all: upload-server upload-docs upload-ui

.PHONY: upload-server
upload-server: build-server
	docker push registry.mrfriendly.uk/wilford-server

.PHONY: upload-docs
upload-docs: build-docs
	docker push registry.mrfriendly.uk/wilford-docs

.PHONY: upload-ui
upload-ui: build-ui
	docker push registry.mrfriendly.uk/wilford-ui

.PHONY: build-server
build-server:
	docker build -t registry.mrfriendly.uk/wilford-server server/

.PHONY: build-docs
build-docs:
	docker build -t registry.mrfriendly.uk/wilford-docs docs/

.PHONY: build-ui
build-ui:
	docker build -t registry.mrfriendly.uk/wilford-ui ui/
