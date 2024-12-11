Vamos a desarrollar lo planteado en [elección de herramientas de testing](https://github.com/MarioRgzLpz/ArbitrageBets/issues/19).

A la hora de elegir herramientas de testing debemos tener en cuenta el nivel de abstracción que necesitamos y cuales son las necesidades de dicho nivel.

# Testing en Rust

## Aserciones

Para elegir la libreria de aserciones voy a tener en cuenta principalmente dos cosas:

- Que no aumente en gran medida la deuda técnica.
- Que permita mensajes de fallo personalizado o que por defecto sean lo suficientemente claros.

La libreria de aserciones nativa de Rust junto con `matches!` (comprueba que dos expresiones coincidan), nos ofrece todo lo que pedimos permitiendo distintos tipos de aserciones y con la capacidad de elegir si una aserción se debe comprobar en release o no, usando el prefijo `debug` antes de la expresión.

También podemos encontrar algunas macros en la libreria estándar que amplían aserciones como `assert_matches!(valor,patrón)` que verifica si una expresión coincide con el patrón proporcionado. Es similar a usar `assert!(matches!(valor, patrón))` pero permite imprimir en depuración el valor que hizo que fallara. Tiene un [issue](https://github.com/rust-lang/rust/issues/82775) abierto en la que se explica que tiene un comportamiento inestable en algunos casos.

Encontramos también librerias externas (`crates`) para aumentar los posibles casos de aserciones y dar mensajes más precisos sobre los posibles errores:
- [`static_assertions`](https://crates.io/crates/static_assertions): Permite tener aserciones en tiempo de compilación y parar la compilación si es que falla. Tiene poco soporte, con su última release de hace 5 años, además de ser un tipo de aserción demasiado específico y que se puede cubrir con las aserciones nativas.
- [`assertor`](https://crates.io/crates/assertor): Crate que permite hacer las aserciones de los tests y los mensajes de error más legíbles. Cuenta con poca comunidad y poco soporte (3 releases en 3 años).
- [`assertables`](https://crates.io/crates/assertables): Crate que al igual que la anterior plantea ciertas macros para hacer las aserciones más legíbles para el ser humano. Es muy reciente ya que solo tiene 3 meses.
- [`assert2`](https://crates.io/crates/assert2): Añade dos macros bastante útiles pero que vamos a evitar puesto que con las nativas podemos hacer exactamente lo mismo aunque debamos pensar algo más.

Encontramos muchos crates similares a `assertor` y `assertables` como `more_asserts`, `cool_asserts`, que tan solo pretenden hacer las aserciones más legibles y que vamos a evitar para no aumentar la deuda técnica.

### Elección de aserciones

Puesto que no vamos a hacer cosas tan complejas y el proyecto no va a ser tan grande no es necesario usar aserciones en tiempo de compilación, ni añadir deuda técnica solo por hacer más legible el testing por lo que para las aserciones elegimos las estandar que trae Rust y no usaremos `assert_matches!` para evitar problemas.

