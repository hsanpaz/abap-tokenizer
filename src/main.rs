// abap-tokenizer/src/main.rs
use abap_tokenizer::config::load_toml_config;
use abap_tokenizer::tokenizer::flexible_tokenizer::FlexibleTokenizer;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargar la configuración
    let config_path = Path::new("config/default_config.toml");
    let config = load_toml_config(config_path.to_str().unwrap())?;

    // Código ABAP de ejemplo
    let input = r#"
    REPORT z_test_program.

    DATA: lv_number TYPE i VALUE 10,
          lv_text   TYPE string VALUE 'Hello, ABAP!'.
*   Esto es un comentario de linea.
    WRITE: 'Number:', lv_number.
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


/*fn main() {

    env_logger::init();

    let string1 = String::from("abcd");
    let string2 = "wxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/