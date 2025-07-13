# Rust-WASM
Project is divided into two parts.
-> Frontend and Backend.
-> Both can be run from the docker compose file.
-> Front end  can be accessed of browser using  http://127.0.0.1:8080/ as the web address
-> Front end provides a button which requests through an api to the backend and returns a simple string as the api's callback result.
-> Frontend functionality is wrapped inside WASM.

To run the program through docker compose setup use the command "docker compose up --build" inside the folder containing docker compose file.