// abap-tokenizer/src/main.rs
use abap_tokenizer::config::load_toml_config;
use abap_tokenizer::tokenizer::flexible_tokenizer::FlexibleTokenizer;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargar la configuración
    let config_path = Path::new("config/default_config.toml");
    let config = load_toml_config(config_path.to_str().unwrap())?;

    // Código ABAP de ejemplo
    let input = 
r#" REPORT z_test_program. "inicio
*   Esto es un comentario de linea.
    DATA: lv_number TYPE i VALUE 10,
          lv_text   TYPE string VALUE 'Hello, ABAP!'.

    WRITE: 'Number:', lv_number. "Comentario abap entre lineas
    WRITE: / 'Text:', lv_text.

    IF lv_number > 5.
      WRITE: / 'Number is greater than 5'.
    ELSE.
      WRITE: / 'Number is less than or equal to 5'.
    ENDIF.
    "#;

    // Crear el tokenizador
    let mut tokenizer = FlexibleTokenizer::new(input, config);

    // Tokenizar y mostrar los resultados
    println!("Tokens encontrados:");
    while let Ok(Some(token)) = tokenizer.next_token() {
        println!("{:?}", token);
    }

    Ok(())
}