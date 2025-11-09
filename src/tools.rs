
pub mod tools {
    use std::{collections::HashMap};
    use mongodb::bson::Document;
    use inline_colorization::*;

    pub fn zip_vec_array(document:mongodb::sync::Cursor<Document>)-> Vec<(std::string::String, i32,Vec<i32>)>{

        let mut lista_equipamento: HashMap<String, Vec<i32>> = HashMap::new();

        for equipamento in document{
            match equipamento { 
                Ok(value) => {
                    if lista_equipamento.contains_key(value.get("modelo").unwrap().as_str().unwrap()) {
                            
                        if let Some(x) = lista_equipamento.get_mut(value.get("modelo").unwrap().as_str().unwrap()) {
                            x.push(value.get("patrimonio").unwrap().as_str().unwrap().parse::<i32>().expect(format!("{:?}-{:?}",value.get("patrimonio"),value.get("_id")).as_str()));
                         
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
    
        let mut sorted : Vec<(String,i32,Vec<i32>)>=vec!();
        for arr in lista_equipamento {
            sorted.push((arr.0,arr.1.len() as i32,arr.1));
        }
        let _=sorted.sort_by(|a, b| a.0.cmp(&b.0));
        
        sorted

    }

    pub fn unzip_array_equipamentos(document:mongodb::sync::Cursor<Document>)-> Vec<(std::string::String, i32,Vec<i32>)>{

        let mut lista_equipamento: HashMap<String, Vec<i32>> = HashMap::new();

        for equipamento in document{
            match equipamento { 
                Ok(value) => {
                    if lista_equipamento.contains_key(value.get("evento").unwrap().as_str().unwrap()) {
                            
                        if let Some(x) = lista_equipamento.get_mut(value.get("evento").unwrap().as_str().unwrap()) {
                            x.push(value.get("patrimonio").unwrap().as_str().unwrap().parse::<i32>().expect(format!("{:?}-{:?}",value.get("patrimonio"),value.get("_id")).as_str()));
                         
                        }
                    }else{
                        lista_equipamento.insert(value.get("evento").unwrap().as_str().unwrap().to_string(),vec![value.get("patrimonio").unwrap().as_str().unwrap().parse::<i32>().expect("")] );
                     
                    }
                },
                _error => {
                    std::process::Command::new("clear").status().unwrap();
                }
            }
        }
    
        let mut sorted : Vec<(String,i32,Vec<i32>)>=vec!();
        for arr in lista_equipamento {
            sorted.push((arr.0,arr.1.len() as i32,arr.1));
        }
        let _=sorted.sort_by(|a, b| a.0.cmp(&b.0));
        
        sorted
    }

      pub fn logo(opt:&str){
          let red=color_bright_green;
          let red1=color_green;
        let rst=color_reset;

        match opt {
            "logo"=>{
                println!("░▒{red}█▀▀█{rst}░{red}█▀▀{rst}░{red}█▀▀{rst}░{red}▀█▀{rst}░{red}▄▀▀▄{rst}░▒{red}█▀▀▄{rst}");
                println!("░▒{red}█{rst}░{red}▄▄{rst}░{red}█▀▀{rst}░{red}▀▀▄{rst}░░{red}█{rst}░░{red}█{rst}░░{red}█{rst}░▒{red}█▄▄▀{rst}");
                println!("░▒{red1}█▄▄▀{rst}░{red1}▀▀▀{rst}░{red1}▀▀▀{rst}░░{red1}▀{rst}░░░{red1}▀▀{rst}░░▒{red1}█{rst}░▒{red1}█{rst}");
                println!("");
            },
            &_=>{"".to_string();}
            
        }
        
    }

    pub fn text_colorized(text:&str,color:&str,color_b:&str){
        // ░█▀▀▄░█▀▀▄░█▀▄░█▀▄░█▀▀░█▀▀░█▀▀▀░█░░░░░▀░░░░▀░█░▄░█░░█▀▄▀█░█▀▀▄░▄▀▀▄░▄▀▀▄░█▀▀█░█▀▀▄░█▀▀░▀█▀░█░▒█░▄░░░▄░█░█░█░░░█░█░░█░▀▀█
        // ░█▄▄█░█▀▀▄░█░░░█░█░█▀▀░█▀░░█░▀▄░█▀▀█░░█▀░░░█░█▀▄░█░░█░▀░█░█░▒█░█░░█░█▄▄█░█▄▄█░█▄▄▀░▀▀▄░░█░░█░▒█░░█▄█░░▄▀▄░▀▄█▄▀░█▄▄█░▄▀▒
        // ░▀░░▀░▀▀▀▀░▀▀▀░▀▀░░▀▀▀░▀░░░▀▀▀▀░▀░░▀░▀▀▀░█▄█░▀░▀░▀▀░▀░░▒▀░▀░░▀░░▀▀░░█░░░░░░░█░▀░▀▀░▀▀▀░░▀░░░▀▀▀░░░▀░░░▀░▀░░▀░▀░░▄▄▄▀░▀▀▀

        let mut encode1=String::new();
        let mut encode2=String::new();
        let mut encode3=String::new();
        let rst=color_reset;
        let coletor=text.chars().enumerate();
        for (_,letra) in coletor{
            match letra {
                'a'=>{encode1.push_str(format!("░{color}█▀▀▄{rst}").as_str());encode2.push_str(format!("░{color}█▄▄█{rst}").as_str());encode3.push_str(format!("░{color_b}▀{rst}░░{color_b}▀{rst}").as_str());},
                'b'=>{encode1.push_str(format!("░{color}█▀▀▄{rst}").as_str());encode2.push_str(format!("░{color}█▀▀▄{rst}").as_str());encode3.push_str(format!("░{color_b}▀▀▀▀{rst}").as_str());},
                'c'=>{encode1.push_str(format!("░{color}█▀▄{rst}").as_str());encode2.push_str(format!("░{color}█{rst}░░").as_str());encode3.push_str(format!("░{color_b}▀▀▀{rst}").as_str());},
                'd'=>{encode1.push_str(format!("░{color}█▀▄{rst}").as_str());encode2.push_str(format!("░{color}█{rst}░{color}█{rst}").as_str());encode3.push_str(format!("░{color_b}▀▀{rst}░").as_str());},
                'e'=>{encode1.push_str(format!("░{color}█▀▀{rst}").as_str());encode2.push_str(format!("░{color}█▀▀{rst}").as_str());encode3.push_str(format!("░{color_b}▀▀▀{rst}").as_str());},
                'f'=>{encode1.push_str(format!("░{color}█▀▀{rst}").as_str());encode2.push_str(format!("░{color}█▀{rst}░").as_str());encode3.push_str(format!("░{color_b}▀{rst}░░").as_str());},
                'g'=>{encode1.push_str(format!("░{color}█▀▀▀{rst}").as_str());encode2.push_str(format!("░{color}█{rst}░{color}▀▄{rst}").as_str());encode3.push_str(format!("░{color_b}▀▀▀▀{rst}").as_str());},
                'h'=>{encode1.push_str(format!("░{color}█{rst}░░░").as_str());encode2.push_str(format!("░{color}█▀▀█{rst}").as_str());encode3.push_str(format!("░{color_b}▀{rst}░░{color_b}▀{rst}").as_str());},
                'i'=>{encode1.push_str(format!("░░{color}▀{rst}░").as_str());encode2.push_str(format!("░░{color}█▀{rst}").as_str());encode3.push_str(format!("░{color_b}▀▀▀{rst}").as_str());},
                'j'=>{encode1.push_str(format!("░░░{color}▀{rst}").as_str());encode2.push_str(format!("░░░{color}█{rst}").as_str());encode3.push_str(format!("░{color_b}█▄█{rst}").as_str());},
                'k'=>{encode1.push_str(format!("░{color}█{rst}░{color}▄{rst}").as_str());encode2.push_str(format!("░{color}█▀▄{rst}").as_str());encode3.push_str(format!("░{color_b}▀{rst}░{color_b}▀{rst}").as_str());},
                'l'=>{encode1.push_str(format!("░{color}█{rst}░").as_str());encode2.push_str(format!("░{color}█{rst}░").as_str());encode3.push_str(format!("░{color_b}▀▀{rst}").as_str());},
                'm'=>{encode1.push_str(format!("░{color}█▀▄▀█{rst}").as_str());encode2.push_str(format!("░{color}█{rst}░{color}▀{rst}░{color}█{rst}").as_str());encode3.push_str(format!("░{color_b}▀{rst}░░▒{color_b}▀{rst}").as_str());},
                'n'=>{encode1.push_str(format!("░{color}█▀▀▄{rst}").as_str());encode2.push_str(format!("░{color}█{rst}░▒{color}█{rst}").as_str());encode3.push_str(format!("░{color_b}▀{rst}░░{color_b}▀{rst}").as_str());},
                'o'=>{encode1.push_str(format!("░{color}▄▀▀▄{rst}").as_str());encode2.push_str(format!("░{color}█{rst}░░{color}█{rst}").as_str());encode3.push_str(format!("░░{color_b}▀▀{rst}░").as_str());},
                'p'=>{encode1.push_str(format!("░{color}▄▀▀▄{rst}").as_str());encode2.push_str(format!("░{color}█▄▄█{rst}").as_str());encode3.push_str(format!("░{color_b}█{rst}░░░").as_str());},
                'q'=>{encode1.push_str(format!("░{color}█▀▀█{rst}").as_str());encode2.push_str(format!("░{color}█▄▄█{rst}").as_str());encode3.push_str(format!("░░░░{color_b}█{rst}").as_str());},
                'r'=>{encode1.push_str(format!("░{color}█▀▀▄{rst}").as_str());encode2.push_str(format!("░{color}█▄▄▀{rst}").as_str());encode3.push_str(format!("░{color_b}▀{rst}░{color_b}▀▀{rst}").as_str());},
                's'=>{encode1.push_str(format!("░{color}█▀▀{rst}").as_str());encode2.push_str(format!("░{color}▀▀▄{rst}").as_str());encode3.push_str(format!("░{color_b}▀▀▀{rst}").as_str());},
                't'=>{encode1.push_str(format!("░{color}▀█▀{rst}").as_str());encode2.push_str(format!("░░{color}█{rst}░").as_str());encode3.push_str(format!("░░{color_b}▀{rst}░").as_str());},
                'u'=>{encode1.push_str(format!("░{color}█{rst}░▒{color}█{rst}").as_str());encode2.push_str(format!("░{color}█{rst}░▒{color}█{rst}").as_str());encode3.push_str(format!("░░{color_b}▀▀▀{rst}").as_str());},
                'x'=>{encode1.push_str(format!("░{color}█{rst}░{color}█{rst}").as_str());encode2.push_str(format!("░{color}▄▀▄{rst}").as_str());encode3.push_str(format!("░{color_b}▀{rst}░{color_b}▀{rst}").as_str());},
                'v'=>{encode1.push_str(format!("░{color}▄{rst}░░░{color}▄{rst}").as_str());encode2.push_str(format!("░░{color}█▄█{rst}░").as_str());encode3.push_str(format!("░░░{color_b}▀{rst}░░").as_str());},
                'w'=>{encode1.push_str(format!("░{color}█{rst}░░░{color}█{rst}").as_str());encode2.push_str(format!("░{color}▀▄█▄▀{rst}").as_str());encode3.push_str(format!("░░{color_b}▀{rst}░{color_b}▀{rst}░").as_str());},
                'y'=>{encode1.push_str(format!("░{color}█{rst}░░{color}█{rst}").as_str());encode2.push_str(format!("░{color}█▄▄█{rst}").as_str());encode3.push_str(format!("░{color_b}▄▄▄▀{rst}").as_str());},
                'z'=>{encode1.push_str(format!("░{color}▀▀█{rst}").as_str());encode2.push_str(format!("░{color}▄▀{rst}▒").as_str());encode3.push_str(format!("░{color_b}▀▀▀{rst}").as_str());},
                _=>  {encode1.push_str(format!("░░").as_str());encode2.push_str(format!("░░").as_str());encode3.push_str(format!("░░").as_str());},
                
            }
        }

        println!("{}",encode1);
        println!("{}",encode2);
        println!("{}",encode3);
        println!("")
    }

}





pub struct Preview {
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
        }
        concat

    }
}

pub struct ModeloQtyNow{
    pub modelo:String,
    pub qty:i8
}

impl ModeloQtyNow {
    pub fn new()->ModeloQtyNow{
        ModeloQtyNow { modelo: String::new(), qty: 0 }
    }

    pub fn update(&mut self,modelo:String,qty:i8){
        self.modelo=modelo ;
        self.qty=qty;
    }
}
