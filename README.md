# Zeus

Утилита для управления развернутыми локально проектами

## Команды

- setup - Клонирует и обновляет репозитории, переходит на нужную ветку
- upgrade - Обновляет конфиг до формата последней версии, обновляет репозитории
- deploy - Запускает сервисы в K8S
- dismiss - Останавливает сервисы в K8S


# TODO:

- Перенести аргументы в конфиг и передавать только его во все действия. Позволит иметь настраиваемые default значения для аргументов CLI (например ветка или namespace при деплое)