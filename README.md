# Latus Nearest Neighbor Search

Latus is a Rust library for approximate nearest neighbor search, which aims to integrate filtering directly into ANN queries in order to enable you to query subsets of the indexed points with re-indexing or post-filtering. For example, if the indexed points represent movies and you'd like to query for the nearest neighbors within a specific genre, you can do that without needing to build a separate index per genre or filtering out movies from the wrong genres out of the returned results.

("Latus" is Latin for "side"; integrated filtering allows you to easily see many sides or facets of your data. It's also a pun on "lattice", one of the quantization methods used in nearest neighbor search. And since this is a Rust project, it's also short for [_Neopetrolisthes maculatus_](https://en.wikipedia.org/wiki/Neopetrolisthes_maculatus), the spotted porcelain crab—which is an anemone-dwelling filter feeder and also secretly a lobster.) 
