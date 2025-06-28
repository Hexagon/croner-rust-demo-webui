# Croner Rust - Web UI Demo

This project is a web-based demonstration of the rust variant of `croner`, providing a user-friendly interface to test and understand cron expressions. It is built with a Rust backend using the Axum web framework and a responsive frontend styled with Tailwind CSS.

## Features

* **Real-time Cron Testing:** Enter a cron expression and instantly see the next five scheduled occurrences.
* **Human-Readable Descriptions:** Get a natural language explanation of what your cron pattern means.
* **Internationalization:** View descriptions in multiple languages (currently English and Swedish).
* **Advanced Parser Options:**
    * Toggle how the **Day of Month** and **Day of Week** fields are combined (AND/OR logic).
    * Control how the **seconds field** is handled (required, optional, or disabled).

## Prerequisites

Before you begin, ensure you have the following installed:
* [Rust and Cargo](https://www.rust-lang.org/tools/install) (latest stable version recommended)

## How to Run the Demo

1.  **Clone the `croner-rust-demo-webui` repository:**
    ```bash
    git clone [https://github.com/hexagon/croner-rust-demo-webui.git](https://github.com/hexagon/croner-rust-demo-webui.git)
    cd croner-rust
    ```

2.  **Run the web ui:**
    ```bash
    cargo run 
    ```
    Alternatively, you can navigate into the example directory (`cd examples/api_demo`) and run `cargo run`.

3.  **Open the Web UI:**
    Once the server is running, you will see the following message in your terminal:
    ```text
    >> Croner API Demo listening on [http://127.0.0.1:3000](http://127.0.0.1:3000)
    >> Visit [http://127.0.0.1:3000](http://127.0.0.1:3000) to use the Cron Tester.
    ```
    Open your web browser and navigate to **`http://127.0.0.1:3000`**.


## Technology Stack

* **Backend:**
    * [Rust](https://www.rust-lang.org/)
    * [Croner](https://crates.io/crates/croner)
    * [Axum](https://github.com/tokio-rs/axum): Web framework
    * [Tokio](https://tokio.rs/): Asynchronous runtime
    * [Serde](https://serde.rs/): Data serialization/deserialization
* **Frontend:**
    * HTML5
    * [Tailwind CSS](https://tailwindcss.com/): Utility-first CSS framework
    * Vanilla JavaScript
