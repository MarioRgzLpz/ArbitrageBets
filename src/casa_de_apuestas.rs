use crate::cuota::Cuota;
use std::collections::HashMap;

/// La clave de `cuotas` es un String representando el nombre del evento y el String de `nombre` representa el nombre de la casa de apuestas
pub struct CasaDeApuestas {
    nombre: String,
    cuotas: HashMap<String, Vec<Cuota>>,
}

impl CasaDeApuestas {
    pub fn new(nombre: String, cuotas: HashMap<String, Vec<Cuota>>) -> Self {
        CasaDeApuestas { nombre, cuotas }
    }

    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }

    pub fn get_cuotas(&self) -> &HashMap<String, Vec<Cuota>> {
        &self.cuotas
    }
}