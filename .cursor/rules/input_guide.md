# Input Function Usage Guide
When reading input values:

1. For single value:
   - Use `read_value(read_line(reader))` function
   Example: `let n: usize = read_value(read_line(reader));`

2. For multiple values in one line:
   - Use `read_values_as!` macro when reading multiple different types
   Example: `let (n, m): (usize, i32) = read_values_as!(read_line(reader), usize, i32);`

3. For n values of the same type:
   - Use `read_n_values(reader, n)` function
   Example: `let arr: Vec<i32> = read_n_values(reader, n);`

4. For reading a matrix:
   - Use `read_map(reader, rows, cols)` function
   Example: `let matrix: Vec<Vec<i32>> = read_map(reader, n, m);`
