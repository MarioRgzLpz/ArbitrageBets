Vamos a desarrollar lo planteado en [elección de herramientas de testing](https://github.com/MarioRgzLpz/ArbitrageBets/issues/19).

A la hora de elegir herramientas de testing debemos tener en cuenta el nivel de abstracción que necesitamos y cuales son las necesidades de dicho nivel.

# Testing en Rust

## Test runners

Para elegir nuestro test runner o framework debemos tener en cuenta los criterios ya nombrados en [#19](https://github.com/MarioRgzLpz/ArbitrageBets/issues/19) y algunas adicionales como son:

- Buen mockeo: que permita simular objetos y posibles casos a testear.
- Capacidad de depuración: Opciones integradas para pausar y analizar el estado del programa durante la ejecución de tests.
- Agrupación de tests: Que permita organizar los tests en grupos y subgrupos lógicos.
- Escalabilidad: Que se permita configurar escenarios más complejos si el proyecto crece.
- Rendimiento de los tests: Que permita paralelismo o concurrencia.
- Integración con el ecosistema Rust o directamente nativo para Rust.

### Estándar - cargo test

Rust trae por defecto soporte para la implementacion de unit testing, simplemente colocando `#[test]` encima de una función para que se trate como un test (esto se incorpora luego en practicamente todos los test runners ya que es la forma de marcar funciones como tests). Esto luego permite que con la llamada `cargo test` se ejecuten dichas funciones para realizar las comprobaciones necesarias del código implementado. Cumple con prácticamente todos los requisitos, permitiendo el mocking dentro de los test simplemente declarando las variables necesarias, usando la flag --show-output podemos depurar los errores además de que se pueden agrupar los tests con #cfg[#test] y usar modulos para dividir los test por funcionalidades.


### Nextest

Nextest es una herramienta para reemplazar a `cargo test` (tiene las mismas funcionalidades y añade algunas más). Tiene algunas cosas adicionales para correr test en CI pero la principal ventaja es que es hasta **3 veces más rapido**. Cumple con la mayoría de los criterios establecidos puesto que actua como una mejora de `cargo test`. Nos permite depurar mejor usando flags como `-v (Verbose)`


### Rstest

Libreria externa diseñada principalmente para test fixtures. Ayuda a escribir pruebas más simples aprovechando el concepto de **fixtures**. Una fixture es algo que encapsula las dependencias de las pruebas, ayudando a reducir la repetición de código al manejar la creación de objetos comunes (por ejemplo, casas de apuestas, cuotas...) que se utilizan en varias pruebas. Descarto esta herramienta porque no supone una gran ventaja con respecto a las anteriores además de no cumpli pero si aumenta la complejidad del testing.


### Proptest
Es un framework de testing que se centra en probar que ciertas propiedades de tu código se cumplen para entradas arbitrarias y, si se encuentra un error, automáticamente encuentra el caso de prueba mínimo para reproducir el problema. Es una opción interesante pero esta pensado para realizar testing a proyectos donde tengamos grandes cantidades de entradas para las funciones, lo que no es mi caso y además el rendimiento disminuye con respecto al estándar o `nextest`.


### Maelstrom
Maelstrom es un test runner de código abierto para `Rust`, `Go` o `Python` que encapsula pruebas en microcontenedores herméticos, ejecutándolos localmente o distribuyéndolos en clústeres. Queda descartado por ser poco conocido, bastante reciente y tener issues como [este](https://github.com/maelstrom-software/maelstrom/issues/13) abierto desde hace un año indicando que no cumple con muchos requisitos de seguridad.


### cargo-qtest
Un test runner interactivo que mejora la experiencia de pruebas en Rust al permitir selección flexible y ejecución de pruebas con patrones personalizados. Es una pequeña mejora a `cargo test` añadiendo como funcionalidad principal el permitir la ejecución de test de manera visual y flexible. Lo descarto por ser poco conocido, no añadir nada muy relevante al estandar pero si deuda técnica.


## Elección de test runner

Finalmente por la estructura y futuro del proyecto he decidido usar `nextest` que tiene una implementación muy sencilla y integración con el estándar, releases recientes y amplia las funciones que se pueden hacer con `cargo test` sin aumentar en gran medida la deuda técnica ni la dificultad y muy querido en la comunidad, además de las ventajas nombradas anteriormente.

## Herramientas CLI

En este caso lo normal para compilar y ejecutar los test es usar el estándar que encontramos en el lenguaje que para Rust es `cargo test`, pero como quiero incorporar el uso de `nextest` simplemente la suborden a ejecutar ahora es `cargo nextest run` que realiza lo mismo y como herramienta de CLI amplia al estándar con algunas otras flags como pueden ser --color para cambiar el color de la salida de los test.