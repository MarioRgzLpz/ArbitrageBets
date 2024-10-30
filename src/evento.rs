#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Deporte {
    Futbol,
    Tenis
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct EventoDeportivo {
    nombre: String,
    resultados: Vec<String>,
    deporte: Deporte
}

impl EventoDeportivo {

    pub fn new(nombre: String, resultados: Vec<String>, deporte: Deporte) -> Self {

        EventoDeportivo {
            nombre,
            resultados,
            deporte
        }
    }

    pub fn get_resultados(&self) -> &Vec<String> {
        &self.resultados
    }
}