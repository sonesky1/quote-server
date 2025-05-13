# quote-server

Application Author:
Susan Onesky

First Date of Publication: 5/2/25 . This is based partially off a previous project for the initial set up as it is similar to what is being done in Full Stack Rust. It will have more features and functionality added. 

Project Name:
quote-server    : A server that uses AXUM, tokio, RUST and SeaORM that displays quotes, authenticates users for security and allows users to enter quotes.

Project Description:

This program was run from a PC in VS Code. It was also tested on an Ubuntu machine at Portland State University in which I could view localhost. This program will continue to be developed using resources such as AI, course content and guidance from Bart Massey, a Portland State professor that teaches courses in RUST. 

To Run:
Install RUST, AXUM, SeaORM, tokio.

Open an IDE or a box that works with localhost on your computer front end

Open the folder you downloaded from github:
https://github.com/sonesky1/quote_server

cargo run

Go to localhost


If you wish to start fresh with a new database file (this will delete all old database history!) you can remove the current database file completely from your file folder (delete it). You would just run the program again after and a new empty database will be created. This option will allow you to customize your own local version to your liking.
This is not available to do once the program is running on a server. In other words, a user of the server cannot delte the database unless that functionality were built in by the developer (you!). 

TESTING:
This project was tested primarily manually with trial and error in most cases at this time. I brought up different localhost web pages on my local machine. This will later be integrated with testing suites using Cypress.io, if applicable to this platform with RUST and AXUM. I am assuming so as it does have a front end but will need more research to find out if this is possible. 


Further possibilities for Development:

Buttons that will be on the view data page to display additional data. Example calls will include:
View 3 most recent quotes entered
View 1 random quotes
User authentication for app safety
Categorizing by topic and grouping similar responses




License:
Apache
http://www.apache.org/licenses/

This code is not to be redistributed, published, nor altered at this time by anyone except the author.
