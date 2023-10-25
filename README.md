# Rust-Developer OTUS course 2023-05

#### Homework 14


**Графический интерфейс "Умной розетки"**

Научиться пользоваться GUI фреймворками Rust.

Результатом является:
Приложение "Умная розетка по TCP" с GUI.

---

Описание:

Реализация на основе `eframe` - `egui` фреймворка

- `smart_house_gui` - обеспечивает функциональность клиента и сервера устройства TCP-сокета.
- `net_socket_emulator` -  запускает TCP-сервер на порту 8888.
- `tcp_socket_ui` - реализация `egui` UI для включения / отключения "умной розетки по TCP" и отображения ее состояния.

Процесс запуска:

запуск TCP сервера для умной раозетки
```bash
cargo run --example net_socket_emulator
```

запуск UI для взаимодействия умной розеткой
```bash
cargo run --example tcp_socket_ui
```
