Vamos a desarrollar lo planteado en [elección de herramientas de testing](https://github.com/MarioRgzLpz/ArbitrageBets/issues/19).

A la hora de elegir herramientas de testing debemos tener en cuenta el nivel de abstracción que necesitamos y cuales son las necesidades de dicho nivel.

# Testing en Rust

## Aserciones

Para elegir la libreria de aserciones lo más importante será que tenga una buena integración con el ecosistema de Rust, que sea flexible 

En Rust encontramos tres tipos de aserciones estandar que a su vez se pueden comportar de dos formas distintas en función de si contienen el prefijo debug o no, útil para evitar que algunas aserciones se comprueben cuando se usa el modo realease. Las distintas aserciones son `assert!`, `assert_eq!`, `assert_ne!` y las versiones con debug. Estas permiten todo lo necesario para realizar aserciones en Rust tanto simples como complejas.

También podemos encontrar algunas macros en la libreria estandar(previamente era un crate) que amplian aserciones como `assert_matches!(valor,patron)` que verifica si una expresion coincide con el patrón proporcionado. Es similar a usar `assert!(matches!(valor, patrón))` pero permite imprimir en depuración el valor que hizó que fallará. Tiene un [issue](https://github.com/rust-lang/rust/issues/82775) abierto en la que se explica que tiene un comportamiento inestable en algunos casos.

Encontramos también librerias externas(crates) para aumentar los posibles casos de aserciones y dar mensajes más precisos sobre los posibles errores:
- `static_assertions`: Permite tener aserciones en tiempo de compilación y parar la compilación si es que falla. Centrado en aserciones invariantes como pueden ser constantes y tipos. Se añade como una dependencia y añade distintas macros a assert como `const_assert!` que permite comprobar que una expresion constante sea verdadera. 
- `assertor`: Es una libreria de aserciones Fluent que permite hacer las aserciones de los tests y los mensajes de error mas legíbles. Cuenta con poca comunidad y poco soporte (3 releases en 3 años).

### Elección de aserciones

Puesto que no vamos a hacer cosas tan complejas y el proyecto no va a ser tan grande no es necesario usar aserciones en tiempo de compilación, ni añadir deuda técnica solo por hacer más legible el testing por lo que para las aserciones elegimos las estandar que trae Rust y no usaremos `assert_matches!` para evitar problemas.

