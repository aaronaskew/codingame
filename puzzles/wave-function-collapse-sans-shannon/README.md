This is not ASCII art! For this in/out puzzle pixels are not available, so instead we will use characters to represent colours. Imagine each character as a pixel of a specific colour.

Procedural generation via wave function collapse (WFC) is a way of generating content based off of a few "prototypical" examples". Given an image prototype of something as small as 16x16 pixels, WFC can make a 30x300 image with very similar structures drawn from the single prototype. This is not limited to flowers or maps but anything with small repeating patterns. After completing this puzzle you will be able to use this same code to generate a wide variety of content, from dungeon maps, to pixel art.

See cover art and
https://github.com/mxgmn/WaveFunctionCollapse/blob/master/images/wfc.gif

The task is to read in a prototype image as well as a partially filled solution. Use the prototype image and WFC (kernel size of 3) to fill in the missing parts of the solution. The prototype and partial solution will both be printable characters. The missing parts are designated with the ? character.

Example

Prototype
12 x 8
+----------+
|          |
|  *       |
| \|    *  |
|  |/   |/ |
|  |    |  |
| \|    |  |
+----------+

Partial          Expected
Solution          Output
+----------+    +----------+
|??????????|    |          |
|??????????|    |  *       |
|??????????|    | \|       |
|?????  ?  |    |  |/   *  |
|? ? ??\| ?|    |  |   \|  |
| ??    ?? |    | \|    |/ |
|  ?/   ?  |    |  |/   |  |
|??????\???|    |  |   \|  |
+----------+    +----------+

Partial                     Expected
Solution                     Output
+--------------------+   +--------------------+
|     ???????????????|   |                    |
|  *         *       |   |  *         *       |
| \|        \|   ????|   | \|        \|    *  |
|  |/  ????  ?/   ?/ |   |  |/        |/   |/ |
|  |  ?????? ?    ?  |   |  |         |    |  |
|  |         ?    ?  |   |  |         |    |  |
|  |  ?????  ?    ?  |   |  |    *    |    |  |
|  |   ?|??  ?    ?  |   |  |    |/   |    |  |
| \??   |?  \|?   ?? |   | \|    |   \|    |  |
+--------------------+   +--------------------+


NOTE There are three features to this puzzle that make this easier than a full WFC implementation. First, the border will always be included in the partial solution and it will be identical to the prototypes border. Second, the procedure outlined by mxgmn (link below) includes using Shannon Entropy. Skip that step for this puzzle and only collapse states that are certain. Third, no reflection or rotation is used in this puzzle.

RESOURCES
Github repo https://github.com/mxgmn/WaveFunctionCollapse
youtube video https://www.youtube.com/watch?v=fnFj3dOKcIQ
and https://www.youtube.com/watch?v=t1O0_yHe-6Y
and suggested by ninjadip https://www.youtube.com/watch?v=2SuvO4Gi7uY

REPRODUCIBILITY In order to achieve full reproducibility of the test and validation output, process the image in these steps. These steps will make more sense once you've read the background resources.
1 * Calculate possible 3x3 patches. A 5x6 prototype would generate 12 patches
2 * Constrain patches from left to right, then from top to bottom
3 * After a patch has been constrained, constrain all symbols that are covered by the 3x3 patch
4 * If there are still uncertain symbols goto step 2

A symbol is something like "#" or "|".

Constraining in step 2 means reduce the list of possible patches to only those patches that are possible given the symbols in the blocks they cover. For example, if all the blocks are unknown (can be any symbol) except the centre block is known to be either '#' or "|" and the lower right is known to be '*' or '/', then reduce the possible patches to be only patches that have the centre as either '#' or '|' and the lower right to be '*' or '/'.

Constraining in step 3 means if, for example, all remaining possible overlapping 3x3 patches are the following 2
Patch   Patch
#..    .*.
#..    .|.
###    ###

then the blocks are reduced to these lists
[#.] [.*] [.]
[#.] [.|] [.]
[#]  [#]  [#]


We now know for certain that the bottom is all '#' and the right side is '.'
And the other positions are constrained to [#.] [.*] and [.|]

HINT The hint is ROT13 encoded so it won't spoil your fun. This site can decode them for you https://rot13.com

Uvag 1 : "Keep two 2d datasets, one for the remaining legal symbols for each square, one for the remaining legal 3x3 patches centered at each square."

Uvag 2 : "For debugging it is helpful to replace the "?" with the number of possible symbols (or possible patches). Each iteration of the constraints should results in a reduction some of the number of possible symbols and patches."

Uvag 3 : "The first test case has 47 unique patches"

Uvag 4 : "The edges are important. Include the edges in the creation of and use of patches"


## Algorithm
1. Read the input bitmap and count NxN patterns.
    1. (optional) Augment pattern data with rotations and reflections.

2. Create an array with the dimensions of the output (called "wave" in the source). Each element of this array represents a state of an NxN region in the output. A state of an NxN region is a superposition of NxN patterns of the input with boolean coefficients (so a state of a pixel in the output is a superposition of input colors with real coefficients). False coefficient means that the corresponding pattern is forbidden, true coefficient means that the corresponding pattern is not yet forbidden.

3. Initialize the wave in the completely unobserved state, i.e. with all the boolean coefficients being true.

4. Repeat the following steps:
    1. Observation:
        1. Find a wave element with the minimal nonzero entropy. If there is no such elements (if all elements have zero or undefined entropy) then break the cycle (4) and go to step (5).
        2. Collapse this element into a definite state according to its coefficients and the distribution of NxN patterns in the input.
    2. Propagation: propagate information gained on the previous observation step.

5. By now all the wave elements are either in a completely observed state (all the coefficients except one being zero) or in the contradictory state (all the coefficients being zero). In the first case return the output. In the second case finish the work without returning anything.