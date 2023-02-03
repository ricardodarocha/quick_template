use std::error::Error;
use handlebars::Handlebars;
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let (
            arg_template,
            arg_json,
            out_destino ) = (
            std::env::args().nth(1).expect("Informe o caminho do template <TEMPLATE>"), 
            std::env::args().nth(2).expect("Informe o caminho dos dados <JSON>"), 
            std::env::args().nth(3).expect("Informe o arquivo de destino <DESTINO>")
        );

    //1. Read the template into strings
    let template = std::fs::read_to_string(&arg_template).unwrap();
    
    //2. Read the Json
    let json = {
        let text = std::fs::read_to_string(&arg_json).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    
    //3. Starts the Handlebars
    let handlebars = Handlebars::new();
    let render = handlebars.render_template(&template, &json)?;

    use std::fs::File;
    use std::io::{BufWriter, Write};

    let f = File::create(&out_destino).expect("Não foi possível criar o arquivo");
    let mut f = BufWriter::new(f);
    f.write_all(render.as_bytes()).expect("Não foi possível escrever no arquivo");
    print!("Arquivo finalizado com sucesso");
    

    Ok(())
}