use petgraph::Graph;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;

fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
  }

fn main() {
    // 入力データ
    let input_data = vec![
        vec![0, 1, 2],
        vec![1, 0, 3],
        vec![2, 3, 0],
    ];

    // グラフを作成
    let mut graph = Graph::<(), i32>::new();
    let nodes_count = input_data.len();

    let nodes: Vec<NodeIndex> = (0..nodes_count).map(|_| graph.add_node(())).collect();

    for i in 0..nodes_count {
        for j in 0..nodes_count {
            let weight = input_data[i][j];
            if weight > 0 {
                graph.add_edge(nodes[i], nodes[j], weight);
            }
        }
    }

    // ダイクストラのアルゴリズムを適用
    let start_node = nodes[1];
    let distances = dijkstra(&graph, start_node, None, |e| *e.weight());

    // 結果を出力
    for (node, distance) in &distances {
        println!("Node: {}, Distance: {:?}", node.index(), distance);
    }

    // 型確認
    println!("{}",type_of(&distances));

    // キーを指定して値を取得
    let node = nodes[0];

    // キーを指定して値を取得
    if let Some(distances) = distances.get(&node) {
        println!("Node's type: {}", type_of(distances));
        println!("Node's score: {}", distances);
    }
}