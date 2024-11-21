Para elegir el gestor de tareas vamos a seguir los criterios nombrados en [elección del toolchain](https://github.com/MarioRgzLpz/ArbitrageBets/issues/13) entre los que se incluyen los siguientes:

- **Facilidad de uso**: Un gestor de tareas debe ser intuitivo y sencillo de manejar.
- **Compatibilidad**: La herramienta debe ser compatible con el resto de herramientas a usar.
- **Deuda técnica**: Es importante que el gestor de tareas tenga un desarrollo estable y bien mantenido. Evitar herramientas con alta deuda técnica garantiza que no enfrentes problemas de rendimiento, seguridad o falta de actualizaciones en el futuro.
- **Estándares del lenguaje**: La herramienta a elegir debe seguir el estado del arte del lenguaje en el que programamos.

Para automatizar tareas en Rust, hay varias opciones que pueden ayudar a gestionar el flujo de trabajo, ya sea para compilar, probar, formatear, generar documentación, o realizar tareas personalizadas. Aquí presento algunas de las opciones más usadas y también las que se recomiendan en la documentación de Rust:

- `cargo-make`: Extensión de `cargo`que permite definir tareas complejas en un solo archivo y orquestarlas en secuencia, incluyendo comandos específicos de Rust y otros del sistema. Es un "task runner" similar a make, pero diseñado específicamente para trabajar con proyectos de `Rust` y `Cargo`.
- `just`: Es un task runner muy simple con una sintaxis similar a un Makefile y que tiene algunas buenas extensiones que permiten pasar argumentos a las tareas o usar comentarios para explicar las tareas.
- `make`: Es una de las herramientas más comunes y muy usada cuyo funcionamiento se basa en un archivo llamado Makefile, donde se definen tareas a través de "reglas" con condiciones y dependencias específicas.
- `task`: Es un task runner para multiples lenguajes escrito en Go y con una sintaxis similar a Makefile pero empleando un fichero con formato yaml.

Finalmente he decidido usar `cargo-make` por las siguientes razones.

- Soporte nativo para tareas de `Rust` (build, test, fmt, etc.).
- Permite configurar tareas personalizadas en Makefile.toml.
- Permite definir dependencias entre tareas.
- Es fácil de usar e integra bien con el ecosistema de `Rust`.
- La deuda técnica es baja, sigue actualizandose constantemente.
- No solo permite ejecutar comandos rust por lo que ofrece flexibilidad.