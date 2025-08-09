use std::collections::{ HashMap, HashSet};
// use std::thread::sleep;
// use std::time::Duration;
use std::io::{stdout, Write};
use termimad::crossterm::{
    cursor::{ Hide, Show},
    event::{
        self,
        Event,
        KeyEvent,
        KeyCode::*,
    },
    queue,
    terminal::{
        self,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    style::Color::*,
};
use termimad::*;

use std::{thread, time};
use std::io::{stdin};

use std::{fs, path::Path};
use std::env::{self};

use mongodb::{ bson::{doc, Document}, sync::{Client, Collection}};
use inquire::{Text,error::InquireError, Select, validator::{ Validation}};
use inline_colorization::*;
#[macro_use]
extern crate goto;
// use genpdf::{elements, Element};

///mods

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column
    area
}
fn run_app(skin: MadSkin ,md :&str) -> Result<(), Error> {
    let mut w = stdout(); // we could also have used stderr
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor
    let mut view = MadView::from(md.to_owned(), view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent{code, ..})) => {
                match code {
                    Up => view.try_scroll_lines(-1),
                    Down => view.try_scroll_lines(1),
                    PageUp => view.try_scroll_pages(-1),
                    PageDown => view.try_scroll_pages(1),
                    _ => break,
                }
            }
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}

fn zip_document_array(document:mongodb::sync::Cursor<Document>)->HashMap<std::string::String, Vec<i32>>{
    
    let mut lista_equipamento: HashMap<String, Vec<i32>> = HashMap::new();

    for equipamento in document{
        match equipamento { 
            Ok(value) => {
                if lista_equipamento.contains_key(value.get("modelo").unwrap().as_str().unwrap()) {
                        
                    if let Some(x) = lista_equipamento.get_mut(value.get("modelo").unwrap().as_str().unwrap()) {
                        x.push(value.get("patrimonio").unwrap().as_str().unwrap().parse::<i32>().expect("errou carra"));
                    }
                    
                }else{
                    lista_equipamento.insert(value.get("modelo").unwrap().as_str().unwrap().to_string(),vec![value.get("patrimonio").unwrap().as_str().unwrap().parse::<i32>().expect("")] );
                }
            },
            _error => {
                std::process::Command::new("clear").status().unwrap();
            }
        }
    }
    lista_equipamento
}


struct Preview {
    arr:Vec<String>
}

