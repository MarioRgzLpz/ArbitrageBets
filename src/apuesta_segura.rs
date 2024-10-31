use std::collections::HashMap;
use crate::{cuota::Cuota, evento::EventoDeportivo};

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
