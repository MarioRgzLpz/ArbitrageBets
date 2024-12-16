use crate::{casa_de_apuestas::CasaDeApuestas, cuota::Cuota, evento::Resultados};
use std::collections::HashMap;

/// La clave de `cuotas` es un String representando el nombre de la casa de apuestas, y el String de `evento` representa el nombre del evento
pub struct ApuestaSegura {
    evento: String,
    cuotas: HashMap<String, Cuota>,
}

impl ApuestaSegura {
    const NUMERO_INVERSA: f64 = 1.0;
    pub fn new(evento: String, cuotas: HashMap<String, Cuota>) -> Self {
        ApuestaSegura { evento, cuotas }
    }
    fn obtener_mejores_cuotas(
        evento: &str,
        casas: &[CasaDeApuestas],
    ) -> HashMap<Resultados, (String, f64)> {
        let mut mejores_cuotas: HashMap<Resultados, (String, f64)> = HashMap::new();

        for casa in casas {
            if let Some(cuotas_evento) = casa.get_cuotas().get(evento) {
                for cuota in cuotas_evento {
                    let resultado = cuota.get_resultado();
                    let valor = cuota.get_valor();

                    if let Some((_, mejor_valor)) = mejores_cuotas.get(resultado) {
                        if valor > *mejor_valor {
                            mejores_cuotas
                                .insert(resultado.clone(), (casa.get_nombre().clone(), valor));
                        }
                    } else {
                        mejores_cuotas
                            .insert(resultado.clone(), (casa.get_nombre().clone(), valor));
                    }
                }
            }
        }

        mejores_cuotas
    }

    fn es_apuesta_segura(mejores_cuotas: &HashMap<Resultados, (String, f64)>) -> bool {
        let valores_cuotas: Vec<f64> = mejores_cuotas.values().map(|(_, valor)| *valor).collect();
        let inversas: Vec<f64> = valores_cuotas.iter().map(|valor| Self::NUMERO_INVERSA / valor).collect();
        let suma_inversa: f64 = inversas.iter().sum();
        suma_inversa < Self::NUMERO_INVERSA
    }

    pub fn calcular_apuestas_seguras(
        evento: &str,
        casas: Vec<CasaDeApuestas>,
    ) -> Option<ApuestaSegura> {

        let mejores_cuotas = Self::obtener_mejores_cuotas(evento, &casas);

        if Self::es_apuesta_segura(&mejores_cuotas) {
            let cuotas: HashMap<String, Cuota> = mejores_cuotas
                .into_iter()
                .map(|(resultado, (casa, valor))| {
                    (casa.clone(), Cuota::new(resultado.clone(), valor))
                })
                .collect();

            Some(ApuestaSegura::new(evento.to_string(), cuotas))
        } else {
            None
        }
    }

    fn leer_json(ruta_archivo: &str) -> Vec<CasaDeApuestas> {
        use std::fs;
    
        let contenido = fs::read_to_string(ruta_archivo)
            .expect(&format!("No se pudo leer el archivo: {}", ruta_archivo));
        
        serde_json::from_str(&contenido)
            .expect("Error al parsear el JSON")
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{evento::Resultados, cuota::Cuota, casa_de_apuestas::CasaDeApuestas};
    use std::collections::HashMap;

    #[test]
    fn test_obtener_mejores_cuotas() {
        let casas_de_apuestas = ApuestaSegura::leer_json("data/test_1.json");

        let mejores_cuotas = ApuestaSegura::obtener_mejores_cuotas("Real Madrid vs Barcelona", &casas_de_apuestas);

        assert_eq!(mejores_cuotas.len(), 3);
        assert_eq!(mejores_cuotas[&Resultados::GanaLocal].1, 3.0);
        assert_eq!(mejores_cuotas[&Resultados::Empate].1, 3.5);
        assert_eq!(mejores_cuotas[&Resultados::GanaVisitante].1, 3.2);
    }

    #[test]
    fn test_es_apuesta_segura() {
        let mejores_cuotas = HashMap::from([
            (Resultados::GanaLocal, ("Betfair".to_string(), 2.9)),
            (Resultados::Empate, ("Bet365".to_string(), 3.5)),
            (Resultados::GanaVisitante, ("Betfair".to_string(), 3.2)),
        ]);

        assert!(ApuestaSegura::es_apuesta_segura(&mejores_cuotas));
    }

    #[test]
    fn test_calcular_apuestas_seguras() {
        let casas_de_apuestas = ApuestaSegura::leer_json("data/test_1.json");

        let resultado = ApuestaSegura::calcular_apuestas_seguras("Real Madrid vs Barcelona", casas_de_apuestas);

        assert!(resultado.is_some());
    }

    #[test]
    fn test_calcular_apuestas_seguras_sin_apuesta_segura() {
        let casas_de_apuestas = ApuestaSegura::leer_json("data/test_2.json");

        let resultado = ApuestaSegura::calcular_apuestas_seguras("Real Madrid vs Barcelona", casas_de_apuestas);

        assert!(resultado.is_none());
    }
}