impl Preview {
    pub fn new()->Preview{
        Preview { arr: vec![] }
    }
    pub fn add_preview(&mut self,element:String){
        if !self.arr.contains(&element){
            self.arr.push(element);
        }
    }
    pub fn display_preview(&self){
        for i in self.arr.clone(){
            println!("{}",i);
        }
    }
    pub fn _display_preview_str(&self)->String{
        let mut concat: String=String::new();
        for i in self.arr.clone(){
            concat.push_str(format!("{} \n",i).as_str());
            // println!("{}",i);
        }
        concat

    }
}
////Saida
fn saida(coll: Collection<Document>)-> Result<(), Box<dyn std::error::Error>>{
    
    let mut prev: Preview=Preview::new();
    let all = coll.find(doc! {}).run()?;    
    let mut eventos: Vec<String> = Vec::new();
    eventos.push("NOVO EVENTO".to_string());
    eventos.sort();
    
    let resultadio = all.filter_map(|s| {
        s.ok().filter(|f| f.get("evento").unwrap().as_str().unwrap() != "deposito")
    });
    
    for result in resultadio {
        eventos.push(result.get("evento").unwrap().as_str().unwrap().to_string());
    }
    
    let lista_usuarios=vec!("Claudio".to_string(),"Eyler".to_string(),"Dourado".to_string());
    let select_usuario: Result<String, InquireError> = Select::new("",lista_usuarios.into_iter().collect() )
    .with_help_message(" ↑↓ ESCOLHA O EVENTO!!")
    .with_page_size(40)
    .prompt()
    .inspect_err(|_f| {
        let _=main();
    });
    
    let lista_eventos: HashSet<String> = eventos.into_iter().collect();
    let select_evento: Result<String, InquireError> = Select::new("", lista_eventos.into_iter().collect())
    .with_help_message(" ↑↓ ESCOLHA O EVENTO!!")
    .with_page_size(40)
    .prompt()
    .inspect_err(|_f| {
        let _=main();
    });
    let validator = |input: &str| if input.chars().count() < 1 {
        std::process::Command::new("clear").status().unwrap();
        let _ =main();
        Ok(Validation::Invalid("nao encontrado".into()))
    } else {
        Ok(Validation::Valid)
    };
    match select_evento {
        Ok(choice) =>
        {
            let event_d: Result<_,_> =  match choice.as_str(){
                "NOVO EVENTO"=>{ Text::new("Evento?").with_page_size(40).with_validator(validator).prompt().inspect_err(|_f| {
                    let _=main();
                })},
                "EXIT" => std::process::exit(0x0100),
                _ => Ok(choice)
            };
            
            loop {
                std::process::Command::new("clear").status().unwrap();
                prev.display_preview();
                let query = Text::new("Patrimonio?").with_page_size(40).with_validator(validator)
                .prompt();
                
                match query {
                    Ok(query) => {
                         let cursor = coll.find_one_and_update(doc! { "patrimonio":&query },doc! {"$set": doc! {"evento": &event_d.as_ref().unwrap(),"user":&select_usuario.as_ref().unwrap()}}).run()?;
                        match &cursor {
                            Some(doc)=>{
                                // println!(r"modelo:{color_cyan}{}{color_reset}", doc.get("modelo").unwrap().as_str().unwrap());
                                prev.add_preview(format!(r"{color_green}{}{color_reset} -- {}",doc.get("patrimonio").unwrap().as_str().unwrap().to_string(),doc.get("modelo").unwrap().as_str().unwrap().to_string()));

                            },
                            None =>{
                                prev.add_preview(format!(r"{} --{color_red}Nao cadastrado!!!{color_reset}",&query));
                                // println!(r"modelo:{color_red}Nao cadastrado!!!{color_reset}");
                                // sleep(Duration::from_secs(2));
                            }
                        }
                    },
                    _error =>{
                        std::process::Command::new("clear").status().unwrap();
                        let _ =main();
                    }
                }
            }
        },
        Err(_) => println!("")

    }
Ok(())
}

////Entrada
fn entrada(coll: Collection<Document>)-> Result<(), Box<dyn std::error::Error>>{
    let validator = |input: &str| if input.chars().count() < 1 {
        std::process::Command::new("clear").status().unwrap();
        let _ =main();
        Ok(Validation::Invalid("nao encontrado".into()))
        
    } else {
        Ok(Validation::Valid)
    };
    
    
    let mut status = true;
    
    while status {
        
        let query = Text::new("Patrimonio?").with_page_size(40).with_validator(validator)
        .prompt();
    
    match query {
        Ok(query) => {
            let cursor = coll.find_one_and_update(doc! { "patrimonio":&query },doc! {"$set": doc! {"evento": "deposito"}}).run()?;
            
            match &cursor {
                Some(doc)=>{
                    println!(r"modelo:{color_cyan}{}{color_reset}", doc.get("modelo").unwrap().as_str().unwrap());
                },
                None =>{
                    println!(r"modelo:{color_red}Nao cadastrado!!!{color_reset}");
                }
            } 
        },
        _error =>{
            status=false;
            std::process::Command::new("clear").status().unwrap();
            let _=main();
        }
    }
}
Ok(())
}

