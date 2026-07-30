# Conway's Game of Life

El juego dibuja cada celda con la funcion `point` sobre un
framebuffer pequeño y escala ese framebuffer para mostrarlo en una ventana más
grande.

## Como compilar y correr

```powershell
cargo run --release --bin conway_game_of_life
```

## Como generar el GIF

```powershell
cargo run --release --bin make_gif
```

## Controles

- `Space`: pausar o continuar la simulacion.
- `R`: reiniciar el patron inicial.
- `Esc`: cerrar la ventana.