use std::collections::{ HashSet};
use std::ops::Not;
use printers::{ get_default_printer,common::base::job::PrinterJobOptions};

use std::{fs,thread, time};
use std::io::{stdin};

use std::env::{self};

use mongodb::{ bson::{doc, Document}, sync::{Client, Collection}};
use inquire::{Text,error::InquireError, Select,Confirm, validator::{ Validation}};
use inline_colorization::*;

mod tools;
use crate::tools::{tools::text_colorized,tools::logo,tools::zip_vec_array,tools::unzip_array_equipamentos,ModeloQtyNow,Preview};

#[macro_use]
extern crate goto;


////Saida
fn saida(coll: Collection<Document>)-> Result<(), Box<dyn std::error::Error>>{
    std::process::Command::new("clear").status().unwrap();
    text_colorized(" saida ",color_bright_green,color_green);
    let mut modelo_qty=ModeloQtyNow::new();
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
                text_colorized("saida", color_bright_blue, color_blue);
                prev.display_preview();

                println!();
                println!("QUANTIDADE:{color_green}{}{color_reset} ---> {color_cyan}{}{color_reset}",modelo_qty.qty,modelo_qty.modelo);
                
                println!("Evento:{color_yellow}{}{color_reset}",&event_d.as_ref().unwrap());
                println!();

                let query = Text::new("Patrimonio?").with_page_size(40)
                .prompt();
            
            match query {
                Ok(query) => {
                    let cursor = coll.find_one_and_update(doc! { "patrimonio":&query },doc! {"$set": doc! {"evento": &event_d.as_ref().unwrap(),"user":&select_usuario.as_ref().unwrap()}}).run()?;

                        match &cursor {
                            Some(doc)=>{
                                // println!(r"modelo:{color_cyan}{}{color_reset}", doc.get("modelo").unwrap().as_str().unwrap());
                                prev.add_preview(format!(r"{color_green}{}{color_reset} -- {}",doc.get("patrimonio").unwrap().as_str().unwrap().to_string(),doc.get("modelo").unwrap().as_str().unwrap().to_string()));
                                let qty_modelo=coll.find(doc! {
                                    "$and": [
                                        doc! { "modelo": { "$eq": doc.get("modelo").unwrap().as_str().unwrap().to_string() }},
                                        doc! { "evento": { "$eq": &event_d.as_ref().unwrap() }}
                                    ]
                                }).run()?;

                                modelo_qty.update(doc.get("modelo").unwrap().as_str().unwrap().to_string(), qty_modelo.count() as i8);
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
    let mut prev: Preview=Preview::new();
    // let validator = |input: &str| if input.chars().count() < 1 {
    //     std::process::Command::new("clear").status().unwrap();
    //     let _ =main();
    //     Ok(Validation::Invalid("nao encontrado".into()))
        
    // } else {
    //     Ok(Validation::Valid)
    // };
    
    
    let mut status = true;
    
    while status {
        std::process::Command::new("clear").status().unwrap();
       text_colorized("entrada",color_bright_green,color_green);
        prev.display_preview();
        println!();
        // .with_validator(validator)
        let query = Text::new("Patrimonio?")
        .with_page_size(40)
        .prompt();
    
    match query{
        Ok(query) => {
            if query.is_empty().not(){
                
                let cursor = coll.find_one_and_update(doc! { "patrimonio":&query },doc! {"$set": doc! {"evento": "deposito"}}).run()?;
                
                match &cursor {
                    Some(doc)=>{
                        prev.add_preview(format!(r"{color_green}{}{color_reset} {color_yellow} {}{color_reset} {color_green} {}{color_reset}",doc.get("patrimonio").unwrap().as_str().unwrap().to_string(),doc.get("evento").unwrap().as_str().unwrap().to_string(),doc.get("modelo").unwrap().as_str().unwrap().to_string()));
                        // println!(r"modelo:{color_cyan}{}{color_reset}", doc.get("modelo").unwrap().as_str().unwrap());
    
                    },
                    None =>{
                        prev.add_preview(format!(r"{} --{color_red}Nao cadastrado!!!{color_reset}",&query));
                        // println!(r"modelo:{color_red}Nao cadastrado!!!{color_reset}");
                    }
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
    std::process::Command::new("clear").status().unwrap();
    text_colorized("procurar",color_bright_green,color_green);


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
                            let query = Text::new("Patrimonio/modelo?").with_page_size(40)
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
                                            println!(r"evento:{color_red}{}{color_reset} usuario:{color_red}{}{color_reset}",doc.get("evento").unwrap().as_str().unwrap(),doc.get("user").unwrap().as_str().unwrap());
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
                                            let _=main();
                                        }
                                    }
                            }
                            std::process::Command::new("clear").status().unwrap();
                            continue 'begin;

                        ]
                    },
                    _=>{

                        println!();
                        let cursor = coll.find(doc! { "modelo":{ "$regex": &choice.as_str() } }).run()?;
            
                        let mut exibir=String::new();
                        
                        for (k,v,patri) in zip_vec_array(cursor).iter(){
                            exibir.push_str(format!("{color_green}{}{color_reset} | {color_cyan}{}{color_reset} \n{color_yellow}{:?}{color_reset}\n \n", {
                                if v < &10 {format!("0{}",v)}else{format!("{}",v)}
                            },{
                                format!("{}{}",k," ".repeat(60-k.len()))
                            },patri).as_str());
                            
                        }
                        println!("{}",exibir);

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

//equipamento
fn equipamento(coll:Collection<Document>)-> Result<(), Box<dyn std::error::Error>>{
    std::process::Command::new("clear").status().unwrap();
    text_colorized("equipamento",color_bright_green,color_green);
    let all = coll.find(doc! {}).run()?;
    let mut modelo: Vec<String> = Vec::new();

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
    let cursor = coll.find(doc! { "modelo": select_modelo.ok().unwrap().as_str() }).run()?;
    
    let mut exibir=String::new();

      for (k,v,patri) in unzip_array_equipamentos(cursor).iter(){
                            exibir.push_str(format!("{color_green}{}{color_reset} | {color_cyan}{}{color_reset} \n{color_yellow}{:?}{color_reset}\n \n", {
                                if v < &10 {format!("0{}",v)}else{format!("{}",v)}
                            },{
                                format!("{}{}",k," ".repeat(60-k.len()))
                            },patri).as_str());
                            
                        }
                        println!("{}",exibir);

                        let mut buffer = String::new();
                        stdin().read_line(&mut buffer)?;
                        std::process::Command::new("clear").status().unwrap();

                        let _ =equipamento(coll);

    Ok(())
}


//cadastrar
fn cadastrar (coll:Collection<Document>  ) -> Result<(), Box<dyn std::error::Error>>{
    std::process::Command::new("clear").status().unwrap();
    text_colorized("cadastrar",color_bright_green,color_green);
    let select_grupo  = Select::new("", vec!("IMAGEM".to_string(),"AUDIO".to_string(),"ENERGIA".to_string(),"COMUNICACAO".to_string(),"FERRAMENTAS".to_string()))
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


     
    for result  in all {
    let doc = result?;
    modelo.push(doc.get("modelo").unwrap().as_str().unwrap().to_string());
    }

     let lista_modelo: HashSet<String> = modelo.into_iter().collect();
    
    std::process::Command::new("clear").status().unwrap();


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
                "NOVO"=>{ Text::new("Novo Modelo?").prompt().inspect_err(|_f| {
                    let _=main();
                })},
                "EXIT" => std::process::exit(0x0100),
                _ => Ok(modelo.clone())
            };

            loop {
                
                let query = Text::new("Patrimonio?").with_page_size(40)
                .prompt();
                
                match query {
                    Ok(query) => {

                        if query.is_empty().not(){

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
                        }else{
                            let _=main();
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
    std::process::Command::new("clear").status().unwrap();
    text_colorized("deletar",color_bright_green,color_green);

    
    let patrimonio = Text::new("Patrimonio?").with_page_size(40).prompt();

    match patrimonio {

        Ok(result)=>{
            if result.is_empty().not(){
                coll.delete_one(doc!{"patrimonio":result}).run()?;
                println!("Deletado!!!");
                let ten_millis = time::Duration::from_millis(3000);
                thread::sleep(ten_millis);
                std::process::Command::new("clear").status().unwrap();
                let _=main();  
            }else{
                std::process::Command::new("clear").status().unwrap();
                let _=main();  
            }
        },
        Err(_erro)=>{
            println!("Nada Deletado!!!");
            let _=main();
        }
    }

    Ok(())
}

//RENOMEAR

fn renomear(coll:Collection<Document> )-> Result<(), Box<dyn std::error::Error>>{
    std::process::Command::new("clear").status().unwrap();
    text_colorized("renomear",color_bright_green,color_green);

    
    let informacao = Text::new("Novo nome:").with_page_size(40).prompt();



    	loop{
    		
		    let patrimonio = Text::new("Patrimonio?")
		        .with_page_size(40)
		        .prompt()
		        .inspect_err(|_f| {
		            let _=main();
		        });


		    match patrimonio {
		            Ok(query) => {
                        if query.is_empty().not(){
                            let cursor = coll.find_one_and_update(doc! { "patrimonio":&query },doc! {"$set": doc! {"modelo": informacao.as_ref().unwrap()}}).run()?;
                           
                           match &cursor {
                               Some(_doc)=>{
                                   std::process::Command::new("clear").status().unwrap();
                                   println!(r"{color_green}Nome  atualizado{color_reset}", );
                                   let ten_millis = time::Duration::from_millis(1000);
                                   thread::sleep(ten_millis);
                                   std::process::Command::new("clear").status().unwrap();
                                   
                               },
                               None =>{
                                   std::process::Command::new("clear").status().unwrap();
                                   println!(r"{color_red}Nada alterado!!!{color_reset}");
                                   let ten_millis = time::Duration::from_millis(1000);
                                   thread::sleep(ten_millis);
                                   std::process::Command::new("clear").status().unwrap();
                                   let _=main();
                               }
                           } 

                        }else{
                            let _=main();
                        }
		            },
		            _error =>{
		                let _ = main();
		            }
		    }
    	}
     
    
    
    // Ok(())
}


//infor

fn info(coll:Collection<Document> )-> Result<(), Box<dyn std::error::Error>>{
    std::process::Command::new("clear").status().unwrap();
    text_colorized("informacoes",color_bright_green,color_green);
    
    let patrimonio = Text::new("Patrimonio?")
        .with_page_size(40)
        .prompt()
        .inspect_err(|_f| {
            let _=main();
        });
   
    let informacao = Text::new("Informacao?").with_page_size(40).prompt();

    match patrimonio {
            Ok(query) => {
                if query.is_empty().not(){

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
                }else{
                    let _ = main();
                }
            },
            _error =>{
                let _ = main();
            }
    }
    
    Ok(())
}


fn imprimir(coll:Collection<Document> )-> Result<(), Box<dyn std::error::Error>>{
    std::process::Command::new("clear").status().unwrap();
    text_colorized("imprimir",color_bright_green,color_green);

  

    let mut modelo: Vec<String> = Vec::new();
    // modelo.push("Modelo/Patrimonio".to_string());
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
                    let query = Text::new("Modelo/Patrimonio?").with_page_size(40).prompt();
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

                    
                    // zip_vec_array (result);
                                let mut exibir=String::new();
                                let mut gravar=String::new();

                                for (k,v ,patri) in zip_vec_array(result).iter(){
                                    exibir.push_str(format!("{color_green}{}{color_reset} | {color_cyan}{}{color_reset} \n{color_yellow}{}{color_reset}\n", {
                                        if v < &10 {format!("0{}",v)}else{format!("{}",v)}
                                    },{
                                        format!("{}{}",k," ".repeat(60-k.len()))
                                    },patri.into_iter().map(|i| format!("{}, ",i.to_string())).collect::<String>()).as_str());
                                    //v
                                    gravar.push_str(format!("{} | {} \n", {
                                        if v < &10 {format!("0{}",v)}else{format!("{}",v)}
                                    },{
                                        format!("{}{}",k," ".repeat(60-k.len()))
                                    }).as_str());
                                }

                                println!("{}",exibir);
                                
                                //
                                let imprimir=Confirm::new("Imprimir")
                                    .with_default(false)
                                    .prompt().inspect_err(|_f| {
                                        let _=imprimir(coll);
                                    }).ok().unwrap();

                                if imprimir == true {

                                    let default_printer = get_default_printer();
                                    if default_printer.is_some() {
                                        let _job_id = default_printer.unwrap().print(gravar.as_bytes(), PrinterJobOptions {
                                            name: None,
                                            raw_properties: &[
                                                ("document-format", "application/vnd.cups-raw"),
                                                ("copies", "1"),
                                            ],
                                        });
                                    }
                                }
                                let _=fs::write(format!("/home/pokety/files/{}",evento.clone()), gravar);
                                
                                std::process::Command::new("clear").status().unwrap();

                                let _ =main();   

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
   
    {
        logo("logo");

    }
    let client = Client::with_uri_str(env::args().nth(1).unwrap())?; 
    
    
    let my_coll: Collection<Document> = client.database("deposito2025").collection("audio_visual");
    
    
    
    let options: Vec<&str> = vec![
        "Procurar",
        "Equipamento",
    	// "Previas",
    	"Entrada",
    	"Saida",
    	"Imprimir",
    	"Renomear",
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
                "Equipamento"=>{let _= equipamento(my_coll);},
                "Cadastrar"=>{let _= cadastrar(my_coll);},
                "Entrada"=>{let _= entrada(my_coll);},
                "Saida"=>{let _= saida(my_coll);},
                "Imprimir"=>{let _=imprimir(my_coll);},
                "Renomear"=>{let _=renomear(my_coll);},
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