////Procura
fn procurar(coll :&Collection<Document> ) -> Result<(), Box<dyn std::error::Error>>{
    let validator = |input: &str| if input.chars().count() < 1 {
        std::process::Command::new("clear").status().unwrap();
        let _ =main();
        Ok(Validation::Invalid("nao encontrado Procurar".into()))
    } else {
        Ok(Validation::Valid)
    };
    let all = coll.find(doc! {}).run()?;
  
    let mut modelo: Vec<String> = Vec::new();
    modelo.push("patrimonio".to_string());

    for result in all {
        let doc =result?;
        modelo.push(doc.get("modelo").unwrap().as_str().unwrap().to_string());
    }
    let lista_modelo: HashSet<String> = modelo.into_iter().collect();
    let select_modelo: Result<String, InquireError> = Select::new("", lista_modelo.into_iter().collect())
    .with_help_message(" ↑↓ Patrimonio ou Modelo!!")
    .with_page_size(40)
    .prompt()
    .inspect_err(|_f| {
        let _=main();
    });

    match select_modelo {
        Ok(choice )=>{
            match choice.as_str() {
                    "patrimonio" =>{
                        gpoint!['begin:
                            let query = Text::new("Patrimonio/modelo?").with_page_size(40).with_validator(validator)
                                .prompt()
                                .inspect_err(|_f| {
                                    let _=main();
                                });
                            let cursor = coll.find(doc! { "patrimonio":&query.unwrap() }).run()?;

                            for result in cursor {
                                    match result {
                                        Ok(doc) => {
                                                
                                            std::process::Command::new("clear").status().unwrap();
                                            {
                                                match doc.get("grupo").unwrap().as_null() {
                                                        Some(_a) =>  println!(r"grupo:{color_green}...{color_reset}"),
                                                        _ => println!(r"grupo:{color_green}{}{color_reset}",doc.get("grupo").unwrap().as_str().unwrap())
                                                    } 
                                            }
                                            println!(r"modelo:{color_cyan}{}{color_reset}", doc.get("modelo").unwrap().as_str().unwrap());
                                            println!(r"patrimonio:{color_blue}{}{color_reset}",doc.get("patrimonio").unwrap().as_str().unwrap());
                                            println!(r"evento:{color_red}{}{color_reset} data:{color_red}{}{color_reset}user:{color_red}{}{color_reset}",doc.get("evento").unwrap().as_str().unwrap(),doc.get("data").unwrap().as_str().unwrap(),doc.get("user").unwrap().as_str().unwrap());
                                            {
                                                if doc.get("ultimoevento").is_some(){
                                        
                                                    println!(r"ultimo evento:{color_red}{}{color_reset}",doc.get("ultimoevento").unwrap().as_str().unwrap());
                                                }else{
                                                        println!(r"ultimo evento:{color_red}-----{color_reset}");
                                                    }
                                            }
                                            println!(r"info:{color_red}{}{color_reset}",doc.get("info").unwrap().as_str().unwrap());
                                            println!(r"{color_red}-------------------------------------------------------------{color_reset}");
                                            let mut buffer = String::new();
                                            stdin().read_line(&mut buffer)?;
                                            std::process::Command::new("clear").status().unwrap();
                                            continue 'begin;     

                                        },
                                        Err(_err)=> {
                                        }
                                    }
                            }
                            std::process::Command::new("clear").status().unwrap();
                            continue 'begin;

                        ]
                    },
                    _=>{
                        let cursor = coll.find(doc! { "modelo":{ "$regex": &choice.as_str() } }).run()?;
            
                        for result in cursor {
                            
                            match result {
                                Ok(value) => {
                                    match value {
                                        doc => {
                                            {
                                                match doc.get("grupo").unwrap().as_null() {
                                                    Some(_a) =>  println!(r"grupo:{color_green}...{color_reset}"),
                                                    _ => println!(r"grupo:{color_green}{}{color_reset}",doc.get("grupo").unwrap().as_str().unwrap())
                                                }  
                                            }
                                            println!(r"modelo:{color_cyan}{}{color_reset}", doc.get("modelo").unwrap().as_str().unwrap());
                                            println!(r"patrimonio:{color_blue}{}{color_reset}",doc.get("patrimonio").unwrap().as_str().unwrap());
                                            println!(r"evento:{color_red}{}{color_reset}",doc.get("evento").unwrap().as_str().unwrap());
                                            println!(r"info:{color_red}{}{color_reset}",doc.get("info").unwrap().as_str().unwrap());
                                            println!(r"{color_red}-------------------------------------------------------------{color_reset}");
                                        }
                                    }
                                },
                                _error => {
                                    std::process::Command::new("clear").status().unwrap();
                                    let _ =procurar(&coll);
                                }
                            }
                        }
                        let mut buffer = String::new();
                        stdin().read_line(&mut buffer)?;
                        std::process::Command::new("clear").status().unwrap();

                        let _ =procurar(&coll);
                    }
            }
        },
        Err(err)=>{
            println!("{}",err)

        }
    }

    Ok(())
}

