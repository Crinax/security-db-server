# Информационной системы для адвокатской конторы

## Установка

1. Для начала нужно установить rust
2. Затем выполнить установку diesel

```sh
cargo install diesel_cli --no-default-features --features postgres
```

3. Далее необходимо установить docker и docker-compose
4. Затем нужно поднять контейнеры с администратором и базой данных (только локально)

```sh
docker-compose up
```

5. Убеждаемся, что всё запустилось (может упасть, если порты заняты, в этом случае их нужно либо освободить, либо `docker-compose.yml` подредачить)
6. Создаём файл `.env`

```sh
DATABASE_URL=postgres://{username}:{password}@{host}:{port}/{db-name}
```

Вместо username, password, host, port, db-name подставить свои данные

7. Далее необходимо иницализировать Diesel

```sh
diesel setup
```

8. Затем, запускаем миграции

```sh
diesel migration run
```

9. После этого стартуем приложение

```sh
cargo run
```
