start_db:
	docker compose -f docker-compose.test.yml up postgres

setup_diesel:
	diesel setup
