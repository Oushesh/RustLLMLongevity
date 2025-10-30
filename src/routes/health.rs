pub async fn health_check() -> &'static str { "OK" }


/*
What it does? 
src/routes/chat.rs. It handles /chat 
--> orchestrates RAG Flow (LLM call +search).

*/