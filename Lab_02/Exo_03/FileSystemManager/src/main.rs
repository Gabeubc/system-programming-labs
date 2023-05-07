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
    ptr::drop_in_place,
    slice::Iter,
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

    fn mk_dir_on_resource(path: &str) -> () {
        let mut dir_builder = DirBuilder::new().create(path);
    }

    fn rm_dir_on_resource(path: &str) -> () {
        let mut rm_dir = std::fs::remove_dir(path);
    }

    fn rm_file_on_resource(path: &str) -> () {
        let mut rm_dir = std::fs::remove_file(path);
    }

    fn mk_file_on_resource(path: &str) -> () {
        let mut mk_file = std::fs::File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(path);
    }

    fn get_file_from_resource(path: &str) -> std::fs::File {
        let mut file = std::fs::File::options()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        file
    }

    //since root is know, check path
    fn option_parser_recursive(
        child_iter: &mut Vec<Node>,
        node_path: &mut Vec<&str>,
        root_path: &str,
        sub_dir_names: &mut Vec<String>,
        cursor: &mut usize,
        result: &mut bool,
        option: usize,
    ) -> () {
        for node in child_iter {
            match node {
                Node::Dir(d) => {
                    if *cursor < node_path.len() {
                        if !sub_dir_names.contains(&d.name) {
                            *result = false;
                        } else {
                            match option {
                                // mk_dir
                                1 => {
                                    if node_path.len() - 2 == *cursor
                                        && d.name
                                            == node_path
                                                .get(node_path.len() - 2)
                                                .unwrap()
                                                .to_string()
                                    {
                                        let mut new_dir = Dir::default();
                                        let mut flag = true;
                                        new_dir.name = node_path.last().unwrap().to_string();
                                        d.children.iter().for_each(|x| match x {
                                            Node::Dir(d) => {
                                                if d.name == new_dir.name {
                                                    flag = false;
                                                }
                                            }
                                            Node::File(f) => {}
                                        });
                                        if flag {
                                            d.children.push(Node::Dir(new_dir));
                                            FileSystem::mk_dir_on_resource(&root_path);
                                            *result = true;
                                        } else {
                                            *result = false;
                                        }
                                    }
                                }
                                // rm_dir
                                2 => {
                                    if *cursor == node_path.len() - 2 {
                                        let mut index = -1;
                                        d.children.iter().enumerate().for_each(|(i, x)| match x {
                                            Node::Dir(d) => {
                                                if d.name == node_path.last().unwrap().to_string() {
                                                    index = i as i32;
                                                }
                                            }
                                            Node::File(f) => {}
                                        });
                                        if index >= 0 {
                                            d.children.remove(index as usize);
                                            FileSystem::rm_dir_on_resource(&root_path);
                                            *result = true;
                                        }
                                    }
                                }

                                3 => {
                                    if node_path.len() - 2 == *cursor
                                        && d.name
                                            == node_path
                                                .get(node_path.len() - 2)
                                                .unwrap()
                                                .to_string()
                                    {
                                        let mut new_file = File::default();
                                        let mut flag = true;
                                        new_file.name = node_path.last().unwrap().to_string();
                                        d.children.iter().for_each(|x| match x {
                                            Node::Dir(d) => {}
                                            Node::File(f) => {
                                                if f.name == new_file.name {
                                                    flag = false;
                                                }
                                            }
                                        });
                                        if flag {
                                            d.children.push(Node::File(new_file));
                                            FileSystem::mk_file_on_resource(&root_path);
                                            *result = true;
                                        } else {
                                            *result = false;
                                        }
                                    }
                                }

                                4 => {
                                    if *cursor == node_path.len() - 2 {
                                        let mut index = -1;
                                        d.children.iter().enumerate().for_each(|(i, x)| match x {
                                            Node::File(f) => {
                                                if f.name == node_path.last().unwrap().to_string() {
                                                    index = i as i32;
                                                }
                                            }
                                            Node::Dir(d) => {}
                                        });
                                        if index >= 0 {
                                            d.children.remove(index as usize);
                                            FileSystem::rm_file_on_resource(&root_path);
                                            *result = true;
                                        } else {
                                            *result = false;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    *cursor += 1;
                    // sub_dir_names.clear();
                    d.children.iter().for_each(|x| match x {
                        Node::Dir(d) => sub_dir_names.push(d.name.clone()),
                        Node::File(f) => {}
                    });
                    FileSystem::option_parser_recursive(
                        &mut d.children,
                        node_path,
                        root_path,
                        sub_dir_names,
                        cursor,
                        result,
                        option,
                    );
                }
                Node::File(f) => {
                    if f.name == node_path.get(*cursor).unwrap().to_string()
                        && *cursor == node_path.len() - 1
                        && option == 0
                    {
                        *result = true;
                    }
                }
            }
        }
        if *cursor > 0 {
            *cursor -= 1;
        }
    }

    fn is_path_valid(&mut self, path: &str, option: usize) -> bool {
        let mut node_path: Vec<&str> = path.split('/').collect();
        let mut sub_dir_names = Vec::<String>::new();
        let mut result = false;
        let mut cursor: usize = 0;
        let mut root_name = "C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_03/FileSystemManager/src/resources/".to_owned();
        /*sroot_name = root_name + self.root.name.as_str();
        root_name = root_name + "/";*/
        if node_path.get(0).unwrap().to_string() != self.root.name {
            root_name = root_name + self.root.name.as_str();
            root_name = root_name + "/";
            root_name = root_name + path;
        } else {
            root_name = root_name + path;
        }
        if node_path.contains(&self.root.name.as_str()) {
            node_path.remove(0);
        }
        if node_path.len() == 0 {
            return true;
        }
        if node_path.len() == 1 {
            match option {
                0 => {
                    let mut flag = false;
                    let file_to_find = node_path.last().unwrap().to_string();
                    self.root.children.iter().for_each(|x| match x {
                        Node::Dir(d) => {}
                        Node::File(f) => {
                            if f.name == file_to_find {
                                flag = true;
                            }
                        }
                    });
                    if flag {
                        result = true;
                    } else {
                        result = false;
                    }
                    return result;
                }

                1 => {
                    let mut new_dir = Dir::default();
                    let mut flag = true;
                    new_dir.name = node_path.last().unwrap().to_string();
                    self.root.children.iter().for_each(|x| match x {
                        Node::Dir(d) => {
                            if d.name == new_dir.name {
                                flag = false;
                            }
                        }
                        Node::File(f) => {}
                    });
                    if flag {
                        self.root.children.push(Node::Dir(new_dir));
                        FileSystem::mk_dir_on_resource(&root_name);
                        result = true;
                    } else {
                        result = false;
                    }
                    return result;
                }
                2 => {
                    let mut index = -1;
                    self.root
                        .children
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| match x {
                            Node::Dir(d) => {
                                if d.name == node_path.last().unwrap().to_string() {
                                    index = i as i32;
                                }
                            }
                            Node::File(f) => {}
                        });
                    if index >= 0 {
                        self.root.children.remove(index as usize);
                        FileSystem::rm_dir_on_resource(&root_name);
                        result = true;
                    } else {
                        result = false;
                    }
                    return result;
                }

                3 => {
                    let mut new_file = File::default();
                    let mut flag = true;
                    new_file.name = node_path.last().unwrap().to_string();
                    self.root.children.iter().for_each(|x| match x {
                        Node::Dir(d) => {}
                        Node::File(f) => {
                            if f.name == new_file.name {
                                flag = false;
                            }
                        }
                    });
                    if flag {
                        self.root.children.push(Node::File(new_file));
                        FileSystem::mk_file_on_resource(&root_name);
                        result = true;
                    } else {
                        result = false;
                    }
                    return result;
                }
                _ => {
                    return false;
                }

                4 => {
                    let mut index = -1;
                    self.root
                        .children
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| match x {
                            Node::File(f) => {
                                if f.name == node_path.last().unwrap().to_string() {
                                    index = i as i32;
                                }
                            }
                            Node::Dir(d) => {}
                        });
                    if index >= 0 {
                        self.root.children.remove(index as usize);
                        FileSystem::rm_file_on_resource(&root_name);
                        result = true;
                    } else {
                        result = false;
                    }
                    return result;
                }
            }
        }

        let mut sub_dirs_names = Vec::<String>::new();
        self.root.children.iter().for_each(|x| match x {
            Node::Dir(d) => sub_dirs_names.push(d.name.clone()),
            Node::File(f) => {}
        });

        FileSystem::option_parser_recursive(
            &mut self.root.children,
            &mut node_path,
            &root_name,
            &mut sub_dirs_names,
            &mut cursor,
            &mut result,
            option,
        );

        return result;
    }

    fn rm_dir(&mut self, path: &str) -> () {
        let result = self.is_path_valid(path, 2);
        if result == true {
            println!("***********rm_dir****************");
            println!("dir {} have been removed", path);
            println!("***********rm_dir end****************");
        } else {
            println!("***********rm_dir****************");
            println!("fail");
            println!("***********rm_dir end****************");
        }
    }

    fn mk_dir(&mut self, path: &str) -> () {
        let result = self.is_path_valid(path, 1);
        if result == true {
            println!("***********mk_dir****************");
            println!("dir {} has been added", path);
            println!("***********mk_dir end****************");
        } else {
            println!("***********mk_dir****************");
            println!("fail");
            println!("***********mk_dir end****************");
        }
    }

    fn mk_file(&mut self, path: &str) -> () {
        let result = self.is_path_valid(path, 3);
        if result == true {
            println!("***********mk_file****************");
            println!("file {} has been added", path);
            println!("***********mk_file end****************");
        } else {
            println!("***********mk_file****************");
            println!("fail");
            println!("***********mk_file end****************");
        }
    }

    fn rm_file(&mut self, path: &str) -> () {
        let result = self.is_path_valid(path, 4);
        if result == true {
            println!("***********rm_file****************");
            println!("file {} has been removed", path);
            println!("***********rm_file end****************");
        } else {
            println!("***********rm_file****************");
            println!("fail");
            println!("***********rm_file end****************");
        }
    }

    fn get_file(&mut self, path: &str) -> std::fs::File {
        let mut node_path: Vec<&str> = path.split('/').collect();
        let mut root_name = "C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_03/FileSystemManager/src/resources/".to_owned();
        if node_path.get(0).unwrap().to_string() != self.root.name {
            root_name = root_name + self.root.name.as_str();
            root_name = root_name + "/";
            root_name = root_name + path;
        } else {
            root_name = root_name + path;
        }
        if self.is_path_valid(path, 0) {
            std::fs::File::options()
                .write(true)
                .read(true)
                .open(root_name)
                .unwrap()
        } else {
            std::fs::File::options()
                .write(true)
                .read(true)
                .open("")
                .unwrap()
        }
    }
}

fn main() {
    let path = "C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_03/FileSystemManager/src/resources/parent_folder";
    let mut file_system = FileSystem::from_dir(path);
    //println!("***********File System Copy****************");
   // println!("{:?}", file_system);
   // println!("***********File System Copy****************");
    file_system.mk_dir("parent_folder/child_folder/okokok");
   // file_system.mk_file("parent_folder/child_folder/okok/file.txt");
    file_system.rm_dir("parent_folder/child_folder/okokok");
    println!("***********File System Copy****************");
    println!("{:?}", file_system);
    println!("***********File System Copy****************");
    file_system.get_file("file_txt.txt");
}
