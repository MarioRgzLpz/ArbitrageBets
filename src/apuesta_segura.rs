use std::collections::HashMap;
use crate::{cuota::Cuota, evento::EventoDeportivo};

pub struct ApuestaSegura {
    evento: String,
    /// La clave de `cuotas` es un String representando el nombre de la casa de apuestas que ofrece dicha cuota
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
