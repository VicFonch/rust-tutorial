mod ml_data;

use crate::ml_data::read_ml_json;
use crate::ml_data::MLDataContainer;
use crate::ml_data::Node;
use std::path::Path;


pub fn correlation_per_node(node_xx:&Node, node_other:&Node) -> f64 {
    
    let mut total:f64 = node_xx.a.len() as f64 - 5.0;
    let mut count:f64 = 0.0;

    let dont_compare = vec!["WH", "LT", "TP", "HT"];

    for key in node_xx.a.keys(){
        if node_other.a.contains_key( key ) && !dont_compare.contains(&&key[..]) && node_xx.a[key] == node_other.a[key]{
            count += 1.0;
        }
    }

    return count/total;
}

fn main(){

    let path = Path::new("resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");
    let data = read_ml_json(&path);

    let mut xx_vec: Vec<Node> = Vec::new();

    let mut cont = 0;
    for i in 0..data.element_statistics.nodes.len() {
        if data.element_statistics.nodes[i].a.contains_key("XX"){
            xx_vec.push(data.element_statistics.nodes[i].clone());
            cont += 1;
        }
    }
    
    println!("El numero de nodos con XX es {}", cont);

    let mut nodes_vec =  Vec::new();
    for i in 0..data.element_statistics.nodes.len() {
        nodes_vec.push(data.element_statistics.nodes[i].clone());
    }

    let mut correlations:Vec<f64> = vec![0.0; nodes_vec.len()];
    
    let mut i = 0;
    for j in 0..cont{
        for node in &nodes_vec{
            correlations[i] = correlation_per_node(&xx_vec[j], &node);
            i += 1;
        }
    }

    for j in 0..cont{
        println!("El vector de correlacion del nodo {} que contiene a XX es", j);
        for cor in &correlations{
            println!("{}",cor);
        }
    }
    
}