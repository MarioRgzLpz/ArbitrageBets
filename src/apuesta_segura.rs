use crate::{casa_de_apuestas::CasaDeApuestas, cuota::Cuota, evento::Resultados};
use std::collections::HashMap;

/// La clave de `cuotas` es un String representando el nombre de la casa de apuestas, y el String de `evento` representa el nombre del evento
pub struct ApuestaSegura {
    evento: String,
    cuotas: HashMap<String, Cuota>,
}

impl ApuestaSegura {
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
        let inversas: Vec<f64> = valores_cuotas.iter().map(|valor| 1.0 / valor).collect();
        let suma_inversa: f64 = inversas.iter().sum();
        suma_inversa < 1.0
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{evento::Resultados, cuota::Cuota, casa_de_apuestas::CasaDeApuestas};
    use std::collections::HashMap;

    #[test]
    fn test_obtener_mejores_cuotas() {
        let cuotas_casa1 = vec![
            Cuota::new(Resultados::GanaLocal, 2.1),
            Cuota::new(Resultados::Empate, 3.5),
            Cuota::new(Resultados::GanaVisitante, 3.0),
        ];

        let cuotas_casa2 = vec![
            Cuota::new(Resultados::GanaLocal, 2.2),
            Cuota::new(Resultados::Empate, 3.3),
            Cuota::new(Resultados::GanaVisitante, 2.8),
        ];

        let mut cuotas_map1 = HashMap::new();
        cuotas_map1.insert("BarcavsMadrid".to_string(), cuotas_casa1);

        let mut cuotas_map2 = HashMap::new();
        cuotas_map2.insert("BarcavsMadrid".to_string(), cuotas_casa2);

        let casa1 = CasaDeApuestas::new("Casa1".to_string(), cuotas_map1);
        let casa2 = CasaDeApuestas::new("Casa2".to_string(), cuotas_map2);

        let mejores_cuotas =
            ApuestaSegura::obtener_mejores_cuotas("BarcavsMadrid", &[casa1, casa2]);

        assert_eq!(mejores_cuotas.len(), 3);
        assert_eq!(mejores_cuotas[&Resultados::GanaLocal].1, 2.2);
        assert_eq!(mejores_cuotas[&Resultados::Empate].1, 3.5);
        assert_eq!(mejores_cuotas[&Resultados::GanaVisitante].1, 3.0);
    }

    #[test]
    fn test_es_apuesta_segura() {
        let mut mejores_cuotas = HashMap::new();
        mejores_cuotas.insert(Resultados::GanaLocal, ("Casa1".to_string(), 3.2));
        mejores_cuotas.insert(Resultados::Empate, ("Casa2".to_string(), 3.5));
        mejores_cuotas.insert(Resultados::GanaVisitante, ("Casa1".to_string(), 3.0));

        assert!(ApuestaSegura::es_apuesta_segura(&mejores_cuotas));

        mejores_cuotas.insert(Resultados::GanaVisitante, ("Casa1".to_string(), 1.5));
        assert!(!ApuestaSegura::es_apuesta_segura(&mejores_cuotas));
    }

    #[test]
    fn test_calcular_apuestas_seguras() {
        let cuotas_casa1 = vec![
            Cuota::new(Resultados::GanaLocal, 3.1),
            Cuota::new(Resultados::Empate, 3.5),
            Cuota::new(Resultados::GanaVisitante, 3.0),
        ];

        let cuotas_casa2 = vec![
            Cuota::new(Resultados::GanaLocal, 2.2),
            Cuota::new(Resultados::Empate, 3.3),
            Cuota::new(Resultados::GanaVisitante, 2.8),
        ];

        let mut cuotas_map1 = HashMap::new();
        cuotas_map1.insert("BarcavsMadrid".to_string(), cuotas_casa1);

        let mut cuotas_map2 = HashMap::new();
        cuotas_map2.insert("BarcavsMadrid".to_string(), cuotas_casa2);

        let casa1 = CasaDeApuestas::new("Casa1".to_string(), cuotas_map1);
        let casa2 = CasaDeApuestas::new("Casa2".to_string(), cuotas_map2);

        let resultado = ApuestaSegura::calcular_apuestas_seguras("BarcavsMadrid", vec![casa1, casa2]);

        assert!(resultado.is_some());
    }
}
