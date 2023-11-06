# Информационной системы для адвокатской конторы

## Установка

1. Для начала нужно установить rust
2. Затем выполнить установку diesel

```sh
cargo install diesel_cli --no-default-features --features postgres
```

3. Создаём файл `.env` по примеру `.env.example`

4. Далее необходимо установить docker и docker-compose
5. Затем нужно поднять контейнеры с администратором и базой данных (только локально)

```sh
docker-compose up
```

6. Убеждаемся, что всё запустилось (может упасть, если порты заняты, в этом случае их нужно либо освободить, либо `.env`/`.docker-compose.yml` подредачить)
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
