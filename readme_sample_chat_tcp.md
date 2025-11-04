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

