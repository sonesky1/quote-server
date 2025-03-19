# drop-in-the-bucket
An Environmental Goals Server App: A server that can be run and allow users to enter data and view entries asynchronously on the web. This app uses Rust, Axum, SeaOrm and SQLite. The server_app spins up a localhost web site for now. If this were to be hosted non-locally it would allow multiple users to access the app and add to it asynchronously. 

It creates a database if one does not exist and allows the user to add entries to it while the database is only updated by one client at a time due to Mutex like functionality that is part of SeaOrm.  SQLit was used for this app due to its future adaptability with more complex queries.  

This will need to be run in a terminal or desktop that can redirect to localhost as this server is on the front end web
interface on a localhost in how it displays and in how a user will interact with it. It is only local at this time
This program was run from my PC in Visual Studio. I am unable to run it in the linux terminal on my PC as it does not bring up localhost. However, it does run in terminal on the PSU Ubuntu machines. 


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


If you don't type anything and push the button you will see a response that explains how to use the app.

If you submit your response you will currently be relocated to a page that displays all the input the server has received so far. This is the contents of the database that is stored locally. 

To enter more information you need to return to the previous screen on the browser , delete info in the box you typed or refresh the screen. You can then enter in another input. 

Please ignore the "View responses" button as this will be changed to do what it should do later. It does not yet redirect correcly.  The submit button will also instead just clear the entry boxes on the page once an entry occurs rather than redirecting the user. 

This project is still being worked on. It will store data as lowercase and filter out text that is likely invalid. It will also allow more interesting calls on the data that will have buttons that will be on the view data page to display additional data. Example calls will include:
View top 5 actions
View top 3 categories
View 10 random actions

Tests have also not yet been implemented but the program is being tested on the front-end by me. More tests will be in the code. I would like a way to test using Cypress (a testing suite for websites) by hitting the site with many entries. I tried this and the site froze at one point and did not redirect and said it was due to something at line 95 of main. However, this only happened to me once and all other times the data added. I had a few windows up on my screen so this could have been why. 

Later, this will have much more functionality on the back end.I have more future ideas such as user authentication.
I also want to capture key words in the entries to gather such as "bike". I have a large list of ideas to implement with the
app to make it more sustainable energy wise. I also want to make it at times utilize AI. 

There are other ideas I have for this app that are confidential at this time. Have fun with the app and I hope it helps you think about small steps you can do to help the environment. It has helped me adjust and notice a few of my own persistent habits and become more aware of small changes I can make. It has also gently made me think more about what I do in my day I could improve upon as the app keeps these small actions in my daily memory in the background as I open it throughout the day. I hope this momentum can improve and I can share this app with friends safely on the web once I have it more developed.  
