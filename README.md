# crypto-trading

## Dependencies
This utilises my homebrewed [dataframe](https://github.com/caljoshba/dataframe/tree/master), so ensure you have that cloned before running this. It needs to sit side-by-side with this codebase.

## What it does
This connects to the Kraken WebSocket API to pull the latest trading information for BTC/EUR.
This converts the response to a struct using [serde_json](https://docs.rs/serde_json/0.9.0-rc2/serde_json/) and is then loaded into the custom dataframe.

The returns for each value is then calculated and a rolling mean applied over the last 20 values to provide some stability to the price and to discern a pattern, otherwise the price fluctuates too much and looks completely random.
The returns are then printed as a graph in the `output` folder for every 100 price events we get from the Kraken API.

## Running the app
Ensure you have the [rust toolchain installed](https://www.rust-lang.org/tools/install) and then run:

    cargo run
  
This will build an run your application

Alternatively, you can run this inside docker:

    docker-compose up

Then `Ctrl + C` when you've had enough
