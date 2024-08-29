# Start request

## body example

```json```
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
        "possible_actions": ["/start"]
    }
}
``

# Hit request

## body example

```json```
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
                "rank": "Four"
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
    },
    "hand_number":0
}
``

# Stand request

## body example

``` json ```
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
``

# Split request

## body request

``` json ```
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
``

# Double request

## body example

``` json ```
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
``