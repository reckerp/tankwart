# tankwart

Monitor German fuel prices and get notified when prices drop below your threshold.

## About

tankwart is a Rust application that polls the [Tankerkönig API](https://creativecommons.tankerkoenig.de/) for fuel prices at gas stations and sends notifications via [ntfy.sh](https://ntfy.sh/) when prices drop below a configured threshold.

## Features

- Monitors Diesel, E5, and E10 fuel prices
- Configurable price thresholds per fuel type
- Notifications via ntfy.sh push notifications
- 5-minute polling interval
- Supports multiple gas stations

## Installation

```bash
git clone https://github.com/reckerp/tankwart.git
cd tankwart
cargo build --release
```

## Configuration

Create a `.env` file based on `.env.example`:

```env
# https://creativecommons.tankerkoenig.de/
TANKERKOENIG_API_KEY=your_api_key

# ntfy.sh
NTFY_TOPIC=your_topic

# separated by comma
STATION_IDS=station_id_1,station_id_2,station_id_3

# price thresholds (optional)
THRESHOLD_E5=1.700
THRESHOLD_E10=1.700
THRESHOLD_DIESEL=1.600
```

### Getting a Tankerkönig API Key

1. Visit <https://creativecommons.tankerkoenig.de/>
2. Register for a free API key
3. Use the API key in your `.env` file

### Finding Gas Station IDs

1. Visit <https://tankerkoenig.de/> (in German)
2. Search for your preferred gas stations
3. The station ID is visible in the URL or can be found through the API

### Setting Up ntfy.sh

1. Visit <https://ntfy.sh/>
2. Subscribe to a topic (or create your own)
3. Use the topic name in your `.env` file
4. Install the ntfy app on your phone to receive push notifications

## Usage

```bash
cargo run --release
```

The application will:

1. Poll fuel prices every 5 minutes
2. Print current prices to the console
3. Send a notification when a price drops below your threshold

## Notifications

Notifications are only sent when:

- The current price is at or below your threshold
- The price has dropped compared to the last check

This prevents notification spam when prices fluctuate around your threshold.

## License

MIT
