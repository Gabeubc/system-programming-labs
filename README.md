# System-programming-Labs
The course have two parts:

## OS internal part:
for the OS internal part we use the OS161 operating system for our laboratories:

### Lab_01: Explore OS161 

In this lab we install the **OS161** operating system and explore it.

#### Set the environment:

Execute the following command to get the OS161 operating system ready on your machine (Linux):

- cd $HOME

- sudo wget https://www.dropbox.com/s/ve16hjptqwrxm3l/os161-kit-mine.tgz (this link is provided by *Politecnico di Torino*)

- tar -zxvf os161-kit-mine.tgz 

tep into os161-kit, if the os161-downloads.tgz folder is not present create a folder a give it os161-downloads as name, step into it and execute the following command: (those link are provided by Harvard University)

- sudo wget http://os161.eecs.harvard.edu/download/binutils-2.24+os161-2.1.tar.gz  
- sudo wget http://os161.eecs.harvard.edu/download/bmake-20101215.tar.gz           
- sudo wget http://os161.eecs.harvard.edu/download/gcc-4.8.3+os161-2.1.tar.gz      
- sudo wget http://os161.eecs.harvard.edu/download/gdb-7.8+os161-2.1.tar.gz
- sudo wget http://os161.eecs.harvard.edu/download/mk-20100612.tar.gz
- sudo wget http://os161.eecs.harvard.edu/download/os161-base-2.0.3.tar.gz
- sudo wget http://os161.eecs.harvard.edu/download/sys161-2.0.8.tar.gz

Now, execute the following command from the OS161-kit folder and you will be ready:

- source os161-env-startup.sh
- source os161-kit-setup.sh ( if you get errors because of files missing, please read the errors and move the necessary folder )
- source os161-kit-build.sh
- source os161-kernel-build.sh




## Programming part:

For the programming part we will use RUST as programming language.

### Lab_01: 

At the end of the exercise, if properly carried out, students will be able to
- Use the cargo programme, correctly handling the reference to external libraries 
and the compilation process, and know the structure of a Rust programme
- Correctly use flow control constructs and the match instruction
- Understand the concept of owning a value and distinguish, in each segment of 
code segment, variables that have a value from those that don't
Manipulate strings, perform conversions between strings and byte arrays; know the 
difference between byte and char in string and slice handling
Understand the semantics of passing parameters to functions
- Perform function testing, referring to pre-existing tests and knowing how to 
write unit tests
- Handling arguments passed through the command line

