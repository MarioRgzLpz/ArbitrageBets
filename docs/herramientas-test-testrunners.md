Vamos a desarrollar lo planteado en [elección de herramientas de testing](https://github.com/MarioRgzLpz/ArbitrageBets/issues/19).

A la hora de elegir herramientas de testing debemos tener en cuenta el nivel de abstracción que necesitamos y cuales son las necesidades de dicho nivel.

# Testing en Rust

## Test runners

Para elegir nuestro test runner o framework debemos tener en cuenta los criterios ya nombrados en [#19](https://github.com/MarioRgzLpz/ArbitrageBets/issues/19) y algunas adicionales como son:

- Que permita paralelismo o concurrencia.
- Que permita aislar tests o conjuntos de tests.

### Estándar - cargo test

Rust trae por defecto soporte para la implementacion de unit testing, simplemente colocando `#[test]` encima de una función para que se trate como un test (esto se incorpora luego en practicamente todos los test runners ya que es la forma de marcar funciones como tests). Esto luego permite que con la llamada `cargo test` se ejecuten dichas funciones para realizar las comprobaciones necesarias del código implementado. Cumple con prácticamente todos los requisitos, permitiendo el mocking dentro de los test simplemente declarando las variables necesarias, usando la flag --show-output podemos depurar los errores además de que se pueden agrupar los tests con #cfg[#test] y usar modulos para dividir los test por funcionalidades.


### Rstest

Test framework diseñada principalmente para test fixtures. Ayuda a escribir pruebas más simples aprovechando el concepto de **fixtures**, donde podemos probar multiples casos para un mismo test usando [rstest] y [case], donde por cada `case` definido nos genera un test. Descarto esta herramienta porque no supone una gran ventaja con respecto a las anteriores, pero si aumenta la complejidad del testing.


### Proptest
Es un framework de testing que genera automaticamente casos de prueba para un test y si encuentra un fallo, genera el caso mínimo de test (para encontrar casos complejos que causan fallos). Es una opción interesante pero esta pensado para realizar testing a proyectos donde tengamos grandes cantidades de entradas para las funciones, lo que no es mi caso y además el rendimiento disminuye con respecto al estándar o `nextest`.


### cargo-qtest
Un test runner interactivo que mejora la experiencia de pruebas en Rust al permitir selección flexible y ejecución de pruebas con patrones personalizados. Es una pequeña mejora a `cargo test` añadiendo como funcionalidad principal el permitir la ejecución de test de manera visual y flexible. Lo descarto por ser poco conocido, no añadir nada muy relevante al estandar pero si deuda técnica.


## Elección de test runner

Finalmente por la estructura y futuro del proyecto he decidido usar la biblioteca nativa que tiene Rust para unit testing que evita deuda técnica y cubre las necesidades del proyecto.

## Herramientas CLI

En este caso lo normal para compilar y ejecutar los test es usar el estándar que encontramos en el lenguaje que para Rust es `cargo test`, pero encontramos la siguiente herramienta: 

Nextest es una herramienta para reemplazar a `cargo test` (tiene las mismas funcionalidades y añade algunas más). Tiene algunas cosas adicionales para correr test en CI pero la principal ventaja es que es hasta **3 veces más rapido**. Tiene una gran comunidad y soporte por lo que no aumenta la deuda técnica en gran medida.

Simplemente la suborden a ejecutar ahora es `cargo nextest run` que realiza lo mismo que `cargo test`.