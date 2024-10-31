use std::collections::HashMap;
use crate::{cuota::Cuota, evento::EventoDeportivo};

pub struct ApuestaSegura {
    evento: EventoDeportivo,
    cuotas: HashMap<String, Cuota>
}

impl ApuestaSegura {
    
    pub fn new(evento: EventoDeportivo) -> Self {
        ApuestaSegura {
            evento,
            cuotas: HashMap::new()
        } 
    }
}
