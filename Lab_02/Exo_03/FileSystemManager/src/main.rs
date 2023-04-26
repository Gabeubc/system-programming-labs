use std::{os::windows::prelude::MetadataExt, fs::ReadDir, path::Path, fs::{self, DirEntry, read_dir}, ops::{Deref, DerefMut}};



enum FileType {
 Text, 
 Binary
}

impl Default for FileType{
    fn default() -> Self {
        FileType::Text
    }
}

#[derive(Default)]
struct File {
 name: String,
 content: Vec<u8>, // max 1000 bytes, rest of the file truncated
 creation_time: u64,
 type_: FileType,
}

#[derive(Default)]
struct Dir {
 name: String,
 creation_time: u64,
 children: Vec<Node>,
}

enum Node {
 File(File),
 Dir(Dir),
}

struct FileSystem {
 root: Dir
} 


impl FileSystem{

    fn new() -> FileSystem {

        FileSystem { root: Dir::default() }

    }

    fn get_name_from_path(path: &str) -> String{

        let mut tmp = String::default();
        tmp = match path.contains('/') {

            true => {
                 path.split('/').collect::<Vec<&str>>().last().unwrap().to_string()
            },

            _ => {
                for c in path.chars(){
                    if c != '\\' {
                        tmp.push(c)
                    }
                    else {
                        break
                    }
                }
                tmp
            }
            
        };
      //  let mut names: Vec<&str>= path.split('/').collect();
        tmp

    }

    fn from_dir_recursive<'a>(path: &str, vec_node: &mut Vec<Node>) -> (){

        let mut dir = Dir::default();
        let mut file = File::default();

        //should i had a termiation condition or the for is enough? 
        // continue....
        for node in read_dir(path).unwrap(){

            // if dir go call recursive func
            if node.as_ref().unwrap().file_type().unwrap().is_dir(){
            dir.name = FileSystem::get_name_from_path(node.as_ref().unwrap().file_name().to_str().unwrap());
            dir.creation_time = node.as_ref().unwrap().metadata().unwrap().creation_time();
            FileSystem::from_dir_recursive(
                node.as_ref().unwrap().path().to_str().unwrap(),
                 &mut dir.children);
            vec_node.push(Node::Dir(Dir::default()));
        }else{
            // be sure that is a file
            if node.as_ref().unwrap().file_type().unwrap().is_file(){
                file.name = node.as_ref().unwrap().file_name().to_str().unwrap().to_string();
                file.creation_time = node.as_ref().unwrap().metadata().unwrap().creation_time();
                // do action depend of the type of file
                match node.as_ref().unwrap().path().extension().unwrap().to_str().unwrap() {

                  "txt" => file.type_= FileType::Text,
                  "bin" => file.type_= FileType::Binary,
                  _ => println!("file type is not handled")

                } 
            }

        }

        }


    }



    fn from_dir(path: &str) -> FileSystem {

        let mut file_system : FileSystem = FileSystem::new();
        let mut metada = std::fs::metadata(path).unwrap();

        if metada.is_dir(){

            file_system.root.name =  FileSystem::get_name_from_path(path);
            file_system.root.creation_time = metada.creation_time();
            FileSystem::from_dir_recursive(path, &mut file_system.root.children);

        }

        file_system

    }

}


fn main() {
    FileSystem::from_dir("C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/test_folder");
}
