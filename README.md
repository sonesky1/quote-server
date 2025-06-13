# quote-server

Application Author:
Susan Onesky

First Date of Publication: 5/2/25 . This is based partially off a previous project for the initial set up as it is similar to what is being done in Full Stack Rust. It will have more features and functionality added. 

Project Name:
quote-server    : A server that uses AXUM, tokio, RUST and SeaORM , CSS and Askama that displays random quotes and allows users to enter quotes.

Project Description:

This program was run from a PC in VS Code. It was also tested on an Ubuntu machine at Portland State University in which I could view localhost. This program will continue to be developed. This is the start of an assignment worked on in the course Full Stack Rust with Bart Massey at PSU

To Run:
Install RUST, AXUM, SeaORM, tokio.

Open an IDE or a box that works with localhost on your computer front end

Open the folder you downloaded from github:
https://github.com/sonesky1/quote_server

cargo run

Go to localhost : http://127.0.0.1:8080

Originally a hardcoded quote will appear. Once you add a new quote then your 1 quote will be the first in the actual database. You may need to add a few to start seeing the random quote effects more since at the start there are not many quotes to choose from. 


If you wish to start fresh with a new database file (this will delete all old database history!) you can remove the current database file completely from your file. You would just run the program again after and a new empty database will be created. This option will allow you to customize your own local version to your liking.
This is not available to do once the program is running on a server. In other words, a user of the server cannot delte the database unless that functionality were built in by the developer (you!). 

TESTING:
This project was tested primarily manually with trial and error in most cases at this time. I brought up different localhost web pages on my local machine. 

Problems found that need to be resolved: This has errors when entering text with  single quotes. It will not add to the database as it messes up the formatting for the sqlite and the string does not filter out that single quote in any way to deal with it. 


Further Goals for Development:

Buttons that will be on the view data page to display additional data. 
Example API calls will include: Ability to delete and edit an entry by ID, a new table for quote category or contributor name and country
User authentication for app safety
Categorizing a quote by topic
Better organization of files
Allowing a query to display quote by topic
A Utoipa-documented REST API, with a documentation endpoint for at least Swagger-UI
A basic REST browser client UI written in Yew, Leptos or some other WASM-based frontend
Authentication using the JWT scheme taught in class or something similar.
Ability to do authenticated writes to the database through one or more client UIs and/or the REST API
A working Dockerfile
Populating database from a large file of quotes 

I would love to change this to not have quotes but be used for language learning as I have an interest in Portuguese.






License:
Apache
http://www.apache.org/licenses/

This code is not to be redistributed, published, nor altered at this time by anyone except the author.
