mod microkernel;
mod cliente_servidor;

use microkernel::{Microkernel, Proceso, MensajeIPC};
use cliente_servidor::{Cliente, Servidor};

fn main() {
    println!("--- INICIANDO SIMULACIÓN DE MICROKERNEL ---");
    let mut mk = Microkernel::nuevo();

    mk.registrar_proceso(Proceso { id: 1, nombre: String::from("ProcesoUsuario") });
    mk.registrar_proceso(Proceso { id: 2, nombre: String::from("ServidorArchivos") });

    // Ya no se simula la respuesta "a mano": el proceso 2 la genera al
    // recibir la solicitud, y el microkernel se encarga de devolverla.
    mk.enviar_mensaje(1, 2, MensajeIPC::SolicitudLectura);

    println!("\n--- INICIANDO SIMULACIÓN CLIENTE-SERVIDOR ---");
    let servidor_principal = Servidor;
    let cliente_uno = Cliente { id: String::from("Cliente_101") };
    
    let respuesta = cliente_uno.solicitar(&servidor_principal, "Consultar base de datos");
    
    println!("[{}] Respuesta recibida del servidor: {}", cliente_uno.id, respuesta.resultado);
}