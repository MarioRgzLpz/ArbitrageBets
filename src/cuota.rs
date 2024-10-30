pub struct Cuota {
    resultado: String,
    valor: f64
}

impl Cuota {

    pub fn new(resultado: String, valor: f64) -> Self {

        Cuota {
            resultado,
            valor
        }
    }

    pub fn get_resultado(&self) -> &String{
        &self.resultado
    }
}