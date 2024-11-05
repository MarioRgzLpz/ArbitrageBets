use std::collections::HashMap;
use crate::{cuota::Cuota, evento::EventoDeportivo};

/// La clave de `cuotas` es un String representando el nombre del evento y el String de `nombre` representa el nombre de la casa de apuestas
pub struct CasaDeApuestas {
    nombre: String,
    cuotas: HashMap<String, Vec<Cuota>>
}

impl CasaDeApuestas {

    pub fn new(nombre: String) -> Self {
        CasaDeApuestas {
            nombre,
            cuotas: HashMap::new()
        }
    }
    
}