//cadastrar
fn cadastrar (coll:Collection<Document>  ) -> Result<(), Box<dyn std::error::Error>>{

    // if let MyOption::Some(value) = none_value {
    //     println!("Found a value: {}", value); // This will not print
    // } else {
    //     println!("No value found."); // This will print
    // }
    
    let select_grupo  = Select::new("", vec!("IMAGEM".to_string(),"AUDIO".to_string(),"ENERGIA".to_string(),"COMUNICACAO".to_string()))
    .with_help_message(" ↑↓ ESCOLHA O GRUPO!!")
    .with_page_size(40)
    .prompt()
    .inspect_err(|_f| {
        let _=main();
    });

    let all= coll.find(doc! {"grupo":select_grupo.as_ref().unwrap().to_string()}).run()?;   
    let mut modelo: Vec<String> = Vec::new();
    modelo.push("NOVO".to_string());
    modelo.sort();

    
    let validator = |input: &str| if input.chars().count() < 1 {
            std::process::Command::new("clear").status().unwrap();
            let _ =main();
            Ok(Validation::Invalid("Invalido".into()))
            
        } else {
        Ok(Validation::Valid)
    };
     
    for result  in all {
    let doc = result?;
    modelo.push(doc.get("modelo").unwrap().as_str().unwrap().to_string());
    }

     let lista_modelo: HashSet<String> = modelo.into_iter().collect();
    
    std::process::Command::new("clear").status().unwrap();

    // let renderconfig=RenderConfig { 
    //     prompt_prefix: Styled::new("______________________________"),
    //         answered_prompt_prefix: Styled::new(""),
    //         prompt: StyleSheet::empty(),
    //         default_value: StyleSheet::empty(),
    //         placeholder: StyleSheet::empty(),
    //         help_message: StyleSheet::empty(),
    //         text_input: StyleSheet::empty(),
    //         error_message: ErrorMessageRenderConfig::empty(),
    //         answer: StyleSheet::empty(),
    //         canceled_prompt_indicator: Styled::new(""),
    //         password_mask: '*',
    //         highlighted_option_prefix: Styled::new(">"),
    //         scroll_up_prefix: Styled::new("↑"),
    //         scroll_down_prefix: Styled::new("↓"),
    //         selected_checkbox: Styled::new("[x]"),
    //         unselected_checkbox: Styled::new("[ ]"),
    //         option_index_prefix: IndexPrefix::None,
    //         option: StyleSheet::empty(),
    //         selected_option: Some(StyleSheet { fg: Some(Color::LightGreen), bg: Some(Color::Rgb { r: 0, g: 0, b: 0 }), att: Attributes::BOLD }),

    //         #[cfg(feature = "date")]
    //         calendar: calendar::CalendarRenderConfig::empty(),

    //         #[cfg(feature = "editor")]
    //         editor_prompt: StyleSheet::empty(),
    // };

    let select_modelo: Result<String, InquireError> = Select::new("", lista_modelo.into_iter().collect())
    // .with_render_config(renderconfig)
    .with_help_message(" ↑↓ ESCOLHA O MODELO!!")
    .with_page_size(40)
    .prompt()
    .inspect_err(|_f| {
        let _=cadastrar(coll.clone());
    });

    match select_modelo {
        Ok(modelo)=>{
            let event_e: Result<_,_> =  match modelo.as_str(){
                "NOVO"=>{ Text::new("Novo Modelo?").with_validator(validator).prompt().inspect_err(|_f| {
                    let _=main();
                })},
                "EXIT" => std::process::exit(0x0100),
                _ => Ok(modelo.clone())
            };

            loop {
                
                let query = Text::new("Patrimonio?").with_page_size(40).with_validator(validator)
                .prompt();
                
                match query {
                    Ok(query) => {


                        let cursor = coll.insert_one(doc!{"grupo":select_grupo.as_ref().unwrap().to_string(),"evento": "deposito","info":"","modelo":event_e.as_ref().unwrap(),"patrimonio":&query}).run();
                       
                       match cursor {
                           
                           Ok(_sucesso) =>{
                                // std::process::Command::new("clear").status().unwrap();
                                println!(r"{color_yellow} {} {color_reset} {} {color_green}  --->Cadastrado{color_reset}",&query,event_e.as_ref().unwrap());
        
                           },
                           Err(_errro)=>{
                                // std::process::Command::new("clear").status().unwrap();
                                println!(r"modelo:{color_red}Ja Cadastrado{color_reset}");
                           }
                       }
                    },
                    _error =>{
                        let _ =main();
                    }
                }
            }
        },
        Err(_error) => {
            // let _=main();
            Ok(())
        },
    }
}

