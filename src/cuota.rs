use crate::evento::Resultados;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Cuota {
    resultado: Resultados,
    valor: f64,
}

impl Cuota {
    pub fn new(resultado: Resultados, valor: f64) -> Self {
        Cuota { resultado, valor }
    }

    pub fn get_resultado(&self) -> &Resultados {
        &self.resultado
    }

    pub fn get_valor(&self) -> f64 {
        self.valor
    }
}
