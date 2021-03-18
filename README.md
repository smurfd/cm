# cm
Code Monkey - Have a monkey keep track of your code and your TODO's inside of it (for starters)

Use Cargo to build the code <code>cargo build</code>
Then Cargo to run it aswell

If you use no parameters for cargo run you will get the usage
<code>cargo run</code>
```
cm wakeup                        // this will initialize a new database project.db
cm eat /path/to/folder projname  // this will consume a folders files into the database
cm spit                          // this will show the database
cm find TODO                     // this will search the database for TODO
cm sleep                         // this will kill the project.db file
cm update                        // this will update the project.db
```

And thats the order in how todo things.
First initiate a database with:
<code>cm wakeup</code>

then add data to your database with 
<code>cm eat /home/smurfd/Code mycode</code>

Then you can search for as an example TODO in your code database (you can search for what you want)
<code>cm find TODO</code>

That could give some output :
```
///////
Searching for: TODO
Found in project: 1
Found in project: 2
///////
TODO found in these rows:
cs | csharp/callback.cs | // TODO Make it show the description instead, After db is put to use
cs | csharp/view.cs | // TODO get filetype and set highlight lang. after that.
pascal | FLOGGER/FLOGGER.C | See the TODO document (which should accompany this program.)
pascal | FLOGGER/MAKEFILE | README INSTALLATION TODO OPTIMIZING Makefile bubble_sort.c \

```
still alot of things to add (aswell as the code :))



