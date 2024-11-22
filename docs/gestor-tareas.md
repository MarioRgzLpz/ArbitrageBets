Para elegir el gestor de tareas vamos a seguir los criterios nombrados en [elección del toolchain](https://github.com/MarioRgzLpz/ArbitrageBets/issues/13) entre los que se incluyen los siguientes:

- **Complejidad**: Debemos tener en cuenta para que vamos a usar la herramienta y no escoger una muy compleja y con demasiadas funcionalidades que no vamos a necesitar. .
- **Compatibilidad con el resto de herramientas**: La herramienta debe ser compatible con el resto de herramientas a usar.
- **Mantenimiento**: Es importante que el gestor de tareas tenga un desarrollo estable y bien mantenido. Evitar herramientas descontinuadas o poco mantenidas garantiza que no enfrentes problemas de rendimiento, seguridad o falta de actualizaciones en el futuro.
- **Integración con el ecosistema Rust**: La herramienta a elegir debe integrarse correctamente con el lenguaje de Rust.

Para automatizar tareas en Rust, hay varias opciones que pueden ayudar a gestionar el flujo de trabajo, ya sea para compilar, probar, formatear, generar documentación, o realizar tareas personalizadas. Aquí presento algunas de las opciones más usadas y también las que se recomiendan en la documentación de Rust:

- `cargo-make`: Extensión de `cargo` que permite definir tareas complejas en un solo archivo y orquestarlas en secuencia, incluyendo comandos específicos de Rust y otros del sistema. Es un "task runner" similar a make, pero diseñado específicamente para trabajar con proyectos de `Rust` y `Cargo`.
- `cargo xtasks`: Es un gestor de tareas que a diferencia de cargo make no se debe instalar en local si no que es una dependencia por lo que su integración con Rust es mejor ya que no se debe instalar nada. Es más complejo que el resto y se deben añadir más dependencias.
- `just`: Es un task runner muy simple con una sintaxis similar a un Makefile y que tiene algunas buenas extensiones que permiten pasar argumentos a las tareas o usar comentarios para explicar las tareas.
- `make`: Es una de las herramientas más comunes y muy usada cuyo funcionamiento se basa en un archivo llamado Makefile, donde se definen tareas a través de "reglas" con condiciones y dependencias específicas.
- `task`: Es un task runner para multiples lenguajes escrito en Go y con una sintaxis similar a Makefile pero empleando un fichero con formato yaml.
- `ninja`: Es un gestor de tareas que se centra en la velocidad de ejecución y esta programado en Go.

Finalmente he decidido usar `cargo-make` por las siguientes razones.

- Soporte nativo para tareas de `Rust` (build, test, fmt, etc.).
- Permite configurar tareas personalizadas en Makefile.toml.
- Permite definir dependencias entre tareas.
- Es fácil de usar e integra bien con el ecosistema de `Rust`.
- La deuda técnica es baja puesto que esta hecho para Rust específicamente y además se actualiza constantemente.
- No solo permite ejecutar comandos rust por lo que ofrece flexibilidad.