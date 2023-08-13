# sizzling1
A really recherche way to heat up your digs with your PC. Also generates tons of quirky images.

## Intro.
This is a project about fractal flames. Fractal flames is an algorithm (of sorts), which aims to generate aesthetically pleasing images, using a combination of math and balderdash. 
The math part is a result from the work of Barnsley et. al. on the subject of fractal geometry, called "Chaos Game Algorithm". As a matter of fact, "Chaos Game Algorithm" is a randomized version of something deterministic, which we better start with. Imagine you want to draw a Sierpinski triangle (because lots of nice things, like Star Destroyers and neatly cut watermelons are triangular). Sierpinski triangle is a fractal (which means that it's a quirky, self-similar set), which can be described as a **limit** of iteratively applying the following steps to "something on a plane":

1. Make it twice as small.
2. Make it twice as small and move half a unit up.
3. Make it twice as small and move half a unit to the right.

Now, the important thing to remember is that it's not important what you wanna make "twice as small" and so forth - as long as it's an object in 2D plane. For simplicity we gonna think that its a unit square. Sierpinski triangle can be drawn deterministically using these steps, in the following manner:
1. Take a square.
2. Make 3 copies of it, apply 1, 2 and 3 to each.
3. Rinse and repeat (this is why it's called **iterated** function system).

On each step this algorithm triples its input - meaning that on each iteration the amount of the "squares" we have to process grows exponentially, like $3^n$ in the case of Sierpinski triangle. Here $n$ is the number of iterations. If it were a fractal called "Sierpinski carpet" it would grow as $5^n$ - because the IFS of that fractal includes 5 transformations. Now, remember how it was mentioned, that Sierpinski triangle is a **limit** of applying something to something? This basically means that our n should be pretty significant, if we are to see anything hi-def in any way.

So, enter the "chaotic" version of this process. The "chaotic" version just says:
1. Take a point on a plane.
2. Randomly select a transformation (i.e. for Sierpinski triangle the choice is between "make it twice as and move to the side / move up / don't move")
3. Apply it to a point. Mark a location on plane where point landed after being "transformed".
4. Repeat 1-3 like a million times, still **much** less computation than in the deterministic version.

And eventually this will yield an image of the Sierpinski triangle. Now, why in the world would we want that?


Enter the balderdash part of the fractal flames algorithm. To explain it, lets start by pointing out that all of the transforms we used previously are called **affine transforms** (for instance the steps we apply to something on a plane to get the fabled triangle). They have the following nice property: for an affine transform it's relatively easy to verify whether it's contractive or not. "Contractivenss" is a general property of any transform, which tells us, if a transform (also sometimes called mapping) makes points in a set closer to each other. In our case we are dealing with the following sets (when drawing a triangle deterministically):
1. Unit square.
2. 3 smaller squares (points in a square became closer).
3. 9 even smaller squares (even closer)
4. ... bazillion squares (all of the points composing each individual square are basically at the verge of starting a thermonuclear synthesis, had they been protons, or something).

Contractivess is a necessary property that a transform should posses, if the chaos game process is to converge. Unfortunately contractiveness of an arbitrary mapping is mighty hard to verify. And that's why we are in the part of the fractal flames algorithm, called "balderdash". The thing is, that by using only nice, affine transforms, we will not get anything remotely artistic (in the broadest definition of a term). To achieve any kind of braintickling result, we'd need to introduce arbitrary transforms applied at each iteration. However in doing so, we are throwing out the window any guarantee that our chaos game will converge. The rendition becomes a game of chance, vaguely described by the following:
1. Take some regular IFS, made out of affine transforms.
2. Choose one at random. Apply it to a point.
3. Apply a linear combination of some arbitrary functions (I call 'em mutators), to the result of the nice affine transform. Remember where it landed.
4. Rinse and repeat like a million times. Maybe you'll get a lost Picasso, maybe a black screen (a lost Malevich that is).

Now, with a decent number of different mutators there is a huge space of possible images to explore, even for one regular IFS, by going through all of the possible combinations of these mutators. Also, there is a huge number of nice IFS that can be constructed from sampling affine transforms from a bunch of preset IFS (the IFSes for some well known fractals like Sierpinski triangle, Barnsley fern, Ice crystal and so on). Multiply these two numbers together and the result will be beyond any reasonable comprehension - in a way it would be poetic.

## Current state of the project
Currently the project doesn't have the core functionality implemented. Initially I focused on implementing adaptive kernel density estimation algorithm, since I really wanted to see if it's gonna be any improvement over the  naive histogram-based estimation. It kinda is, however I think the whole idea of KDE is nipped in the bud by the fact that there are only so many pixels on the screen.
The complete core functionality would include:
1. The "search mode" - running in this mode, the algorithm will sift through the possible images searching for those satisfying some criterion. The density criterion for now seems like a good choice - it's always nice to have a lot of non-zero pixels in the image. Additionally it'd probably be nice to have some ML gizmo here - the balderdash part of the fractal flames seems to be quite fitting to these undisciplined algorithms. 
2. The "hi-def render mode" - will allow to render the selected fractal in high def (the search mode will save only thumbnails). Interestingly enough, any image can be described by the two numbers - the index of the permutation of all the permutations out of the set of all possible mutators, and out of the set of all possible affine transforms. In this project these sets are of fixed size, with a fixed order.
3. Utilizing all the cores, to achieve max heat output possible.

## Note
This is a learning project. I use it to learn Rust, hence there are a lot of comments explaining how something works and how I don't know why something works. Additionally there is a lot of seemingly needless code, which was written just to demonstrate some concept.

## Why is it considered a "way to heat up your digs"?
I spent quite some time dwelling on the fractal flames algorithm / fractal art. On the one hand it's an incredibly interesting area, which requires a lot of research, and implementing a lot of inherently non-trivial, refreshing things. It's a nice brain-teaser. On the other hand - it's a completely unmanageable, non-deterministic mess, which **might** produce something decent-looking, but not in any way worth the electric bill it'll most probably incur. So, I feel pretty frustrated with this project and at the same time fairly motivated by it, and to resolve the frustration I turned it into a sort of joke. Also, living without central heating I can attest that it does indeed make a room warmer.