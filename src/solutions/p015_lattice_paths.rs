euler::solution!(p015, "Lattice paths", 137846528820);

pub fn p015() -> usize {
    let size = 21;

    let mut num_routes = vec![0; size * size];

    // Initialize the top and left edges - each vertex has exactly one
    // way to get to it.
    for x in 0..size {
        num_routes[x] = 1;
        num_routes[x * size] = 1;
    }

    for y in 1..size {
        for x in 1..size {
            // Vertex (x, y) could come from (x-1, y) or (x, y-1).
            let pos1 = x - 1 + y * size;
            let pos2 = x + (y - 1) * size;
            num_routes[x + y * size] = num_routes[pos1] + num_routes[pos2];
        }
    }

    num_routes[size * size - 1]
}
