# Buscador de palabras en archivos (Rust)
CLI en rust para buscar palabras en un archivo de texto 

## Funcionalidades

* Leer archivos `.txt`
* Buscar palabras dentro del contenido
* Ignorar mayúsculas y minúsculas
* Mostrar número de línea
* Contar coincidencias

## Uso

Ejecutar:
```
cargo run
```

Luego la terminal le solicita:

* nombre del archivo
* palabra a buscar

## Ejemplo:

```
1: Rust es rápido
3: Me gusta Rust

Se encontraron 2 coincidencias
```

## Tecnologías

* Rust

## Aprendizajes

Este proyecto me permitió practicar:

* Manejo de archivos (`fs`)
* Entrada por consola (`stdin`)
* Iteradores (`.lines()`, `.enumerate()`)
* Manejo de strings

---

## 2. Agregar sección “Mejoras futuras”

```md
##  Mejoras futuras

- Soporte para argumentos por línea de comandos
- Flags como `-i` para ignorar mayúsculas
- Mejor manejo de errores

## Nota

Este proyecto está inspirado en el clásico ejemplo de "grep" de Rust, pero implementado desde cero como práctica y mejora personal.
