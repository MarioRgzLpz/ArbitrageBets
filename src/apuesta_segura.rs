use crate::{cuota::Cuota, evento::EventoDeportivo};

pub struct ApuestaSegura {
    evento: EventoDeportivo,
    cuotas: Vec<Cuota>
}

impl ApuestaSegura {
    
    pub fn new(evento: EventoDeportivo, cuotas: Vec<Cuota>) -> Result<Self,String> {

        for cuota in &cuotas {
            if !evento.get_resultados().contains(&cuota.get_resultado()) {
                return Err(format!("Resultado no válido '{}' en la cuota.", cuota.get_resultado()));
            }
        }

        Ok(ApuestaSegura { evento, cuotas })
    }
}