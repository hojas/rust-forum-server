start_db:
	docker compose -f docker-compose.test.yml up postgres -d

setup_diesel:
	diesel setup

migration_create_users:
	diesel migration generate create_users

migration_create_posts:
	diesel migration generate create_posts

migration_create_collected_posts:
	diesel migration generate create_collected_posts

migration_create_comments:
	diesel migration generate create_comments

run_migration:
	diesel migration run
	diesel migration redo
