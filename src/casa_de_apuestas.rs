use std::collections::HashMap;
use crate::{cuota::Cuota, evento::EventoDeportivo};

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

    pub fn agregar_evento(&mut self, evento: EventoDeportivo, cuotas: Vec<Cuota>) -> Result<(),String> {

        if cuotas.len() != 3 {
            return Err(format!("A un evento solo se pueden asociar tres cuotas"));
        }

        let mut resultados_set = std::collections::HashSet::new();
        for cuota in &cuotas {
            if !resultados_set.insert(cuota.get_resultado()) {
                return Err(format!("No se puede asociar el resultado {:?} más de una vez", cuota.get_resultado()));
            }
        }

        let nombre_evento = evento.get_nombre().clone();

        self.cuotas.insert(nombre_evento, cuotas);
        Ok(())
    }
}
