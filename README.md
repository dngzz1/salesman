# Multiple Traveling Salesmen with Capacities

## Statement of the Problem

Given $n$ cities and $k$ salesmen, each needing to visit exactly $n_1, ..., n_k$ cities (where $n_1 + ... + n_k = n$) and returning to the original city, what is the minimal distance traveled in total?

## One Traveling Salesman

We use the _simulated annealing algorithm_ to achieve this. Represent the salesman's journey by a mutable vector of cities of length $n$. In simulated annealing, we swap city $i$ and $j$ and check whether this gives a shorter distance. If it is shorter, we approve it, and if it is longer, we still approve it with a certain probability (controlled by a variable called temperature) which decreases exponentially as the algorithm proceeds. We eventually terminate the algorithm when the temperature drops below a certain threshold.

<p align="center">
<img src="images/read_me/one.svg" width="300" height="300">
</p>

## Multiple Salesmen: Example 1

We represent the solution by a mutable vector of cities of length $n$. The first $n_1$ entries represent the first salesman's journey, the next $n_2$ entries represent the second salesman's journey and so on.

I tried to modify the simulated annealing algorithm for the single traveling salesman by changing the distance function. However, this was not successful. Therefore, I decided to first cluster the cities.

The clustering algorithm can also use simulated annealing. The _cluster metric_ is
$$\mathcal{M}(C_1, C_2,...,C_k) = \sum_{i=1}^k \sum_{\mathbf{x}_j \in C_i} \lVert \mathbf{x}_j - \mathbf{z}_i \rVert$$

where $\mathbf{z}_i$ is the barycenter of cluster $i$. See [this](http://library.isical.ac.in:8080/jspui/bitstream/10263/5650/1/Clustering%20using%20simulated%20annealing%20with%20probabilistic%20redistribution-IJOPRAAI-15-2-2001-%20p%20269-285.pdf) article.

We then run the salesman algorithm on each cluster.

```rust
let num_points = 40;
    let seed = Some(42);
    let intensity = 10.0;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let salesmen_capacities = [20, 10, 10];

    let is_loop = true;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    let stringing = salesman::string::get_stringing(&points, &salesmen_capacities, &order, is_loop);
    let svg = salesman::utils::svg::create_svg(&stringing);
    svg::save("images/read_me/three_closed.svg", &svg).unwrap();
```

<p align="center">
<img src="images/read_me/three_closed.svg" width="300" height="300">
</p>

We can also run the algorithm for open strings, by setting `is_loop = false` instead:

<p align="center">
<img src="images/read_me/three_open.svg" width="300" height="300">
</p>

## Multiple salesmen: Example 2

40 cities, 5 salesmen, 8 cities each:

<p align="center">
<img src="images/read_me/five.svg" width="300" height="300">
</p>

## Multiple salesmen: Example 3

100 cities, 10 salesmen, 10 cities each:

<p align="center">
<img src="images/read_me/ten.svg" width="300" height="300">
</p>
