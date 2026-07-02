use std::collections::HashMap;
use reqwest::blocking::Client;

use scraper::{Html, Selector};

mod student;
mod storage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let target_url = "https://registro.uptc.edu.co/listaProgramasResultados.jsp";
    
    let response = client.get(target_url).send()?.text()?;
    let document = Html::parse_document(&response);

    let row_selector = Selector::parse("tr.bordes_tabla").unwrap();
    let id_selector = Selector::parse("span.cajas").unwrap();
    let name_selector = Selector::parse("strong").unwrap();

    let mut programas: HashMap<String, String> = HashMap::new();

    for row in document.select(&row_selector) {
        let id = row
            .select(&id_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        let nombre = row
            .select(&name_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        programas.insert(id, nombre);
    }

    for (id, name) in programas{
        save_by_id(id, name.clone(), &client)?;
        println!("Guardado el {}.json", &name);
    }
    Ok(())
}

fn save_by_id(id: String, name: String, client:&Client) -> Result<(), Box<dyn std::error::Error>>{
    let url = format!("https://registro.uptc.edu.co/admitidosprograma.jsp?nroprog={}&nomprog={}", id, name);
    let response= client.get(url).send()?.text()?;
    let document  = Html::parse_document(&response);
    
    let row_selector = Selector::parse("tr.cajasc").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    for row in document.select(&row_selector){
        let cells: Vec<_> = row.select(&cell_selector).collect();

        if cells.len()==10{
            let student = student::Student{
                id: cells[0].text().collect::<Vec<_>>().join("").trim().to_string(),
                name: cells[2].text().collect::<Vec<_>>().join("").trim().to_string(),
                pun_uptc: cells[4].text().collect::<Vec<_>>().join("").trim().to_string(),
                factors: vec![cells[5].text().collect::<Vec<_>>().join("").trim().to_string(), 
                              cells[6].text().collect::<Vec<_>>().join("").trim().to_string(),
                              cells[7].text().collect::<Vec<_>>().join("").trim().to_string(),
                              cells[8].text().collect::<Vec<_>>().join("").trim().to_string()],
                admitido: cells[9].text().collect::<Vec<_>>().join("").trim().to_string().eq("ADMITIDO"),
                program: name.clone()
            };
            storage::Storage::save_student(student, name.clone())?;
            //println!("{:#?}",student);
        }
    }
    Ok(())
}