use std::io::{self, Write, stdin, stdout};

pub fn get_models() -> io::Result<Vec<String>> {
    let mut model: Vec<String> = Vec::new();

    print!("Please enter car model: "); stdout().flush()?;
    
    let mut modelstr = String::new();
    stdin().read_line(&mut modelstr)?;

    let modelstr = modelstr.trim().to_string();
    model.push(modelstr);

    Ok(model)
}

pub fn list_models(model: &Vec<String>) -> io::Result<()> {
    for i in model {
        println!("- {}", i); 
    }

    Ok(())
}