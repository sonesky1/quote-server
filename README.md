# drop-in-the-bucket

Application Author:
Susan Onesky

Date of Publication: March 19th, 2025

Project Name:
Drop in The Bucket - an Environmental Actions Server App

Project Description:
Do you sometimes feel like what you do will not make a difference in helping the environment? What you do is just a "Drop in the Bucket?" ...so why bother? Drop in the Bucket app is here to prove you wrong..to show what happens when small actions add up exponentially. This app has the focus of allowing multiple users to add a small or large environmental action they did that day as well as a topic area they could categorize it under. The data a user enters 
will be stored and when they submit their information it is stored and they view all the data
that has been entered into the database by everyone that has used the app. 
This application has a very specific design purpose on the front end of it having a simple user interface. It is not meant to be confusing but rather stick to the point - changing action and sharing actions with others within a community of application users. 

In theory this application could be run
locally on any server if the web site address was changed inside the code. Perhaps users
would want to run their own server in a different state. This could help with keeping
data more specific and pertaining to a city, state or even a group of individuals. 

Another option with the app is to make it cater to larger groups on the web. It could focus
on being available to anyone in a country, region or even worldwide. The application stores a 
database locally. If the app were to be hosted on various sites, each site server would have
its own local database. This could be useful for distributing the database across sites that just want specific users with their own unique data. 

Warning:
This application does not have any safeguards for use on the WWW right now beyond localhost. It does not have any kind of authentication. As data would be stored locally on a server at this time this could create problems. The application is still under development and for this
reason has not been made public. It also does not currently do any checking of text that is entered
by a user in any way at this time. 

The application spins up a server locally that allows adding data and view entries asynchronously on the web.  This app uses Rust, Axum, SeaOrm, tokio and SQLite. If you do not have those installed you will need to do so. The server_app spins up a localhost web site. You can add to it from accessing local host in different browsers on your desktop at this time.  If this were to be hosted non-locally it would allow multiple users to access the app and add to it asynchronously from anywhere the web site could be accessed.

The creates a database if one does not exist and allows the user to add entries to it while the database is only updated by one client at a time due to Mutex like functionality that is part of Arc with SeaOrm.  In addition, SQLite was used for calling the database. 

How to RUN the Program:

This will need to be run in a terminal or desktop that can redirect to localhost as this server visible for now only on the front-end on a local host. 

This program was run from a PC in VS Code. It was also tested on an Ubuntu machine at Portland State University in which I could view localhost.

Install the necessary on your computer if not done so including RUST, AXUM, SeaORM, tokio.

Open an IDE or a box that works with localhost on your computer front end

Open the folder you downloaded from github:
https://github.com/sonesky1/drop-in-the-bucket

Navigate to the directory "server-app" ,type the following in the terminal in your IDE.

cargo run

two boxes will appear and a Submit  button when you navigate to the localhost that will appear when running the program. Type your environmental goal step in box 1. In box 2 type a category
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

If you submit your response a reroute to a page that displays all the input the server has received so far. This is the contents of the database that is stored locally. What you see
could be from your last sessions, last sessions of other users as well as how the database was
originally uploaded to github with populated information.

If you wish to start fresh with a new database file (this will delete all old database history!) you can remove the current database file completely from your file folder (delete it). You would just run the program again after and a new empty database will be created. 

To enter more information you need to return to the previous screen on the browser , delete info in the box you typed or refresh the screen there on that home page. You can then enter in another input. 

TESTING:

This project was tested primarily manually with trial and error in most cases at this time. I brought up different localhost web pages on my local machine when the server was running to be sure all were adding to the same database. I did testing on the front end of adding entries
and observing whether the app behaved as expected. I tried entering text in the boxes, not entering text in both or just one box. I also tried deleting the database to see whether it created
another new database if one no longer existed. 


I was unable to test this with hitting it with enough data very quickly to test what happens nor was I able to do extensive error testing of data 
types , lengths or errors users could have with entering data. What would have been needed with more time in order to run more test scenarios would be test suites with pre-populated to data and actions on the data being entered and then comparing output of strings that would be expected to result. These could be unit tests hard coded inside the code. However, 
to test behaviour of the front end in the future I would like to create tests using Cypress, a suite for testing web applications. I spent time testing the app
with manual entries since so much of what was seen was on the front end. Setting up the server itself to run and add just a basic entry to start and to be able to view this entry calling the database and populating it on the front end is what took the majority of the time on the project. I run localhost many many times until seeing the basic operations of that occuring as a base. So, this made most testing just trial, error and seeing localhost results. 

This is not how I would continue if developing further. As the basics are here the data entry details of errors to catch in entries woudl be the next step if time had allowed. Users could
enter very long text, embed HTML into their text entry even- which would not work with this application I realized since the application does not return JSON nor filter out any HMTL entered into a String typed into the field! HTML entered as well as text in the database could lead to big unpredictable issues. 

Further possibilities for Development:

Buttons that will be on the view data page to display additional data. Example calls will include:
View top 5 actions
View top 3 categories
View 10 random actions
Data by location
User authentication for app safety
Categorizing by topic and grouping similar responses under a count 
Removing non key words in storage (I  verb tense->verb only  pronouns )
filtering of bad responses (censoring ?) before adding to database
Avoid spamming of site by a bad agent, shut down client, prevent access

There are other ideas I have for this app as well and will continue with it.

 Have fun with the app and I hope it helps you think about small steps you can do to help the environment. It has helped me adjust and notice a few of my own hard to change habits. It has also gently made me think more about what I do in my day I could improve upon. The app keeps these small actions in my daily memory in the background. I hope I can keep the momentum going to share this app with friends safely on the web once I have it more developed.

 Sources:
 Deepseek was used in converting some code to axum (although with many alterations needed!) and assisting with populating an html String table from the database. Jim Blandy's Programming with RUST Server example in Chapter 1-2, with actix, was used in original idea in how to create a simple landing page in html with boxes and heading.

License:
Apache
http://www.apache.org/licenses/

This code is not to be redistributed, published, nor altered at this time by anyone except the author.