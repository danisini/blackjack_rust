# Blackjack

# Description

I have implemented a blackjack server which has the following API:


/start -> starts a new game


/hit -> player draws one more card


/split -> player splits their hand into two (only if their hand consists of same ranked card)


/stand -> player stands and wait for dealer to draw cards until game finishes


/double -> player doubles both their stakes (if present), draws one more card to the first hand and then stands


# How to start it
1) Download the git project
2) cargo build
3) cargo run
4) run a query with following request bodies as examples:

# Start request

## body example

```json
{
    {
    "state": {
        "player_won": false,
        "has_player_won": false,
        "has_dealer_won": false,
        "is_stake_doubled": false,
        "is_round_over": false,
        "stake": 0.0,
        "additional_stake":0.0,
        "win_amount": 0.0,
        "balance": 100.0,
        "player_hand": [],
        "dealer_hand": [],
        "player_split_hand": [],
        "possible_actions": ["/start"],
        "cards_dealt": []
    },
    "stake": 1.0
}
}
```

# Hit request

## body example

```json
{
    {
    "state": {
        "has_player_won": false,
        "has_dealer_won": false,
        "is_round_over": false,
        "is_stake_doubled": false,
        "stake": 1.0,
        "additional_stake": 0.0,
        "win_amount": 0.0,
        "balance": 100.0,
        "possible_actions": [
            "/double",
            "/stand",
            "/hit"
        ],
        "player_hand": [
            {
                "suit": "Diamonds",
                "rank": "Ace"
            },
            {
                "suit": "Diamonds",
                "rank": "Jack"
            }
        ],
        "player_split_hand": [],
        "dealer_hand": [
            {
                "suit": "Hearts",
                "rank": "Seven"
            }
        ],
        "cards_dealt": [
            {
                "suit": "Diamonds",
                "rank": "Ace"
            },
            {
                "suit": "Diamonds",
                "rank": "Jack"
            },
            {
                "suit": "Hearts",
                "rank": "Seven"
            }
        ]
    },
    "hand_number": 0
    }
}
```

# Stand request

## body example

``` json
{
    "state": {
        "player_won": false,
        "has_player_won": false,
        "has_dealer_won": false,
        "is_stake_doubled": false,
        "is_round_over": false,
        "stake": 1.0,
        "additional_stake":0.0,
        "win_amount": 0.0,
        "balance": 100.0,
        "player_hand": [
            {
                "suit": "Clubs",
                "rank": "Eight"
            },
            {
                "suit": "Clubs",
                "rank": "Ten"
            }
        ],
        "player_split_hand": [],
        "dealer_hand": [
            {
                "suit": "Clubs",
                "rank": "Nine"
            }
        ],
        "possible_actions": ["/hit", "/double", "/stand"]
    }
}
```

# Split request

## body request

``` json
{
    "state": {
        "player_won": false,
        "has_player_won": false,
        "has_dealer_won": false,
        "is_stake_doubled": false,
        "is_round_over": false,
        "stake": 1.0,
        "additional_stake":0.0,
        "win_amount": 0.0,
        "balance": 100.0,
        "player_hand": [
            {
                "suit": "Clubs",
                "rank": "Ten"
            },
            {
                "suit": "Clubs",
                "rank": "Ten"
            }
        ],
        "player_split_hand": [],
        "dealer_hand": [
            {
                "suit": "Clubs",
                "rank": "Nine"
            }
        ],
        "possible_actions": ["/split"]
    },
    "additional_stake": 2.0
}
```
# Double request

## body example

``` json 
{
    "state": {
        "player_won": false,
        "has_player_won": false,
        "has_dealer_won": false,
        "is_stake_doubled": false,
        "is_round_over": false,
        "stake": 1.0,
        "additional_stake":0.0,
        "win_amount": 0.0,
        "balance": 100.0,
        "player_hand": [
            {
                "suit": "Clubs",
                "rank": "Ten"
            },
            {
                "suit": "Clubs",
                "rank": "Ten"
            }
        ],
        "player_split_hand": [],
        "dealer_hand": [
            {
                "suit": "Clubs",
                "rank": "Nine"
            }
        ],
        "possible_actions": ["/double", "/hit", "/split"]
    }
}
```