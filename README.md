# Renderizador de Sistema Solar - Laboratorio

## Objetivo

El objetivo de este proyecto de laboratorio es practicar la creación de shaders interesantes mediante la modificación de colores utilizando los parámetros disponibles. Diseñarás y crearás 7 cuerpos celestes en tu renderizador de software utilizando únicamente shaders (sin texturas ni materiales).

### Requisitos:
1. **Crear una "estrella"** que servirá como el sol de tu sistema solar.
2. **Crear al menos un planeta rocoso**.
3. **Crear al menos un gigante gaseoso**.
   
---

## Instrucciones:

### 1. Clonar el Repositorio

Para comenzar, clona el repositorio a tu máquina local:

```bash
git clone https://github.com/nicollegordillo/Lab4-shaders.git
cd Lab4-shaders
```

### 2. Compilar y Ejecutar
* cargo build
* cargo run

## Puntos Completados:
### Diseño e Implementación de Shaders:
1. Estrella (Sol): Creado usando un shader simple con un efecto de resplandor y transiciones de color.
2. Planeta Rocoso con varias capas (Tierra): Implementado con un shader que simula detalles en la superficie como océanos, continentes y nubes que se mueven a su propio ritmo.
3. Gigante Gaseoso: Diseñado con un shader más complejo para simular capas gaseosas simuladas con lerp.
4. Efectos Atmosféricos: Nubes en movimiento y capas atmosféricas a al menos un planeta, dándole un toque dinámico.
5. Anillos Planetarios (Saturno): Implementados los anillos de Saturno utilizando shaders, con efectos dinámicos para simular su textura y apariencia.
6. Luna Orbitando un Planeta Rocoso (Tierra): Creada una luna que orbita la Tierra con un shader que simula su movimiento y textura.

## Cambiar entre Planetas
Puedes cambiar entre los planetas utilizando las teclas numéricas del teclado. Cada planeta o cuerpo celeste se asigna a una tecla específica:

NumPad 1: Júpiter

NumPad 2: Saturno (con anillos)

NumPad 3: Urano

NumPad 4: Venus

NumPad 5: Marte

NumPad 6: Tierra (con luna)

NumPad 7: Mercurio

NumPad 8: Sol

NumPad 0: Neptuno

## Screenshots
Neptuno:

![neptune](https://github.com/nicollegordillo/Lab4-shaders/blob/master/images/neptune.png)

Júpiter:

![jupiter](https://github.com/nicollegordillo/Lab4-shaders/blob/master/images/jupiter.png)

Saturno:

![saturno](https://github.com/nicollegordillo/Lab4-shaders/blob/master/images/saturn%20(2).png)

Urano:

![urano](https://github.com/nicollegordillo/Lab4-shaders/blob/master/images/urano.png)

Venus:

![venus](https://github.com/nicollegordillo/Lab4-shaders/blob/master/images/venus.png)

Marte:

![marte](https://github.com/nicollegordillo/Lab4-shaders/blob/master/images/mars.png)

Tierra:

![tierra](https://github.com/nicollegordillo/Lab4-shaders/blob/master/images/earth.png)

Mercurio:

![mercurio](https://github.com/nicollegordillo/Lab4-shaders/blob/master/images/mercury.png)

Sol:

![sol](https://github.com/nicollegordillo/Lab4-shaders/blob/master/images/sun.png)
