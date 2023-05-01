use core::{fmt, panic};
use std::{
    any::type_name,
    fmt::Debug,
    fs::{self, read_dir, DirEntry},
    fs::{DirBuilder, ReadDir},
    io::{BufReader, Read},
    ops::{Add, Deref, DerefMut},
    os::windows::prelude::MetadataExt,
    path::Path,
    slice::Iter, ptr::drop_in_place,
};

#[derive(Debug)]
enum FileType {
    Text,
    Binary,
}

impl Default for FileType {
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
    root: Dir,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            root: Dir::default(),
        }
    }

    fn read_content_of_file(content: &mut Vec<u8>, path: &str) -> () {
        let mut file = std::fs::File::options().read(true).open(path).unwrap();
        let mut buf: &mut [u8] = Default::default();
        let mut tmp = u8::default();
        let mut tmp_vec = ['c' as u8; 100];
        let mut file_size = std::fs::metadata(path).unwrap().file_size();
        if file_size < 100 {
            unsafe {
                unsafe {
                    buf = std::slice::from_raw_parts_mut(tmp_vec.as_mut_ptr(), file_size as usize);
                    file.read_exact(buf).unwrap();
                    for c in tmp_vec.iter() {
                        content.push(*c)
                    }
                }
            }
        } else {
            for i in 0..100 {
                unsafe {
                    buf = std::slice::from_raw_parts_mut(&mut tmp, std::mem::size_of::<u8>());
                    file.read_exact(buf).unwrap();
                    content.push(tmp);
                }
            }
        }
    }

    fn from_dir_recursive<'a>(path: &str, vec_node: &'a mut Vec<Node>) -> () {
        //should i had a termiation condition or the for is enough?
        // continue....
        for node in read_dir(path).unwrap() {
            let mut dir = Dir::default();
            let mut file = File::default();
            // if dir go call recursive func
            if node.as_ref().unwrap().file_type().unwrap().is_dir() {
                dir.name = node
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string();
                dir.creation_time = node.as_ref().unwrap().metadata().unwrap().creation_time();
                FileSystem::from_dir_recursive(
                    node.as_ref().unwrap().path().to_str().unwrap(),
                    &mut dir.children,
                );
                vec_node.push(Node::Dir(dir));
            } else {
                // be sure that is a file
                if node.as_ref().unwrap().file_type().unwrap().is_file() {
                    file.name = node
                        .as_ref()
                        .unwrap()
                        .file_name()
                        .to_str()
                        .unwrap()
                        .to_string();
                    file.creation_time = node.as_ref().unwrap().metadata().unwrap().creation_time();
                    FileSystem::read_content_of_file(
                        &mut file.content,
                        node.as_ref().unwrap().path().to_str().unwrap(),
                    );
                    // do action depend of the type of file
                    match node
                        .as_ref()
                        .unwrap()
                        .path()
                        .extension()
                        .unwrap()
                        .to_str()
                        .unwrap()
                    {
                        "txt" => file.type_ = FileType::Text,
                        "bin" => file.type_ = FileType::Binary,
                        _ => println!("file type is not handled"),
                    }
                    vec_node.push(Node::File(file));
                }
            }
        }
    }

    fn from_dir(path: &str) -> FileSystem {
        let mut file_system: FileSystem = FileSystem::new();
        let mut metada = std::fs::metadata(path).unwrap();

        if metada.is_dir() {
            file_system.root.name = Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            file_system.root.creation_time = metada.creation_time();
            FileSystem::from_dir_recursive(path, &mut file_system.root.children);
        }

        file_system
    }
