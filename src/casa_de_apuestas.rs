use std::collections::HashMap;
use crate::{cuota::Cuota, evento::EventoDeportivo};
pub struct CasaDeApuestas {
    nombre: String,
    cuotas: HashMap<EventoDeportivo, Vec<Cuota>>
}

impl CasaDeApuestas {

    pub fn new(nombre: String) -> Self {
        CasaDeApuestas {
            nombre,
            cuotas: HashMap::new()
        }
    }
}