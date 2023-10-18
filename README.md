# Rust-Developer OTUS course 2023-05

#### Homework 13


**Веб-сервер "умного дома".**

Научиться пользоваться веб-фреймворками Rust.

Результатом является: Веб-сервер "умного дома".

---
## Примеры запуска

Запуск HTTP сервера
```bash
cargo run
```

Запуск эмуляторов устройств на TCP сокетах

```bash
cargo run --example run_socket_emulator -- --address 127.0.0.1:8080
```
```bash
cargo run --example run_socket_emulator -- --address 127.0.0.1:8090
```


## Примеры curl запросов

Создание комнат
```bash
curl -X POST "127.0.0.1:8888/room/kitchen"
```
```bash
curl -X POST "127.0.0.1:8888/room/bathroom"
```
Получение данных о комнатах
```bash
curl -X GET "127.0.0.1:8888/room"
```
```bash
curl -X GET "127.0.0.1:8888/room/kitchen"
```

Добавление устройств в комнаты
```bash
curl -X POST "127.0.0.1:8888/device/kitchen" -H 'Content-Type: application/json' -d '{"device_name": "socket_1", "address": "127.0.0.1:8080", "device_type": "tcp_socket"}'
```
```bash
curl -X POST "127.0.0.1:8888/device/bathroom" -H 'Content-Type: application/json' -d '{"device_name": "socket_1", "address": "127.0.0.1:8090", "device_type": "tcp_socket"}'
```
```bash
curl -X POST "127.0.0.1:8888/device/bathroom" -H 'Content-Type: application/json' -d '{"device_name": "socket_2", "address": "127.0.0.1:8091", "device_type": "tcp_socket"}'
```

Получение данных об устройствах в комнате
```bash
curl -X GET "127.0.0.1:8888/room"
```

Опрос состояния устройств
```bash
curl -X GET "127.0.0.1:8888/status/bathroom"
```
```bash
curl -X GET "127.0.0.1:8888/status/kitchen"
```

Удаление устройств и вывод отчета
```bash 
curl -X DELETE "127.0.0.1:8888/device/bathroom/socket_1"
```
```bash
curl -X GET "127.0.0.1:8888/room"
```

## Описание API эндпоинтов

| Type    | Method   | Endpoint                         |
|---------|----------|----------------------------------|
| ROOM    |          |                                  |
|         | `POST`   | `/room/{room_id}`                |
|         | `GET`    | `/room`                          |
|         | `GET `   | `/room/{room_id}`                |
|         | `DELETE` | `/room/{room_id}`                |
| DEVICES |          |                                  |
|         | `POST`   | `/device/{room_id}`              |
|         | `GET`    | `/device/{room_id}/{device_id}`  |
|         | `DELETE` | `/device/{room_id}/{device_id}`  |
| STATUS  |          |                                  |
|         | `GET`    | `/status/{room_id}`              |
|         | `GET`    | `/status/{room_id}/{device_id}`  |

