# Notes

* The player is the 5th player (epsilon)
* The 4 hashes indicate where walls are I think
* Trying to test the 'C' command, and the site is not working :(
* 'C' doesn't move player, probably due to wall
* trying 'D'
* 'D' doesn't move player, probably due to wall
* trying 'E'
* 'E' is left
* This is Pac Man!

## Things to do

* [ ] Implement game logic
  * [ ] pathfinding (depth or breadth first search?)
    * [ ] avoid ghosts
    * [ ] assuming no power pellets
    * [ ] test for screen wrapping (doubtful)

* [x] Codify the walls

* [x] Decipher commands
  * up: 'C',
  * down: 'D',
  * left: 'E',
  * right: 'A',
  * stay_put: 'B',

* [x] Decipher wall characters
  * wall0 = above
  * wall1 = right
  * wall2 = below
  * wall3 = left
