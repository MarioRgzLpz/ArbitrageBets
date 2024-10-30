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

    pub fn agregar_evento(&mut self, evento: EventoDeportivo, cuotas: Vec<Cuota>) -> Result<(),String> {

        for cuota in &cuotas {
            if !evento.get_resultados().contains(&cuota.get_resultado()) {
                return Err(format!("Resultado no válido '{}' en la cuota.", cuota.get_resultado()));
            }
        }

        self.cuotas.insert(evento, cuotas);
        Ok(())
    }
}