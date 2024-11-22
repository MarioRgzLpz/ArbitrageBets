#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Resultados {
    GanaLocal,
    Empate,
    GanaVisitante,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct EventoDeportivo {
    nombre: String,
    resultados: Vec<Resultados>,
}

impl EventoDeportivo {
    pub fn new(nombre: String) -> Self {
        EventoDeportivo {
            nombre,
            resultados: vec![
                Resultados::GanaLocal,
                Resultados::Empate,
                Resultados::GanaVisitante,
            ],
        }
    }

    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }
}