//deletar
fn deletar(coll:Collection<Document> )-> Result<(), Box<dyn std::error::Error>>{

    let validator = |input: &str| if input.chars().count() < 1 {
            std::process::Command::new("clear").status().unwrap();
            let _ =main();
            Ok(Validation::Invalid("Invalido".into()))
            
        } else {
        Ok(Validation::Valid)
    };
    
    let patrimonio = Text::new("Patrimonio?").with_page_size(40).with_validator(validator).prompt();

    match patrimonio {

        Ok(result)=>{
            coll.delete_one(doc!{"patrimonio":result}).run()?;
            println!("Deletado!!!");
            let ten_millis = time::Duration::from_millis(3000);
            thread::sleep(ten_millis);
            std::process::Command::new("clear").status().unwrap();
            let _=main();
        },
        Err(_erro)=>{
            println!("Nada Deletado!!!");
            let _=main();
        }
    }

    Ok(())
}

//infor

fn info(coll:Collection<Document> )-> Result<(), Box<dyn std::error::Error>>{

    let validator = |input: &str| if input.chars().count() < 1 {
            std::process::Command::new("clear").status().unwrap();
            let _ =main();
            Ok(Validation::Invalid("Invalido".into()))
            
        } else {
        Ok(Validation::Valid)
    };
    
    let patrimonio = Text::new("Patrimonio?")
        .with_page_size(40)
        .with_validator(validator)
        .prompt()
        .inspect_err(|_f| {
            let _=main();
        });

    let informacao = Text::new("Informacao?").with_page_size(40).with_validator(validator).prompt();

    match patrimonio {
            Ok(query) => {
                 let cursor = coll.find_one_and_update(doc! { "patrimonio":&query },doc! {"$set": doc! {"info": informacao.unwrap()}}).run()?;
                
                match &cursor {
                    Some(_doc)=>{
                        std::process::Command::new("clear").status().unwrap();
                        println!(r"{color_green}Informacoes atualizadas{color_reset}", );
                        let ten_millis = time::Duration::from_millis(3000);
                        thread::sleep(ten_millis);
                        std::process::Command::new("clear").status().unwrap();
                        let _=main();
                    },
                    None =>{
                        std::process::Command::new("clear").status().unwrap();
                        println!(r"{color_red}Nada alterado!!!{color_reset}");
                        let ten_millis = time::Duration::from_millis(3000);
                        thread::sleep(ten_millis);
                        std::process::Command::new("clear").status().unwrap();
                        let _=main();
                    }
                } 
            },
            _error =>{
                let _ = main();
            }
    }
    
    Ok(())
}

// fn imprimir_pdf(coll:Collection<Document> )->Result<(),Box<dyn std::error::Error>>{

//     let font_family = genpdf::fonts::from_files("./fonts", "Sansation", None).expect("Failed to load font family");
//     // Create a document and set the default font family
//     let mut doc = genpdf::Document::new(font_family);
//     // Change the default settings
//     doc.set_title("Demo document");
//     // Customize the pages
//     let mut decorator = genpdf::SimplePageDecorator::new();
//     decorator.set_margins(10);
//     doc.set_page_decorator(decorator);
//     // Add one or more elements
//     doc.push(genpdf::elements::Paragraph::new("This is a demo document."));
//     // Render the document and write it to a file
//     doc.render_to_file("output.pdf").expect("Failed to write PDF file");
//     Ok(())
// }

