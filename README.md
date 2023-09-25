# WorldGuess
Totally not [GeoGuessr](https://www.geoguessr.com)

### Author: Christian Torralba

#### Concept:
I always thought GeoGuessr was a cool idea for a game, so I wanted to copy it, but not exactly. This website is similar, but different. It provides a unique (in my opinion) approach to a guessing game by using static satellite images of locations on earth instead of a street view map. I have made use of the google maps API to obtain random satellite images of the top 1000 most populated cities. 

#### Building:
To build this project, you will need a few things. The database runs locally, using docker, and the routes are hosted on your local machine. The database defaults to port 5432 so if you already have a postgres instance running,  you may need to close it.


### Ideas:
- Leaderboard (In progress)
- Indivdual ranking (In progress)
