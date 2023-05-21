# Rust - Sample API notes

## Prerequisite
1. rust build nightly using rustup chain command.
    a. install rust with this command (Unix like or linux)
    ```shell
       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    b. install rust nightly (version this 1.71 rust nightly)
    ```shell
      rustup install nightly
    ```
    c. set the rust nightly installation set to default 
    ```shell
      rustup default nightly
    ```
    d. install the crate by this command
    ```shell
      make install
    ```
    or use this command
    ```shell
      cargo add actix-web
        cargo add actix-cors
        cargo add serde_json
        cargo add serde --features derive
        cargo add chrono --features serde
        cargo add env_logger
        cargo add dotenv
        cargo add uuid --features "serde v4"
        cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
        # HotReload
        cargo install cargo-watch
        # SQLX-CLI
        cargo install sqlx-cli
    ```
2. make `.env` file on the root project folder
3. setup the env like database, etc.
4. compile the web app
   ```shell
      cargo build
    ```
5. running the web app service
    ```shell
      cargo run 
    ```
   or prefer the release
   ```shell
      cargo run --release
   ```
    
