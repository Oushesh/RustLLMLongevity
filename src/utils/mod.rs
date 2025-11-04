use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Instant,
};


//Entry point for our websocket route
async fn chat_route(

)-> Result <HttpResponse,Error>{
    ws::start(
        session::WsChatSession{
            id:0,
            hb: Instant::now(),
            room: "main".to_owned(),
        }
    )

}



/// Displays state 
async fn get_count (count:)-> impl{
    let current_count = count.load();
    format!
}


// the actor-based websocket examples REQUIRE 'actix_web::main' for actor support
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env_logger::init_from_env()


    //set up applications state
    //keep a count of the numer of visitoers
    let app_state = Arc::new(AtomicUsize::new(0));


    // start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

    //Log and see all the possible information available
    log::info!("")
}