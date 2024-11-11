Para automatizar tareas en Rust, hay varias opciones que pueden ayudar a gestionar el flujo de trabajo, ya sea para compilar, probar, formatear, generar documentación, o realizar tareas personalizadas. Aquí presento algunas de las opciones más usadas:

- `cargo-make`
- `bacon`
- `make`

Finalmente he decidido usar `cargo-make` por las siguientes razones.


- Cargo-make es una herramienta muy popular para automatizar tareas en `Rust`. Es un "task runner" similar a make, pero diseñado específicamente para trabajar con proyectos de `Rust` y `Cargo`.

- Soporte nativo para tareas de `Rust` (build, test, fmt, etc.).
- Permite configurar tareas personalizadas en Makefile.toml.
- Permite definir dependencias entre tareas.
- Es fácil de usar e integra bien con el ecosistema de `Rust`.
