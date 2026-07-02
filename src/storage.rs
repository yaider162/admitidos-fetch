use std::{fs::{OpenOptions}, io::BufWriter, io::BufReader};
use std::io::{Seek, SeekFrom};

use crate::student::Student;

pub struct Storage;

impl Storage{
    pub fn save_student(student: Student, program: String) -> Result<(), std::io::Error>{
        //let file = File::create(format!("{}.json",program))?;
        let mut file = OpenOptions::new().read(true)
            .write(true)
            .create(true)
            .open(format!("{}.json",program))?;

        let mut students: Vec<Student> = if file.metadata()?.len() == 0{
            Vec::new()
        }else {                
            let reader = BufReader::new(&file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
        };

        students.push(student);

        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;

        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &students)?;
        Ok(())
    }
}