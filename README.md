# Project 3: Serverless Data Engineering Pipeline 

## Goals
This project is about using Shuttle.rs to deploy serverless data engineering pipelines using Rust besed on a simple rock-paper-scissors game. I build a useful, serverless application in Rust. Shuttle.rs is a Rust-based tool that simplifies the deployment process of Rust applications to AWS Lambda. By using Shuttle.rs, you can package your Rust-based data engineering pipeline into a zip file that can be uploaded to AWS Lambda. This allows you to build a serverless data engineering pipeline with Rust and take advantage of Rust's performance and safety features. The process involves creating your Rust-based pipeline, using Shuttle.rs to package it into a zip file, creating a new AWS Lambda function, uploading the zip file to your Lambda function, adding a trigger to your Lambda function, testing your pipeline, and monitoring it to ensure it is working correctly.

### Rules:
1. "Rock, paper, scissors" is a hand game played by two people. The game has three possible outcomes: a win for the player, a win for the computer, or a tie.

2. In the game, the player and the computer both choose one of three options: rock, paper, or scissors. The rules for determining the winner are:
* Rock beats scissors
* Scissors beats paper
* Paper beats rock
* If both the player and the computer choose the same option, the game is a tie.

3. So, for example, if the player chooses "rock" and the computer chooses "scissors", the player wins. If the player chooses "scissors" and the computer chooses "paper", the player wins. If the player chooses "paper" and the computer chooses "rock", the player wins. On the other hand, if the player chooses "rock" and the computer chooses "paper", the computer wins. If the player chooses "scissors" and the computer chooses "rock", the computer wins. If the player chooses "paper" and the computer chooses "scissors", the computer wins.

4. In summary, to win at "rock, paper, scissors", the player needs to choose the option that beats the computer's choice.

## Functions
microservice using Actix Web provides an HTTP endpoint at "/play" that can be used to play the game "rock, paper, scissors". When a user accesses the endpoint, the service generates a random selection for the computer's move (either "rock", "paper", or "scissors"), and returns a response to the user indicating the computer's move. The implementation of the game logic is simple, as it only generates a random computer move, without taking into account the user's input or determining a winner.

The actix_web macro is used to define the entry point of the Actix Web server, which is provided by the ShuttleActixWeb type from the shuttle_service crate. The playgame function is the handler for the "/play" endpoint and returns an HTTP response with the computer's move. The actix_web macro is then used to wrap the main function that sets up the Actix Web server and registers the playgame handler with the service configuration.

Overall, your microservice provides a simple implementation of the "rock, paper, scissors" game over HTTP using Actix Web and Rust.

<img width="1023" alt="Screen Shot 2023-03-09 at 9 53 11 PM" src="https://user-images.githubusercontent.com/112274822/224213987-49562f9f-821f-42ef-9ce5-58d28b298b1b.png">
