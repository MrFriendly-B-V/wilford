all: help
.SILENT:

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

.PHONY: up
up: test_oidc_key.pem test_oidc_key.pem.pub config.json
	docker compose up -d
	echo "Wilford UI available at 	http://localhost:2522"
	echo "Wilford Docs available at	http://localhost:2523"
	echo "EspoCRM UI availabel at 	http://localhost:2524"
	echo "If this is the first run, please configure EspoCRM and Wilford."

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
