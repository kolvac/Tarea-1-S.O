# Tarea 1: Microkernel y Cliente-Servidor

Simulación en Rust. Sistemas Operativos, Universidad Alberto Hurtado.

**Autores**: Lukas Ibaceta, Marcelo Urrutia.
**Profesor**: Fabio Sáez.
**Fecha**: 14 de Septiembre de 2026.

## Componentes
El proyecto se divide en módulos independientes bajo `src/`.
- `main.rs`: Binario central. Arranca ambas simulaciones.
- `microkernel.rs`: Modela procesos con structs. Utiliza enums `MensajeIPC`. El núcleo enruta los mensajes. Aísla fallas estructurales. Castiga el rendimiento operativo.
- `cliente_servidor.rs`: Structs de `Petición` y `Respuesta`. Comunicación directa sin núcleo intermediario. Dispara la velocidad. Crea dependencia absoluta al servidor.

## Uso
El control de dependencias lo gestiona `Cargo.toml`. Ejecuta el proyecto en la terminal.

```bash
cargo run
```
