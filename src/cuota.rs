use crate::evento::Resultados;

#[derive(Debug, Clone)]
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
}
