Para elegir el gestor de tareas vamos a seguir los criterios nombrados en [elección del toolchain](https://github.com/MarioRgzLpz/ArbitrageBets/issues/13).

Para automatizar tareas en Rust, hay varias opciones que pueden ayudar a gestionar el flujo de trabajo, ya sea para compilar, probar, formatear, generar documentación, o realizar tareas personalizadas. Aquí presento algunas de las opciones más usadas:

- `cargo-make`: Extensión de `cargo`que permite definir tareas complejas en un solo archivo y orquestarlas en secuencia, incluyendo comandos específicos de Rust y otros del sistema. 
- `bacon`: Herramienta de recarga en caliente que observa los cambios en el código fuente y compila o ejecuta automáticamente las pruebas al detectar modificaciones.
- `make`: Es una de las herramientas más comunes y muy usada cuyo funcionamiento se basa en un archivo llamado Makefile, donde se definen tareas a través de "reglas" con condiciones y dependencias específicas.

Finalmente he decidido usar `cargo-make` por las siguientes razones.


- Cargo-make es una herramienta muy popular para automatizar tareas en `Rust`. Es un "task runner" similar a make, pero diseñado específicamente para trabajar con proyectos de `Rust` y `Cargo`.

- Soporte nativo para tareas de `Rust` (build, test, fmt, etc.).
- Permite configurar tareas personalizadas en Makefile.toml.
- Permite definir dependencias entre tareas.
- Es fácil de usar e integra bien con el ecosistema de `Rust`.
