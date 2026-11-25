/// Configuración mínima leída de un texto con líneas `clave=valor`.
pub struct Config {
    pub nombre: String,
    pub nivel: u32,
}

/// Lee una configuración desde un texto. Devuelve error si está vacía
/// o si el nivel no es un número.
pub fn parse_config(texto: &String) -> Result<Config, String> {
    let lineas: Vec<&str> = texto.lines().collect();
    if lineas.len() == 0 {
        return Err("configuración vacía".to_string());
    }
    let mut nombre = String::new();
    let mut nivel = 0;
    for linea in lineas {
        if let Some((clave, valor)) = linea.split_once('=') {
            match clave.trim() {
                "nombre" => nombre = valor.trim().to_string(),
                "nivel" => {
                    nivel = valor
                        .trim()
                        .parse()
                        .map_err(|_| "nivel inválido".to_string())?
                }
                _ => {}
            }
        }
    }
    Ok(Config { nombre, nivel })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lee_nombre_y_nivel() {
        let texto = String::from("nombre=demo\nnivel=3");
        let config = parse_config(&texto).unwrap();
        assert_eq!(config.nombre, "demo");
        assert_eq!(config.nivel, 3);
    }
}
