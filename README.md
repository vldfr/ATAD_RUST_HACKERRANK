# ATAD_RUST_HACKERRANK

## Summary
Easy Challenges(12):
- 1-10
- 13
- 15 

Medium Challenges(3):
- 11
- 12
- 14


## Explanations:
Each list item corresponds to the appropriately numbered file.
1. Just prints a sum of two numbers
2. Iteration over an array and adding to the sum.
3. compares 3 pairs of scores between two people and gives one of the people a win or nothing in case of a draw.
4. same as in 2, only I use i64 for bigger integers.
5. I only need to sum diagonals, so in one loop I do the iteration of both diagonals at once.
6. quite self-explanatory, special thing is that I cast f32 so that calculations are correct and {:.6} so that it prints 6 decimals.
7. Prints spaces and # in decreasing and increasing ammounts.
8. I created the combinations of 5 elements by 4 by hand and then for each of the combinations I determined if it was smaller than the minimum or bigger than the maximum and adjusted those values accordingly.
9. I keep track of the maxUnit: if it reappears I increment, if new is found, I reset the count.
10. I removed the AM/PM part, split by ":", converted to numbers and then handled the edge-cases for 12AM/PM.
11. Quite difficult, I made an array of moves that generates all sum components for x and y, so for example: final_pos_x = current_x+ sum_component. Then used a queue for breadth-first-search. every time a new valid non-final move is discovered it is added to the queue with a step increase. in case it's the final move, I return, because it's the first move, with the least amount of steps that accomplishes the goal.
12. I failed at first to implement a simple version due to time constraints, so I had to implement it using BTreeSet, which has the range function that gives a range based on a range of values, not a range of indexes. That Way, I could implement it with a single for loop, and reduced the complexity.
13. I sorted the arrays to be able to iterate over them in a simple way, and every time there was a difference between them I didn't increment the "a" array, because it was missing the number, checked if I already have it in missing numbers, if not I pushed it, then I continued with the "b" array
14. I just double-iterated over the array decreasingly, to find element pairs that sum up to k.
15. Found a nice trick to sum slices of the array and established the left and right slices of each item and then iterated over each item to see if it has equal left and right sums.