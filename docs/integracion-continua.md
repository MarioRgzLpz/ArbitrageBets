Como bien hemos hablado [aquí](https://github.com/MarioRgzLpz/ArbitrageBets/issues/36) para el correcto desarrollo de software es necesario emplear integración continua, por lo que debemos elegir alguna herramienta que nos ayude con esta tarea.

## Criterios a cumplir por la herramienta

- De uso gratuito o que sea gratuito para proyectos open source.
- Que permita trabajar con Docker.
- Que permita trabajar con Rust.
- Que se pueda conectar con Github Checks API, necesario para poder integrarla con Github.

## Selección de herramientas

### [CircleCI](https://circleci.com/):
Proporciona un plan gratuito con limitaciones (máximo de 6000 minutos). Ofrece una integración con GitHub Checks y tiene soporte nativo para Docker, permitiendo la ejecución de construcciones en contenedores.

### [AppVeyor](https://www.appveyor.com/):
AppVeyor es una opción que nos permite integrar CI en proyectos de manera gratuita si es open source, como es nuestro caso. Además, se puede integrar fácilmente con Github.

### [GithubActions](https://docs.github.com/es/actions):
Es compatible con Docker de forma nativa y con Github . Además, ofrece una gran cantidad de recursos de forma gratuita.Encontramos actions para practicamente cualquier lenguaje.

### [GitLab](https://docs.gitlab.com/ee/ci/):
Compatible de forma nativa con Docker al igual que Github Actions y es posible integrarlo en repositorios de Github. Además, posee planes gratuitos para usarlo, aunque con ciertas limitaciones, por ejemplo si tu proyecto es codigo abierto es gratuito.

### [TravisCI](https://www.travis-ci.com): 
Compatible con Docker y Github. Aunque hay que tener en cuenta que es de pago, tiene un formato de prueba gratuito en el que no te pide datos de pago. Por no ser totalmente gratuito lo vamos a descartar.


## Decisión
Como podemos ver hay muchas opciones que podemos considerar válidas pero, he decidido usar GithubActions y AppVeyor por ser ambos gratis, además de no requerir instalación en local y la facilidad que tienen de integración con Github. También he añadido Circle-Ci por que lo sugiere el guión.
Para GithubActions he decidido probar las últimas 3 versiones de Rust porque según el [repòsitorio de nextest](https://github.com/nextest-rs/nextest), son para las cuales la construcción de los tests esta soportada y como cargo nextest run hace un build de los tests esto es necesario.