/* 
    fn mk_dir(path: &str) -> Dir {
        let mut dir = Dir::default();
        let mut dir_builder = DirBuilder::new().create(path);
        let mut tmp_path = String::default();
        match dir_builder {
            Ok(ok) => {
                dir.name = path.split('/').last().unwrap().to_string();
                dir.creation_time = u64::default();
            }
            Err(err) => {
                let mut vec_node_path: Vec<&str> = path.split('/').collect();
                vec_node_path.iter().for_each(|x| {
                    tmp_path.push_str(x);
                    /*println!("{}", tmp_path);
                    match Path::new(tmp_path.as_str()).is_dir() {
                        true => print!(""),
                        false => panic!("{} is an invalid path", tmp_path),
                    }*/
                    tmp_path.push('/');
                });
            }
        }
        dir
    }
    */

    // search root
    fn check_for_valid_path(
        child_iter: &mut Vec<Node>,
        node_path: &mut Vec<&str>,
        result: &mut bool,
        cursor: &mut usize,
        option: usize,
    ) -> () {
        let mut flag = false;
        for node in child_iter {
            match node {
                Node::Dir(d) => {
                    if d.name != node_path.get(0).unwrap().to_string() {
                        FileSystem::check_for_valid_path(
                            &mut d.children,
                            node_path,
                            result,
                            cursor,
                            option,
                        );
                    } else {
                        *cursor = 1;
                        let mut sub_dirs_names= Vec::<String>::new();
                        d.children.iter().for_each(
                            |x| match x {
                                Node::Dir(d) => sub_dirs_names.push(d.name.clone()),
                                Node::File(f) => {}
                            }
                        );
                        // handle option 2 with path <=2
                        if option == 2 && sub_dirs_names.contains(&node_path.last().unwrap().to_string()) && node_path.len() == 2{
                            let mut index = -1;
                            d.children.iter().enumerate().for_each(|(i, x)|
                        match x {
                            Node::Dir(d) => {
                                if d.name == node_path.last().unwrap().to_string() && d.children.is_empty(){
                                    index = i as i32;
                                    *result = true;
                                }
                            },
                            Node::File(f) => {}
                            
                        }
                        );
                        if index >= 0{
                            d.children.remove(index as usize);
                        }
                        flag = true;
                        }
                        if flag != true{
                            FileSystem::is_valid_path_recursive(
                                &mut d.children,
                                node_path,
                                &mut sub_dirs_names,
                                cursor,
                                result,
                                option,
                            );
                        }
                        
                        if *result != false {
                            if node_path.len() > *cursor  {
                                *result = false;
                            } else if node_path.len() == *cursor {
                                *result = true;
                            }
                        }
                    }
                }
                Node::File(f) => {}
            }
        }
    }

    //since root is know, check path
    fn is_valid_path_recursive(
        child_iter: &mut Vec<Node>,
        node_path: &mut Vec<&str>,
        sub_dir_names: &mut Vec<String>,
        cursor: &mut usize,
        result: &mut bool,
        option: usize,
    ) -> () {
        for node in child_iter {
            match node {
                Node::Dir(d) => {

                    if *cursor < node_path.len() {
                        if !sub_dir_names.contains(&node_path.get(*cursor).unwrap().to_string()) {
                            *result = false;
                        } 
                        else {
                            match option {
                                
                                // mk_dir
                                1 => {
                                    if node_path.len() - 2 == *cursor && d.name == node_path.get(node_path.len() - 2).unwrap().to_string(){
                                        let mut new_dir = Dir::default();
                                        new_dir.name = node_path.get(*cursor + 1).unwrap().to_string();
                                        d.children.push(Node::Dir(new_dir));
                                        *result = true;
                                    }
                                }
                                // rm_dir
                                2 => {
                                    if *cursor == node_path.len() -2 {
                                        let mut index = -1;
                            d.children.iter().enumerate().for_each(|(i, x)|
                        match x {
                            Node::Dir(d) => {
                                if d.name == node_path.last().unwrap().to_string(){
                                    index = i as i32;
                                    *result = true;
                                }
                            },
                            Node::File(f) => {}
                            
                        }
                        );
                        d.children.remove(index as usize);
                                    }  

                                }

                                _ => {}
                            }
                        }
                    }
                    
                    *cursor += 1;
                    sub_dir_names.clear();
                    d.children.iter().for_each(
                        |x| match x {
                            Node::Dir(d) => sub_dir_names.push(d.name.clone()),
                            Node::File(f) => {}
                        }
                    );
                    FileSystem::is_valid_path_recursive(
                        &mut d.children,
                        node_path,
                        sub_dir_names,
                        cursor,
                        result,
                        option,
                    );
                    
                }
                Node::File(f) => {}
            }
        }
    }

    fn is_path_valid(&mut self, path: &str, option: usize) -> bool {
        let mut node_path: Vec<&str> = path.split('/').collect();
        let mut sub_dir_names = Vec::<String>::new();
        let mut result = true;
        let mut cursor: usize = 0;
        if node_path.len() == 0 {
            return true;
        }
        if node_path.get(0).unwrap().to_string() == self.root.name {
            cursor = 1;
            FileSystem::is_valid_path_recursive(
                &mut self.root.children,
                &mut node_path,
                &mut sub_dir_names,
                &mut cursor,
                &mut result,
                option,
            );
        } else {
            FileSystem::check_for_valid_path(
                &mut self.root.children,
                &mut node_path,
                &mut result,
                &mut cursor,
                option,
            );
        }

        return result;
    }

    fn rm_dir(&mut self, path: &str) -> () {
        self.is_path_valid(path, 2);
        println!("***********rm_dir****************");
        println!("dir {} have been removed",path);
        println!("***********rm_dir end****************");
    }

    fn mk_dir(&mut self, path:&str) -> (){
        self.is_path_valid(path, 1);
        println!("***********mk_dir****************");
        println!("dir {} have been added",path);
        println!("***********mk_dir end****************");
    }
}

fn main() {
    let path = "C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_03/FileSystemManager/src/resources/parent_folder";
    let mut file_system = FileSystem::from_dir(path);
    println!("***********File System Copy****************");
    println!("{:?}", file_system);
    println!("***********File System Copy****************");
    //let result = file_system.is_path_valid("child_folder/empty_child_folder", 2);
    /*println!("{}", result);*/
    file_system.rm_dir("child_folder/empty_child_folder");
    println!("***********File System Copy****************");
    println!("{:?}", file_system);
    println!("***********File System Copy****************");
}