fn imprimir(coll:Collection<Document> )-> Result<(), Box<dyn std::error::Error>>{
    let skin = make_skin();
    
    let font_family = genpdf::fonts::from_files("./fonts", "Sansation", None).expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Acap locacao");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    


    let validator = |input: &str| if input.chars().count() < 1 {
        std::process::Command::new("clear").status().unwrap();
        let _ =main();
        Ok(Validation::Invalid("Invalido".into()))
        
    } else {
        Ok(Validation::Valid)
    };

    let mut modelo: Vec<String> = Vec::new();
    modelo.push("Modelo/Patrimonio".to_string());
    modelo.sort();

    let all=coll.find(doc!{}).run()?;

    for result  in all {
        let doc = result?;
        
        modelo.push(doc.get("evento").unwrap().as_str().unwrap().to_string());
    }
    
    let lista_modelo: HashSet<String> = modelo.into_iter().collect();

    let select_modelo: Result<String, InquireError> = Select::new("", lista_modelo.into_iter().collect()).with_help_message(" ↑↓ ESCOLHA O EVENTO!!").with_page_size(40).prompt();

    match select_modelo {
        Ok(evento)=>{
            match evento.as_str() {
                "Modelo/Patrimonio"=>{
                    let query = Text::new("Modelo/Patrimonio?").with_page_size(40).with_validator(validator).prompt();
                    let result=coll.find(doc! {"patrimonio":{ "$regex": query.unwrap()}}).run()?;
                    for equipamento in result{
                        match equipamento {
                            Ok(sucesso)=>{
                                println!("{:?}",sucesso);
                            },
                            Err(_err)=>{}
                        }
                    }
                },
                _=>{
                    let result: mongodb::sync::Cursor<Document>=coll.find(doc! {"evento":evento.clone()}).run()?;

                    
                    let paths=format!("./{}.txt",evento);
                   
                    let path: &Path=Path::new(&paths);
                   
                        let _=fs::remove_file(&path);
                        let constent=fs::read_to_string(&path);


                        match constent {
                            Ok(mut _arquivo)=>{
                                std::process::Command::new("clear").status().unwrap();
                                let _=main();

                            },
                            Err(_erro)=>{
                                let mut gravar=String::new();

                                for (k,v) in zip_document_array(result).iter(){
                                    gravar.push_str(format!("{color_green}{}{color_reset} | {color_cyan}{}{color_reset} \n", {
                                        if v.len() < 10 {format!("0{}",v.len())}else{format!("{}",v.len())}
                                    },{
                                        format!("{}{}",k," ".repeat(60-k.len()))
                                    }).as_str());
                                    //v
                                    doc.push(genpdf::elements::Paragraph::new(format!("{} | {} | {:?}\n", {
                                        if v.len() < 10 {format!("0{}",v.len())}else{format!("{}",v.len())}
                                    },{
                                        format!("{}{}",k," ".repeat(60-k.len()))
                                    },v).as_str()));
                                }
                                
                                let _=run_app(skin.to_owned() ,gravar.as_str());
                                //
                                let _=fs::write(path,gravar); 
                                
                                // doc.render_to_file(format!("{evento}.pdf")).expect("Failed to write PDF file");
                              
                                std::process::Command::new("clear").status().unwrap();

                                let _ =main();
                            }
                        }           
                    }
                }

            },
            Err(_err)=>{
                let _=main();
            }
        }
             
    Ok(())
}
fn main() -> mongodb::error::Result<()> {
    
    std::process::Command::new("clear").status().unwrap();
    let client = Client::with_uri_str(env::args().nth(1).unwrap())?; 

    
    let my_coll: Collection<Document> = client.database("deposito2025").collection("audio_visual");

    
   
    let options: Vec<&str> = vec![
        "Procurar",
    	// "Previas",
    	"Entrada",
    	"Saida",
    	"Imprimir",
    	// "Renomear",
        //"OS_Ativas",
    	//"Grupos",
    	//"Grupos/Modelos",
    	"Cadastrar",
    	"Info",
    	"Deletar",
    	//"Geral",
    	"EXIT",
    ];
    
    let ans: Result<&str, InquireError> = Select::new("", options).with_help_message(" ↑↓ USE AS SETAS PARA ESCOLHER!!").with_page_size(40).prompt();


    match ans {
        Ok(choice) =>
        {
            match choice{
                "Procurar"=>{let _= procurar(&my_coll);},
                "Cadastrar"=>{let _= cadastrar(my_coll);},
                "Entrada"=>{let _= entrada(my_coll);},
                "Saida"=>{let _= saida(my_coll);},
                "Imprimir"=>{let _=imprimir(my_coll);},
                "Deletar"=>{let _= deletar(my_coll);},
                "Info"=>{let _= info(my_coll);},
                "EXIT"=>{std::process::exit(0x0100);},
                &_ => std::process::exit(0x0100),
            }
        },
        Err(_) => {
            println!("Selecione uma opcao");
            let _=main();
        },
    }
    Ok(())
}