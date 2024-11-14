Para elegir el gestor de dependencias vamos a seguir los criterios nombrados en [elección del toolchain](https://github.com/MarioRgzLpz/ArbitrageBets/issues/13).

En `Rust`, el gestor de dependencias estándar es `Cargo`, que viene integrado en el ecosistema de `Rust`. Cargo maneja tanto las dependencias como la construcción de proyectos, por lo que no necesitas un gestor de dependencias adicional. Las razones por las que elijo Cargo son:

### Integración con el lenguaje: 
Cargo está diseñado para trabajar específicamente con `Rust`
 y se incluye en el toolchain de `Rust`, por lo que no requiere instalación adicional.

### Facilidad para declarar dependencias: 
Las dependencias se manejan en el archivo `Cargo.toml`, que es sencillo y fácil de leer.

### Resolución automática de versiones: 
`Cargo` permite especificar versiones de forma flexible, usando especificadores como ^, ~, o directamente números de versión exactos, y resolverá automáticamente las mejores versiones compatibles entre paquetes.

### Actualización y gestión automática de dependencias: 
Comandos como `cargo update` simplifican la actualización de dependencias a las últimas versiones compatibles.

`Cargo` cumple con el estado del arte porque es un gestor de dependencias que, además de su simplicidad, soporta el uso de lock files (archivo `Cargo.lock`) para asegurar la consistencia en las versiones usadas. Esto es similar a cómo funcionan otros gestores en lenguajes populares (ej., `NPM` en `JavaScript`, `Pip` en `Python`), pero adaptado para `Rust`
.