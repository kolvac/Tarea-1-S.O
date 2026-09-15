#[allow(dead_code)]
#[derive(Debug)]
pub enum MensajeIPC {
    SolicitudLectura,
    RespuestaDatos(String),
}

pub struct Proceso {
    pub id: u32,
    pub nombre: String,
}

impl Proceso {
    // Cada proceso decide cómo responder a los mensajes que le llegan.
    // Esto representa que los servicios del sistema (aquí, el "servidor de
    // archivos") viven como procesos comunes fuera del núcleo, y no como
    // código privilegiado dentro de él, que es la idea central de un
    // microkernel.
    pub fn manejar_mensaje(&self, mensaje: &MensajeIPC) -> Option<MensajeIPC> {
        match mensaje {
            MensajeIPC::SolicitudLectura => {
                let contenido = format!("contenido_generado_por_{}.txt", self.nombre);
                Some(MensajeIPC::RespuestaDatos(contenido))
            }
            // Una respuesta no dispara, a su vez, otra respuesta.
            MensajeIPC::RespuestaDatos(_) => None,
        }
    }
}

pub struct Microkernel {
    procesos_registrados: Vec<Proceso>,
}

impl Microkernel {
    pub fn nuevo() -> Self {
        Microkernel { procesos_registrados: Vec::new() }
    }

    pub fn registrar_proceso(&mut self, proceso: Proceso) {
        self.procesos_registrados.push(proceso);
        let p = self.procesos_registrados.last().unwrap();
        println!("Microkernel: Proceso {} (ID: {}) registrado.", p.nombre, p.id);
    }

    fn buscar_proceso(&self, id: u32) -> Option<&Proceso> {
        self.procesos_registrados.iter().find(|p| p.id == id)
    }

    // El microkernel NO procesa la petición: solo la entrega al proceso
    // destino (búsqueda por id) y, si ese proceso genera una respuesta, la
    // enruta de vuelta al origen. Toda la comunicación pasa por esta única
    // vía de IPC, nunca por una llamada directa entre procesos, que es
    // justamente el rasgo distintivo que se pide simular.
    pub fn enviar_mensaje(&self, origen: u32, destino: u32, mensaje: MensajeIPC) {
        println!("Microkernel enrutando IPC de {} a {}: {:?}", origen, destino, mensaje);

        match self.buscar_proceso(destino) {
            Some(proceso) => {
                if let Some(respuesta) = proceso.manejar_mensaje(&mensaje) {
                    println!(
                        "Microkernel enrutando IPC de {} a {}: {:?}",
                        destino, origen, respuesta
                    );
                }
            }
            None => {
                println!("Microkernel: error, el proceso destino {} no existe.", destino);
            }
        }
    }
}