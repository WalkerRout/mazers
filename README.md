# mazers
Quickly find the shortest path in a maze

Utilizes generative recursion to generate valid next positions based off of current position.
The entire algorithm is tail recursive, meaning the compiler is able to optimize it iterations.
For debug builds, a dynamic stack allocator is used to avoid crashing when debugging. It is not
necessary in release mode, as the compiler is able to properly evaluate the tail-recursive calls.
