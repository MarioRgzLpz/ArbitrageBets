use std::collections::HashMap;
use crate::{cuota::Cuota, evento::EventoDeportivo};

pub struct CasaDeApuestas {
    nombre: String,
    /// La clave de `cuotas` es un String representando el nombre del evento al cual queremos asociar las cuotas
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
