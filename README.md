# Description
The aim of this university project is to discover rust language and create a first application with it.

The name, "BOF", stands for "Box Of Files". 

To run the implemented features, run <code>./target/debug/bof \<command\></code>. The implemented commads are the following : 
- init : it will create a <code>.bof</code> folder in the current folder.
- index : it will create, inside the <code>.bof</code> folder, index files for every element contained in your current folder. Each file will be contained in a folder having for name the first two characters of the hash created for the indexed element. The file will contain some metadata of the element.
- showdir : LEGACY. I created it to discover what was possible with the language. It will print the content of your current folder.


Code improvements : 
- do only one recursive pass on the files : when computing the hashes of a folder, directly create the files of the met files and folders

# Initial goals

The project has the following goals :

MVP : 
- create a command `bof init` to create a folder `.bof` at the root of the folder
- create a command  `bof index` which will create, for each file of the folder, a tuple stored in the `.bof` folder. For files, the tuple will be : (KEY, TYPE, CREATION_TIME, MODIFICATION_TIME, SIZE, INODE?) and for folders : (KEY, [(KEY, KIND, NAME), …], INODE ?)

MVP+ :
- create an inverted table that maintains, for each file, the list of all the folders where we can find it
- create the idea of "META BOF" that would maintain a global index of multiple BOFs

