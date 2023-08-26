### Global Guess (But really jus the US)
Christian Torralba


#### Concept
The whole project was supposed to be a game. Similar the GeoGuesser, it was a location guessing game, but instead of streetview,
I decided to go for a static satellite image guessing game, where it would provide a picture of a place on earth, and the user can click the location
on an embedded map to make a guess.


This project really took me for a ride, I had this whole idea planned out to make use of the NASA Earth API, but
unfortunately, it was down! I wanted to stick with this idea (mainly because I had no others), so I decided to check out other
apis that allow static satellite images of Earth. None other than google came to the rescue. Unfortunately I did have to input
my credit card info, but that's besides the point. 

#### Issues:
When I first started, it was going fairly smoothly, until I hit the first roadblock, which was the fact that the google request I was using only
returned a PNG picture, no json, no other information. So not only did I not have info on where the location of the picture was, I did not know how to deal with
PNGs in a `Reponse` type. I eventually got that all sorted out but ran into the next big issue, the picture would occasionally have the location of the image in a little title on the picture!
This totally destroyed my project at the time becuase the whole purpose was to guess the location on earth, and now the picture was telling the user exactly where it was.
Now, this may not be legal, but I looked into it, and found a quick way to just cut the bottom 20 pixels off of the bottom, please don't tell google!

The program as of now works, alright. There are major addition I wish I could have made before turning it in. I was planning 
on making a leaderboard, a score system, and many other things, but unfortunately I just didn't have the time.

The next major issue I had was finding random locations on earth. Since ~70% of earth is covered in water, I couldn't really just make up random latitude and longitude points.
I looked around and found a cool API that will return a json contain a BUNCH of information regarding a location on earth: link here (https://3geonames.org/)
The issue with this was that the majority of location it would give me we really really random, like in the middle of Siberia, Russia, and it gave me almost no locations 
of major citites. So, I found a JSON file with the top 1000 U.S citites by population, put that into my resources' directory, and just pull from that!

#### Conclusion
This project was really fun. I really learned a lot more about Rust and it's backend capabilities and it honestly blew my mind.
I took a Rust class last term and was still on the fence about it going into this one, but it's probably my new favorite language.
As for the project itself, I really wish I didn't spend the majority of time on things like getting the embedded map to work, and
figuring out how to color a marker on the google map. I defenitely think I could have done much more with this given the time.