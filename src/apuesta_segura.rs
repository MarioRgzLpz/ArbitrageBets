use std::collections::HashMap;
use crate::{cuota::Cuota, evento::EventoDeportivo};

/// La clave de `cuotas` es un String representando el nombre de la casa de apuestas, y el String de `evento` representa el nombre del evento
pub struct ApuestaSegura {
    evento: String,
    cuotas: HashMap<String, Cuota>
}

impl ApuestaSegura {
    
    pub fn new(evento: String) -> Self {
        ApuestaSegura {
            evento,
            cuotas: HashMap::new()
        } 
    }
}
