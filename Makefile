up:
	docker compose up -d

down:
	docker compose down

rust:
	docker compose exec rust /bin/bash

build:
	docker compose exec rust cargo build

run:
	docker compose exec rust cargo run

mysql:
	docker-compose exec mysql mysql -u diesel-user diesel -p

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: help up down rust build run mysql
.DEFAULT_GOAL := help
