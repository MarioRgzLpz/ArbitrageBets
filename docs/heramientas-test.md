Vamos a desarrollar lo planteado en [elección de herramientas de testing](https://github.com/MarioRgzLpz/ArbitrageBets/issues/19).

A la hora de elegir herramientas de testing debemos tener en cuenta el nivel de abstracción que necesitamos y cuales son las necesidades de dicho nivel.

# Testing en Rust

## Aserciones
En Rust encontramos tres tipos de aserciones estandar que a su vez se pueden comportar de dos formas distintas en función de si contienen el prefijo debug o no, útil para evitar que algunas aserciones se comprueben cuando se usa el modo realease. Las distintas aserciones son: 
| Tipo               | Habilitado en Debug | Habilitado en Release | Propósito principal                                    |
|--------------------|---------------------|-----------------------|------------------------------------------------------|
| `assert!`          | Sí                 | Sí                   | Verificar siempre condiciones críticas.             |
| `assert_eq!`       | Sí                 | Sí                   | Verificar igualdad de valores siempre.              |
| `assert_ne!`       | Sí                 | Sí                   | Verificar desigualdad de valores siempre.           |
| `debug_assert!`    | Sí                 | No                   | Verificar durante desarrollo sin afectar producción.|
| `debug_assert_eq!` | Sí                 | No                   | Verificar igualdad en modo debug.                   |
| `debug_assert_ne!` | Sí                 | No                   | Verificar desigualdad en modo debug.                |

También podemos encontrar algunas macros en la libreria estandar(previamente era un crate) que amplian aserciones como `assert_matches!(valor,patron)` que verifica si una expresion coincide con el patrón proporcionado. Es similar a usar `assert!(matches!(valor, patrón))` pero permite imprimir en depuración el valor que hizó que fallará. Tiene un [issue](https://github.com/rust-lang/rust/issues/82775) abierto en la que se explica que tiene un comportamiento inestable en algunos casos.

También encontramos librerias externas(crates) para aumentar los posibles casos de aserciones y dar mensajes más precisos sobre los posibles errores:
- `static_assertions`: Permite tener aserciones en tiempo de compilación y parar la compilación si es que falla. Centrado en aserciones invariantes como pueden ser constantes y tipos. Se añade como una dependencia y añade distintas macros a assert como `const_assert!` que permite comprobar que una expresion constante sea verdadera. 

### Elección de aserciones

Puesto que no vamos a hacer cosas tan complejas y el proyecto no va a ser tan grande no es necesario usar aserciones en tiempo de compilación por lo que para las aserciones elegimos las estandar que trae Rust y no usaremos `assert_matches!` para evitar problemas.

## Test runners

Para elegir nuestro test runner o framework debemos tener en cuenta los criterios ya nombrados en [#19](https://github.com/MarioRgzLpz/ArbitrageBets/issues/19) y algunas adicionales como son:

- Buen mockeo: que permita simular objetos y posibles casos a testear.
- Capacidad de depuración: Opciones integradas para pausar y analizar el estado del programa durante la ejecución de tests.
- Agrupación de tests: Que permita organizar los tests en grupos y subgrupos lógicos.
- Escalabilidad: Que se permita configurar escenarios más complejos si el proyecto crece.
- Rendimiento de los tests: Que permita paralelismo o concurrencia.
- Reporte detallado de errores: Mensajes claros que indican exactamente qué falló y por qué.

### Estándar

Rust trae por defecto soporte para la implementacion de unit testing, simplemente colocando `#[test]` encima de una función para que se trate como un test (esto se incorpora luego en practicamente todos los test runners ya que es la forma de marcar funciones como tests). Esto luego permite que con la llamada cargo test se ejecuten dichas funciones para realizar las comprobaciones necesarias del código implementado. Como ventajas tenemos lo siguiente:
- **Integración nativa**: Ya está incluido en el lenguaje, sin necesidad de instalaciones adicionales.
- **Fácil de usar**: Solo necesitas ejecutar `cargo test` para ejecutar todas las pruebas.
- **Soporte para pruebas paralelas**: Ejecuta pruebas de manera concurrente por defecto, lo que mejora el rendimiento.
- **Agrupación de tests**: Usando `[#cfg(test)]` podemos compilar solo los tests cuando explicitamente se lo decimos mediante el uso de `cargo test` para permitir un aumento del rendimiento y velocidad de compilación. Permite agrupar los tests usando modulos.

### Nextest

Nextest es una herramienta para reemplazar a `cargo test` (tiene las mismas funcionalidades y añade algunas más). Tiene algunas cosas adicionales para correr test en CI pero la principal ventaja es que es hasta **3 veces más rapido**. Se puede usar con `cargo nextest`. Algunas de las ventajas son:

- **Interfaz limpia y eficiente**:  Resultados presentados de forma concisa, destacando qué pruebas pasaron y cuáles fallaron.
- **Detección de problemas en pruebas**:  
  - Identifica pruebas lentas y con fugas de memoria.  
  - Finaliza pruebas que excedan el tiempo límite.  
- **Filtrado avanzado y configuración personalizada**:  
  - Filtra pruebas usando un lenguaje integrado para ejecutar subconjuntos específicos.  
  - Configura ajustes individuales, como reintentos automáticos o marcarlas como "pesadas".  
- **Optimizado para CI (Integración Continua)**:  
  - Binarios precompilados para instalación rápida.  
  - Perfiles de configuración específicos para entornos de CI.  
  - Divide pruebas en múltiples trabajos de CI reutilizando compilaciones.  
  - Reintenta automáticamente pruebas fallidas y marca como inestables aquellas que pasan en intentos posteriores.  
  - Genera reportes en formatos estándar como XML de JUnit.   

### Rstest

Libreria externa diseñada principalmente para test fixtures. Ayuda a escribir pruebas más simples aprovechando el concepto de **fixtures**. Una fixture es algo que encapsula las dependencias de las pruebas, ayudando a reducir la repetición de código al manejar la creación de objetos comunes (por ejemplo, casas de apuestas, cuotas...) que se utilizan en varias pruebas. Las ventajas que encontramos son:

- **Reducción de repetición**:  
  Utiliza fixtures para evitar la duplicación de código al crear objetos comunes en varias pruebas.
- **Facilidad para crear pruebas parametrizadas**:  
  Genera pruebas de manera sencilla utilizando tablas de casos (`#[case]`), lo que permite probar varias entradas automáticamente.
- **Configuración flexible de pruebas**:  
  Permite inyectar múltiples fixtures en una misma prueba y configurar diferentes comportamientos según sea necesario.
- **Soporte para combinaciones de valores**:  
  Permite generar pruebas para todas las combinaciones posibles de valores de entrada, utilizando la anotación `#[values]`.

### Proptest
Es un framework de testing que se centra en probar que ciertas propiedades de tu código se cumplen para entradas arbitrarias y, si se encuentra un error, automáticamente encuentra el caso de prueba mínimo para reproducir el problema. Las principales ventajas son:

- **Pruebas basadas en propiedades**: Facilita la creación de pruebas que verifican propiedades generales de una función, no solo casos específicos.
- **Generación automática de entradas**: Crea automáticamente entradas para pruebas, cubriendo una amplia gama de casos.
- **Fácil integración con `cargo test`**: Compatible con el runner de pruebas estándar de Rust, lo que simplifica la configuración.
- **Detección de errores ocultos**: Ayuda a encontrar errores sutiles que pueden no ser obvios en pruebas manuales tradicionales.


## Maelstrom
Maelstrom es un test runner de código abierto para `Rust`, `Go` o `Python` que encapsula pruebas en microcontenedores herméticos, ejecutándolos localmente o distribuyéndolos en clústeres. Las principales ventajas incluyen:

- **Facilidad de uso**: Se puede usar como reemplazo directo de `cargo test` funcionando con las pruebas existentes con mínima configuración.
- **Aislamiento total**: Ejecuta cada prueba en su propio contenedor ligero, eliminando errores causados por dependencias implícitas o entre pruebas.
- **Escalabilidad**: Puede ejecutarse como un clúster; añadir más máquinas incrementa proporcionalmente el rendimiento.

---

## cargo-qtest
Un test runner interactivo que mejora la experiencia de pruebas en Rust al permitir selección flexible y ejecución de pruebas con patrones personalizados. Sus ventajas incluyen:

- **Interfaz interactiva**: Facilita la búsqueda y selección de pruebas basándose en patrones de nombres o rutas específicas.
- **Flexibilidad en la ejecución**: Permite ejecutar pruebas de distintos módulos simultáneamente, ideal para proyectos grandes.
- **Compatibilidad completa**: Funciona como una extensión de `cargo test`, soportando todos sus argumentos y flags.
- **Optimización del flujo de trabajo**: Simplifica la ejecución de pruebas específicas sin necesidad de recordar rutas complejas o comandos largos.

### Elección de test runner

Finalmente por la estructura y futuro del proyecto he decidido usar `nextest` que tiene una implementación muy sencilla y integración con el estándar, releases recientes y amplia las funciones que se pueden hacer con `cargo test` sin aumentar en gran medida la deuda técnica ni la dificultad y muy querido en la comunidad, además de las ventajas nombradas anteriormente. Tanto `rstest` como `proptest` tienen una curva de aprendizaje más alta y no son tan sencillos de implementar como el estándar, `cargo-qtest` no amplia mucho la funcionalidad de cargo test y `maelstrom` no es tan especifico para Rust además de ser bastante reciente y no tener tanto soporte. 
## Herramientas CLI

En este caso lo normal para compilar y ejecutar los test es usar el estándar que encontramos en el lenguaje que para Rust es `cargo test`, pero como quiero incorporar el uso de `nextest` simplemente la suborden a ejecutar ahora es `cargo nextest` que realiza lo mismo y como herramienta de CLI no supone una ventaja pero es necesario para usar nextest como nuestro test runner que si que incrementa el rendimiento.
