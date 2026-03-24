use arrow::array::Int64Array;

fn main() {
    println!("Hello, world!");
}

fn ch1() {
    use rand::RngExt;

    struct Row {
        id: i64,
        cost: f64,
        cost_components: Vec<f64>,
    }

    const N: usize = 10;
    let mut rng = rand::rng();
    let rows: Vec<Row> = (0..N)
        .map(|i| Row {
            id: i as i64,
            cost: rng.random_range(0.0..100.0),
            cost_components: (0..rng.random_range(0..10))
                .map(|_| rng.random_range(0.0..1000.0))
                .collect(),
        })
        .collect();

    let mut ids = Vec::with_capacity(rows.len());
    let mut costs = Vec::with_capacity(rows.len());
    let mut components = Vec::with_capacity(rows.len());

    for r in rows {
        ids.push(r.id);
        costs.push(r.cost);
        components.push(r.cost_components);
    }
}
