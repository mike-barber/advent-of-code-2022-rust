This definitely needed improvement. The original solution converged in about 80 minutes, but does produce the correct answer at least.

After submitting and a lot of further thought about the problem, I constrained the states by limiting the number of robots produced. There's no use having 20 ore-producing robots when the most we can ever use is, say, 3. I also re-wrote the DFS to use a priority queue to explore the states with the highest potential first, and finally added Rayon for good measure.

This brings the total runtime down to around 6-7 seconds. Further optimisations are definitely possible.
