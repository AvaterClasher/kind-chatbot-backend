<!-- @format -->

<h1 align="center">Kind Chatbot</h1>

<p align="center">
  An open-source AI chatbot server built with Rocket, Gemini API, and Rust 🦀.
</p>

<p align="center">
  <a href="#technologies-used"><strong>Features</strong></a>.
  <a href="#running-locally"><strong>Running locally</strong></a>.
  <a href="#api"><strong>API</strong></a>.
  <a href="#authors"><strong>Author</strong></a>.
</p>

## Technologies Used

-   [Rocket 🚀](https://rocket.rs/) as the server framework
-   [Gemini API](https://ai.google.dev/) as the LLM API
-   Written in [Rust 🦀](https://www.rust-lang.org/)

## Running locally

You will need to use the environment variables defined in [`.env.example`](.env.example) to run Rocket server. It's recommended you just copy the variable to a `.env` file.

You can get the Gemini API Key [here](https://aistudio.google.com/app/apikey).

> Note: You should not commit your `.env` file or it will expose secrets that will allow others to control access to your various Gemini and authentication provider accounts.

-   Install Rust:
    -   For Linux : `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    -   For Windows : [See this link](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup)
-   Set the Current Rust toolchain to nighly to build the server: `rustup default nightly`
-   Then Just Run the Server using: `cargo run`

Your server should now be running on [localhost:8000/chat](http://localhost:8000/chat).

## API

REST API :

```
curl -X POST -H "Content-Type: application/json" -d '{
    "user_message": "{PROMPT_HERE}"
}' https://localhost:8000/chat
```

## Authors

Rustfully made by 🦀

-   Soumyadip Moni ([@avater_clasher](https://github.com/AvaterClasher))
