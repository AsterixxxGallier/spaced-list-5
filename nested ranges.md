valid:
indices: 0 1 2 3 4 5 6 7
         ( ( ) ( ( ) ) )
offsets: 7 1 1 3 1 1 3 7

invalid:
indices: 0 1 2 3 4 5 6 7
         ) ( ( ) ) ( ( )
offsets: x 3 1 1 3 x 1 1

invalid:
indices: 0 1 2 3 4 5 6 7
         ( ( ) ( ( ) ( )
offsets: x 1 1 x 1 1 1 1

invalid:
indices: 0 1 2 3 4 5 6 7
         ) ( ) ) ( ( ) (
offsets: x 1 1 x x 1 1 x

invalid:
indices: 0 1 2 3 4 5 6 7 8 9 A B C D E F
         ) ) ) ) ( ( ( ) ( ( ) ) ( ( ) (
offsets: x x x x x x 1 1 3 1 1 3 x 1 1 x

invalid:
indices: 0 1 2 3 4 5 6 7 8 9 A B C D E F
         ( ) ( ) ( ( ( ) ) ( ) ) ) ) ) ) 
offsets: 1 1 1 1 7 3 1 1 3 1 1 7 x x x x


Three approaches for open nested ranges:
1. Add placeholder nodes for unavailable range bounds (e.g. missing parentheses)
2. Only allow the last ending bound to be missing
3. Give up the even index = start; odd index = end requirement
With approaches 1 and 2, place sublists within a range. This can reliably be interpreted as nested ranges.








