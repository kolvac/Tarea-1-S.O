pub struct Peticion {
    pub operacion: String,
}

pub struct Respuesta {
    pub resultado: String,
}

pub struct Servidor;

impl Servidor {
    // El servidor ofrece un servicio bien definido
    pub fn procesar(&self, r: Peticion) -> Respuesta {
        Respuesta {
            resultado: format!("Operación procesada con éxito: {}", r.operacion),
        }
    }
}

pub struct Cliente {
    pub id: String,
}

impl Cliente {
    // El cliente solicita el servicio enviando una petición
    pub fn solicitar(&self, srv: &Servidor, op: &str) -> Respuesta {
        srv.procesar(Peticion {
            operacion: String::from(op),
        })
    }
}