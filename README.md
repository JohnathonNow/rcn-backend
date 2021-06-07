# Cosmetics Network backend
This is the backend code for the [Cosmetics](https://github.com/JohnathonNow/runelite-cosmetic-network) plugin for RuneLite.

It provides a very simple REST api, with the following endpoints:

- `GET /{names}` where `{names}` is a comma-separated list of usernames of players. It returns a json list of objects of the cosmetic data for the players queried.  
- `PUT /{token}` where `{token}` is an API token given to trusted users. The body of the request should be a json object containing the cosmetic data of a player.

The format of cosmetic data is as follows:
```json
{
  "name": string,
  "head": integer,
  "body": integer,
  "cape": integer,
  "legs": integer,
  "neck": integer,
  "hand": integer,
  "ring": integer,
  "feet": integer,
  "weap": integer,
  "shld": integer,
  "jaws": integer,
  "hair": integer}
}
```
