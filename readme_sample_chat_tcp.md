## Websocket Chat Example

This  mvp is an extension of the actix chat example. 

Backend Framework: Actix

Multi-threaded chat server example

Fancy shiny features: 

* Browser-based WebSocket client served from static html+js --> Can I combine it with NextJS???


* Chat server runs in separate thread
* Tcp listener runs in a different thread.
* Application state is shared with the websocket server and a resource at /count/
* Uses actors for improved readability of code 


Chat server listens for incoming tcp connections. Server can access several types of message: 

/list -- list all available rooms,
/join name -- join room, if room does not exist
/name -- set session name
/some message -- just string, send mesage to all peers in the same room.


* Multiple clients try to get listen to server. if client does not respond within 10s connection gets dropped.


