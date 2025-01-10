This project is meant to discover rust language and create a first application with it.

The project, "BOF", standing for "Box Of Files" has the following goals :

MVP : 
- create a command `bof init` to create a folder `.bof` at the root of the folder
- create a command  `bof index` which will create, for each file of the folder, a tuple stored in the `.bof` folder. For files, the tuple will be : (KEY, TYPE, CREATION_TIME, MODIFICATION_TIME, SIZE, INODE?) and for folders : (KEY, [(KEY, KIND, NAME), …], INODE ?)

MVP+ :
- create an inverted table that maintains, for each file, the list of all the folders where we can find it
- create the idea of "META BOF" that would maintain a global index of multiple BOFs

Project improvements : 
- do only one recursive pass on the files : when computing the hashes of a folder, directly create the files of the met files and folders