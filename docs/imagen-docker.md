# Elección de imagen

Como bien se indica en el guion y [aquí](https://github.com/MarioRgzLpz/ArbitrageBets/issues/31) necesitamos elegir una imagen de docker para nuestro contenedor.

## Criterios

Para la elección de la imagen vamos a seguir los siguientes criterios:

- Tamaño reducido de la imagen.
- Que este actualizada y se actualice con frecuencia.
- Preferiblemente oficial o de un publisher verificado.
- Que tenga Rust instalado por defecto.

## Posibles imágenes

Como requisito indispensable es que traiga Rust instalado por defecto por lo que las que muestro aquí todas lo traen.

### [Debian Rust](https://hub.docker.com/_/debian)

Encontramos `Debian` con `Rust` con sus dos variantes `bookworm` y `bullseye`. Esta es la imagen por defecto al descargar una imagen de rust. Ambas imagenes traen gran cantidad de dependencias por defecto y son bastante pesadas, por lo que descartamos su uso.

### [Slim Rust](https://hub.docker.com/_/debian)

Basada en debian-slim, es la imagen más ligera y trae las dependencias mínimas instaladas para correr `Rust`, por lo que deberemos configurarla nosotros. Es una imagen oficial y se actualiza constantemente.

### [Alpine Rust](https://hub.docker.com/_/alpine)

Imagen muy ligera aunque algo más pesada que Slim (~50MB). Trae muy pocas dependencias por defecto y usa `musl libc` cosa que nos da problemas para poder usar `cargo install` ya que necesitamos `glibc`. Por lo tanto, queda descartada. 

### [Bitnami Rust](https://hub.docker.com/r/bitnami/rust)

Imagen basada en `Debian` bastante más pesada que las anteriores. Se mantiene actualizada y trae Rust por defecto con un conjunto de dependencias. Por su tamaño, la descartamos.

## Elección de imagen

Comparando los tamaños y viendo lo que yo necesito en mi proyecto, he decidido usar rust:slim, que me permite realizar lo que yo quiero con el menor tamaño posible, evitando instalar dependencias adicionales como implicaría, por ejemplo, el uso de alpine.
