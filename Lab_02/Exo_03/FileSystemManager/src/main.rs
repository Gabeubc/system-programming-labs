use core::{fmt, panic};
use std::{os::windows::prelude::MetadataExt, fs::{ReadDir, DirBuilder}, path::Path, fs::{self, DirEntry, read_dir}, ops::{Deref, DerefMut, Add}, io::{BufReader, Read}, fmt::Debug};


#[derive(Debug)]
enum FileType {
 Text, 
 Binary
}

impl Default for FileType{
    fn default() -> Self {
        FileType::Text
    }
}

#[derive(Default, Debug)]
struct File {
 name: String,
 content: Vec<u8>, // max 1000 bytes, rest of the file truncated
 creation_time: u64,
 type_: FileType,
}




#[derive(Default, Debug)]
struct Dir {
 name: String,
 creation_time: u64,
 children: Vec<Node>,
}
#[derive(Debug)]
enum Node {
 File(File),
 Dir(Dir),
}
#[derive(Debug)]
struct FileSystem {
 root: Dir
} 


impl FileSystem{

    fn new() -> FileSystem {

        FileSystem { root: Dir::default() }

    }
    
    fn read_content_of_file(content: &mut Vec<u8>, path: &str) -> (){

        let mut file = std::fs::File::options().read(true).open(path).unwrap();
        let mut buf: &mut [u8] = Default::default();
        let mut tmp = u8::default();
        let mut tmp_vec= ['c' as u8; 100];
        let mut file_size = std::fs::metadata(path).unwrap().file_size();
        if file_size < 100{
            unsafe{
                unsafe{
                    buf = std::slice::from_raw_parts_mut(tmp_vec.as_mut_ptr(), 
                        file_size as usize);
                    file.read_exact(buf).unwrap();
                    for c in tmp_vec.iter(){
                        content.push(*c)
                    }
                }
            }
        }
        else{

                
            for i in 0..100{
                unsafe{
                    buf = std::slice::from_raw_parts_mut(&mut tmp, 
                        std::mem::size_of::<u8>());
                    file.read_exact(buf).unwrap();
                    content.push(tmp);
                }
            
        }
            
        }

    }

    fn from_dir_recursive<'a>(path: &str, vec_node: &'a mut Vec<Node>) -> (){

        //should i had a termiation condition or the for is enough? 
        // continue....
        for node in read_dir(path).unwrap(){
            
        let mut dir = Dir::default();
        let mut file = File::default();
            // if dir go call recursive func
            if node.as_ref().unwrap().file_type().unwrap().is_dir(){
                dir.name = node.as_ref().unwrap().file_name().to_str().unwrap().to_string();
                dir.creation_time = node.as_ref().unwrap().metadata().unwrap().creation_time();
                FileSystem::from_dir_recursive(
                    node.as_ref().unwrap().path().to_str().unwrap(),
                &mut dir.children);
                vec_node.push(Node::Dir(dir));
        }else{
            // be sure that is a file
            if node.as_ref().unwrap().file_type().unwrap().is_file(){
                file.name = node.as_ref().unwrap().file_name().to_str().unwrap().to_string();
                file.creation_time = node.as_ref().unwrap().metadata().unwrap().creation_time() ;
                FileSystem::read_content_of_file(&mut file.content, node.as_ref().unwrap().path().to_str().unwrap());
                // do action depend of the type of file
                match node.as_ref().unwrap().path().extension().unwrap().to_str().unwrap() {

                  "txt" => file.type_= FileType::Text,
                  "bin" => file.type_= FileType::Binary,
                  _ => println!("file type is not handled")

                }
                vec_node.push(Node::File(file)); 
            }

        }

        }


    }



    fn from_dir(path: &str) -> FileSystem {

        let mut file_system : FileSystem = FileSystem::new();
        let mut metada = std::fs::metadata(path).unwrap();

        if metada.is_dir(){

            file_system.root.name = Path::new(path).file_name().unwrap().to_str().unwrap().to_string();
            file_system.root.creation_time = metada.creation_time() ;
            FileSystem::from_dir_recursive(path, &mut file_system.root.children);

        }

        file_system

    }

    fn mk_dir (path: &str) -> Dir{

        let mut dir = Dir::default();
        let mut dir_builder = DirBuilder::new().create(path);
        let mut tmp_path = String::default();
        match dir_builder{
            Ok(ok) => {
                dir.name = path.split('/').last().unwrap().to_string();
                dir.creation_time = u64::default();
            },
            Err(err) => {
                let mut vec_node_path: Vec<&str> = path.split('/').collect();
                vec_node_path.iter().for_each(|x|
                {   
                    tmp_path.push_str(x);
                    println!("{}",tmp_path);
                    match Path::new(tmp_path.as_str()).is_dir() {
                        true => print!(""),
                        false => panic!("{} is an invalid path", tmp_path)
                        
                    }
                    tmp_path.push('/');
                }
            );
            }
        } 
        dir
    }

}


fn main() {
  let path = "C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_03/FileSystemManager/src/resources/parent_folder";
  let file_system =  FileSystem::from_dir(path);
  println!("{:?}", file_system);
  FileSystem::mk_dir("C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_03/FileSystemManager/src/parent_folder/gg");
}
