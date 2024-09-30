IMAGE_NAME := taxes-compare-webserver
APP_NAME := taxes-compare
USERNAME := simmonsd99
VERSION := 0.0.4-local

install-local:
	npm install

local:
	npm start

clean:
	rm -rf node_modules
	rm -rf build
	rm -rf package-lock.json

lint:
	npm run lint

build:
	docker build --no-cache . -t $(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)

# Run published image... maybe make another make target for local image.
run:
	docker run --rm -p 8080:8080 $(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)

publish: build
	docker push $(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)

.PHONY: build
