# ArbitrageBets
Mario Rodríguez López

# Descripción del problema

Apostar requiere de tiempo, conocimiento y suerte. Pero no todas las apuestas son así. Existe el Arbitrage Betting, o apuestas seguras, que es una estrategia que aprovecha las diferencias en las cuotas ofrecidas por distintas casas de apuestas para asegurar una ganancia, sin importar el resultado de un evento deportivo. Esta estrategia se basa en colocar apuestas en todos los posibles resultados de un evento en diferentes casas de apuestas, de manera que se cubran todas las opciones y se garantice una ganancia debido a la disparidad en las cuotas.
[Explicación en detalle](https://oddspedia.com/es/apuestas/estrategias/arbitrage-betting)

Puesto que estas disparidades no duran gran cantidad de tiempo es necesario recibir la información de las apuestas de manera casi inmediata y accesible.

# Propuesta de solución

Realizar una aplicación que obtenga apuestas seguras de distintas casas de apuestas y que notifique mediante Telegram/Twitter a traves de un bot.

# Documentación 

- [¿Qué es una arbitrage bet?](docs/arbitrage_bet.md)

- [Personas](docs/user-jorneys.md)

- [Historias de usuario](docs/user-stories.md)

- [Milestones](docs/milestones.md)

# Herramientas utilizadas

- Lenguaje de programación: `Rust`

- Gestor de dependencias: `Cargo`. [Más información](docs/gestor-dependencias.md)

- Gestor de tareas: `cargo-make`. [Más información](docs/gestor-tareas.md)
    - Si no esta instalado su instalación es muy simple y basta con ejecutar el siguiente comando:
    `cargo install cargo-make`
    - Para formatear el codigo y corregir la sintaxis realizamos: `cargo make check`
    - Para limpiar los archivos de compilación realizamos: `cargo make clean`
    - Para correr los tests realizamos: `cargo make test`
    - Para compilar el proyecto realizamos: `cargo make build`
    - Con `cargo make full` corregimos sintaxis, compilamos y corremos tests.
    - Con `cargo make clean-full` realizamos todo lo anterior pero primero limpiamos los archivos de compilación.

- [Aserciones](docs/herramientas-test-aserciones.md) de Rust y [test-runner](docs/herramientas-test-testrunners.md): 
    - Para instalar el test runner simplemente usamos el siguiente comando:
    `cargo install cargo-nextest --locked`
    - Para usar nuestro test runner simplemente hacemos: `cargo nextest run`

- [Imagen Docker](docs/imagen-docker.md)
    
## Contenedor de pruebas

Se puede construir una imagen del contenedor y ejecutarla con

```bash
docker build -t mariorgzlpz/arbitragebets . && docker run -u 1001 -t -v `pwd`:/app/test mariorgzlpz/arbitragebets
```

También, podemos usar la imagen que está en [DockerHub](https://hub.docker.com/r/mariorgzlpz/arbitragebets)

```bash
docker run -t -v `pwd`:/app/test mariorgzlpz/arbitragebets:latest
```