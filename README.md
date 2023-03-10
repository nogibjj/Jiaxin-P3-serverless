# Project 3: Serverless Data Engineering Pipeline 

## Goals
This project is about using Shuttle.rs to deploy serverless data engineering pipelines using Rust besed on a simple rock-paper-scissors game. I build a useful, serverless application in Rust. Shuttle.rs is a Rust-based tool that simplifies the deployment process of Rust applications to AWS Lambda. By using Shuttle.rs, you can package your Rust-based data engineering pipeline into a zip file that can be uploaded to AWS Lambda. This allows you to build a serverless data engineering pipeline with Rust and take advantage of Rust's performance and safety features. The process involves creating your Rust-based pipeline, using Shuttle.rs to package it into a zip file, creating a new AWS Lambda function, uploading the zip file to your Lambda function, adding a trigger to your Lambda function, testing your pipeline, and monitoring it to ensure it is working correctly.

## Rules
1. "Rock, paper, scissors" is a hand game played by two people. The game has three possible outcomes: a win for the player, a win for the computer, or a tie.
2. In the game, the player and the computer both choose one of three options: rock, paper, or scissors. The rules for determining the winner are:
* Rock beats scissors
* Scissors beats paper
* Paper beats rock
* If both the player and the computer choose the same option, the game is a tie.
3. So, for example, if the player chooses "rock" and the computer chooses "scissors", the player wins. If the player chooses "scissors" and the computer chooses "paper", the player wins. If the player chooses "paper" and the computer chooses "rock", the player wins. On the other hand, if the player chooses "rock" and the computer chooses "paper", the computer wins. If the player chooses "scissors" and the computer chooses "rock", the computer wins. If the player chooses "paper" and the computer chooses "scissors", the computer wins.
4. In summary, to win at "rock, paper, scissors", the player needs to choose the option that beats the computer's choice.

## Structure Diagram
<img width="1001" alt="Screen Shot 2023-03-10 at 12 07 51 AM" src="https://user-images.githubusercontent.com/112274822/224228728-5aa89a04-79fa-4a2b-91e0-4bdc60f86b18.png">

## Demo Link
https://teams.microsoft.com/l/message/19:0ceb53838ecf4f9381360fecae2915f6@thread.tacv2/1678425282734?tenantId=cb72c54e-4a31-4d9e-b14a-1ea36dfac94c&groupId=7757ff33-20f5-434b-8d0b-5fe61b251c34&parentMessageId=1678425282734&teamName=IDS-721-Spring-2023&channelName=Week%209%20Individual%20Project%203&createdTime=1678425282734&allowXTenantAccess=false

## Functions
microservice using Actix Web provides an HTTP endpoint at "/play" that can be used to play the game "rock, paper, scissors". When a user accesses the endpoint, the service generates a random selection for the computer's move (either "rock", "paper", or "scissors"), and returns a response to the user indicating the computer's move. The implementation of the game logic is simple, as it only generates a random computer move, without taking into account the user's input or determining a winner.

The actix_web macro is used to define the entry point of the Actix Web server, which is provided by the ShuttleActixWeb type from the shuttle_service crate. The playgame function is the handler for the "/play" endpoint and returns an HTTP response with the computer's move. The actix_web macro is then used to wrap the main function that sets up the Actix Web server and registers the playgame handler with the service configuration. Overall, your microservice provides a simple implementation of the "rock, paper, scissors" game over HTTP using Actix Web and Rust.

<img width="1023" alt="Screen Shot 2023-03-09 at 9 53 11 PM" src="https://user-images.githubusercontent.com/112274822/224213987-49562f9f-821f-42ef-9ce5-58d28b298b1b.png">

## Prepration
1. Set virtual environment: 
* `python3 -m venv env`
* `source env/bin/activate`
2. Install rust: 
* `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* `source "$HOME/.cargo/env"`
3. login to Shuttle.rs
<img width="1343" alt="Screen Shot 2023-03-09 at 10 16 07 PM" src="https://user-images.githubusercontent.com/112274822/224214373-8ed46dda-cf39-4779-8124-aa3516c1b999.png">

## Usage
1. Start by installing the cargo shuttle subcommand by running the following in a terminal:
`cargo install cargo-shuttle`
2. Log in, run this command to start the login process:
`cargo shuttle login`
3. Authenticate, run this command to authenticate:
`cargo shuttle login --api-key 6jiJsI8dy6QwGTQT`
4. Initialize, run this command to initialize your project:
`cargo shuttle init`
<img width="701" alt="Screen Shot 2023-03-09 at 9 24 15 PM" src="https://user-images.githubusercontent.com/112274822/224215076-041bfd8a-793f-4b20-b2bb-567553d006da.png">

## Running Locally
To test my app locally before deploying, use: `cargo shuttle run`
<img width="862" alt="Screen Shot 2023-03-09 at 10 32 56 PM" src="https://user-images.githubusercontent.com/112274822/224216622-4e426116-5495-426d-992f-59f4e8392bac.png">

* When type "/play", there are three possible results:
<img width="662" alt="Screen Shot 2023-03-09 at 9 04 16 PM" src="https://user-images.githubusercontent.com/112274822/224216196-42f9d2a9-5738-472d-9739-530071f510d7.png">
<img width="671" alt="Screen Shot 2023-03-09 at 9 05 16 PM" src="https://user-images.githubusercontent.com/112274822/224216199-7d4363bd-a83e-408e-898d-a8898b155efb.png">
<img width="650" alt="Screen Shot 2023-03-09 at 9 04 24 PM" src="https://user-images.githubusercontent.com/112274822/224216200-b25453ab-30bc-48e9-b6c5-177e2e40b992.png">

## Deploy Via Shuttle
1. Run this command to deploy your project 🥳:`cargo shuttle deploy`
<img width="1020" alt="Screen Shot 2023-03-09 at 9 24 28 PM" src="https://user-images.githubusercontent.com/112274822/224215251-91c7ac9e-ec44-4b9f-a558-e67c2af43634.png">

2. Sometimes when you deploy your project, it will show errors that need to debug:
<img width="1007" alt="Screen Shot 2023-03-09 at 6 26 43 PM" src="https://user-images.githubusercontent.com/112274822/224215423-09654a50-448e-4700-870e-9c41a431261d.png">

3. Once finsh debugging your errors, deploy it again untill it succeed:

4. Your service will immediately be available at {crate_name}.shuttleapp.rs. My example:
<img width="1015" alt="Screen Shot 2023-03-09 at 10 26 02 PM" src="https://user-images.githubusercontent.com/112274822/224215735-4af31e8d-83ae-470f-ab23-bc4f305343de.png">

5. Run this code in terminal: `curl https://playgame.shuttleapp.rs/play`

* eg: I run it many times, and for each time, it will return me a random result from the list: [Rock, paper, scissors]

<img width="929" alt="Screen Shot 2023-03-09 at 10 26 16 PM" src="https://user-images.githubusercontent.com/112274822/224217330-9f03ae57-ee35-4400-b26f-5952cba85550.png">

### **Reference**
1. https://docs.rs/shuttle-service/latest/shuttle_service/
2. https://docs.shuttle.rs/examples/actix
