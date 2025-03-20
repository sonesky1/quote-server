# drop-in-the-bucket

Application Author:
Susan Onesky

Project Name:
Drop in The Bucket - an Environmental Goals Server Application

This app has the focus of allowing multiple users to add a small or large environmental goal they achieved as well as a topic area they could categorize it under. The data a user enters 
will be stored and when they submit their information it is stored and they view all the data
that has been entered into the database by everyone that has used the app. This application has a design purpose on the front end of it having a simple user interface. It is not meant to be confusing but rather stick to the point - changing action and sharing actions with others within a community of application users. In theory this application could be run
locally on any server if the web site address was changed inside the code. Perhaps users
woudl want to run their own server in a different state. This could help with keeping
data more specific and pertaining to a city, state or even a group of individuals. 

Another option with the app is to make it cater to larger groups on the web. It could focus
on being available to anyone in a country, region or even worldwide. The application stores a 
database locally. If the app were to be hosted on various sites, each site server would have
its own local database. This could be useful for distributing the database across sites that just want specific users with their own unique data. 

Warning:
This application does not have any safeguards for use on the WWW right now beyond localhost. It does not have any kind of authentication. As data would be stored locally on a server at this time this could create problems. The application is still under development and for this
reason has not been made public yet, either. 

The application spins up a server locally that allows adding data and view entries asynchronously on the web.  This app uses Rust, Axum, SeaOrm, tokio and SQLite. If you do not have those installed you will need to do so. The server_app spins up a localhost web site for now. If this were to be hosted non-locally it would allow multiple users to access the app and add to it asynchronously.

The creates a database if one does not exist and allows the user to add entries to it while the database is only updated by one client at a time due to Mutex like functionality that is part of SeaOrm.  In addition, SQLite was used for calling the database. 

This will need to be run in a terminal or desktop that can redirect to localhost as this server visible for now only on the front-end on a local host. 

This program was run from a PC in VS Code. It was also tested on an Ubuntu machine at Portland State University in which I could view localhost.


To run the program:
From inside the directory "server-app" type the following:

cargo run

two boxes will appear and a Submit  button. Type your environmental goal step in box 1. In box 2 type a category
it could pertain to. You can create a category or if you see a related one appear when you start typing
you could repeat the same one. You can create your own category as these are up to interpretation. You can put more than once category as well if separated by commas. 

Examples:

Action:                                    Category:
Rode my bike to work                       transportation
Re-used my grocery bag                     reuse
Washed laundry in cold cycle               energy
Planted a tree                             trees
Planted a bush for local bees              plants
Did not buy anything at Target today       consumerism
Bought a used cooking pan at Goodwill      reuse, consumerism


If you don't type anything in either or both boxes and push the submit button you will see a response that explains how to use the app.

If you submit your response you will currently be relocated to a page that displays all the input the server has received so far. This is the contents of the database that is stored locally. 

To enter more information you need to return to the previous screen on the browser , delete info in the box you typed or refresh the screen there on that home page. You can then enter in another input. 


This project is still being worked on. It will store data as lowercase and filter out text that is likely invalid. It will also allow more interesting calls on the data that will have buttons that will be on the view data page to display additional data. Example calls will include:
View top 5 actions
View top 3 categories
View 10 random actions

Tests have also not yet been implemented but the program is being tested on the front-end by me. More tests will be in the code. I would like a way to test using Cypress (a testing suite for websites) by hitting the site with many entries. I tried this and the site froze at one point and did not redirect and said it was due to something at line 95 of main. However, this only happened to me once and all other times the data added. I had a few windows up on my screen so this could have been why. 

Later, this will have much more functionality on the back end.I have more future ideas such as user authentication.
I also want to capture key words in the entries to gather such as "bike". I have a large list of ideas to implement with the
app to make it more sustainable energy wise. I also want to make it at times utilize AI. 

There are other ideas I have for this app that are confidential at this time. Have fun with the app and I hope it helps you think about small steps you can do to help the environment. It has helped me adjust and notice a few of my own persistent habits and become more aware of small changes I can make. It has also gently made me think more about what I do in my day I could improve upon as the app keeps these small actions in my daily memory in the background as I open it throughout the day. I hope this momentum can improve and I can share this app with friends safely on the web once I have it more developed.  
