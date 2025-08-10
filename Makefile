.PHONY: dev clean

DOCKER_DEV = docker-compose.dev.yml
DOCKER_PROD = docker-compose.prod.yml

dev:
	@echo "Starting development environment..."
	docker compose -f $(DOCKER_DEV) up --build
devd:
	@echo "Starting development environment..."
	docker compose -f $(DOCKER_DEV) up --build -d

down:
	@echo "Stopping development environment..."
	docker compose -f $(DOCKER_DEV) down --remove-orphans

clean:
	@echo "Stopping environment..."
	docker compose -f $(DOCKER_DEV) down --remove-orphans
