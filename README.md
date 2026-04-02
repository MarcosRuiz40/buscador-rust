# Buscador de palabras en archivos (Rust)
Herramienta CLI desarrollada en Rust que permite buscar palabras dentro de archivos de texto, mostrando líneas coincidentes y cantidad total de resultados.

## Ejecución rápida

```
cargo run
```

Luego la terminal le solicita:

* nombre del archivo
* palabra a buscar

## Funcionalidades

* Leer archivos `.txt`
* Buscar palabras dentro del contenido
* Ignorar mayúsculas y minúsculas
* Mostrar número de línea
* Contar coincidencias

## Ejemplo:

```
1: Rust es rápido
3: Me gusta Rust

Se encontraron 2 coincidencias
```

## Tecnologías

* Rust

## Decisiones aplicadas:
* Uso de `to_lowecase()` para que al momento de búscar alguna palabra no importe si tiene mayúsculas o minúsculas
* Iteración con `.Line()` y `.Enumerate`

## Aprendizajes

Este proyecto me permitió practicar:

* Manejo de archivos (`fs`)
* Entrada por consola (`stdin`)
* Iteradores (`.lines()`, `.enumerate()`)
* Manejo de strings

---

## 2. Agregar sección “Mejoras futuras”

- Soporte para argumentos por línea de comandos
- Flags como `i`
- Manejo de errores sin `expect`
- Modularización del código

## Sobre mí

Soy estudiante de desarrollo de software enfocado en Rust y backend.
Este proyecto es parte de mi aprendizaje construyendo herramientas reales.
