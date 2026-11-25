use rusters_mcp_demo::config::parse_config;

fn main() {
    let texto = String::from("nombre=demo\nnivel=3");
    match parse_config(&texto) {
        Ok(config) => println!("{} nivel {}", config.nombre, config.nivel),
        Err(error) => eprintln!("error: {error}"),
    }
}
