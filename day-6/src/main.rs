use trees::{tr,Node,Tree};

fn main() {
        let mut data = get_data();
        data.pop();

        let tree = create_tree(data);
        // println!("Full Tree: {:?}", tree);
        // let count = orbital_count(&tree);
        // println!("Orbital Count: {:?}", count );

        println!("getting LCA");
        let size = get_lca("YOU".to_string(),"SAN".to_string(),tree);
        println!("distance {:?}",size );

}

fn create_tree(data: Vec<String>) -> Tree<String>{

    let root_str = get_root_node_string(data.clone());
    let mut tree = tr(root_str);
    add_roots(data.clone(),&mut tree);

    tree
}

fn add_roots(data: Vec<String>,node: &mut Node<String>) {

    for line in &data {
        let left = &line[..3];
        let right = &line[4..];
        
        if left.to_string() == node.data {
            let new_node = tr(right.to_string());
            node.push_back(new_node);        
        }
    }

    for mut child in node.iter_mut() {
        add_roots(data.clone(),&mut child);
    }


}

fn get_root_node_string(data: Vec<String>) -> String {
    let mut found_parent = false;
    let line1 = data[0].clone();
    let mut current_root = &line1[..3];

    while !found_parent{
        found_parent = true;
        for line in &data {
            let left = &line[..3];
            let right = &line[4..];

            if right == current_root{
                found_parent = false;
                current_root = left.clone();//to_string();
            }
        }
    }

    current_root.to_string()
}


fn orbital_count(node: &Node<String>) -> i32 {
    let mut count = 0;
    for child in node.iter(){
        count += orbital_count(child);
    }
    count += (node.node_count()-1) as i32;

    count
}

fn get_lca(node1: String,node2:String, tree: Tree<String>) -> usize{
    // let visits = tree.root().bfs().iter.collect::<Vec<_>>();
    // for visit in visits{
        // println!("{:?}",visit.root());
    // }

    // tr("test".to_string())
    let mut path1 = Vec::new();
    get_path(node1,&tree, &mut path1);
    // println!("{:?}", path1);

    let mut path2 = Vec::new();
    get_path(node2,&tree, &mut path2);
    // println!("{:?}", path2);

    let mut on1 = path1.pop();
    let mut on2 = path2.pop();
    let common;

    loop {
        let n1 = path1.pop();
        let n2 = path2.pop();

        if n1 != n2 {
            common = on1.unwrap();
            break;
        }

        on1 = n1;
        on2 = n2;
    }

    // println!("{:?}", path1);
    // println!("{:?}", path2);
    let size = path1.len() + path2.len();
    // println!("{:?}", size);
    // println!("{:?}",common);

    // let node; 
    // search_node(common.to_string(),&tree,node);

    size
}

// fn search_node(name: String,node: &Node<String>, search: &Node<String>) -> bool {
//     if node.data == name {
//         println!("{:?}",node );
//         search = node.clone();
//         return true;
//     }

//     for child in node.iter() {
//         return search_node(name.clone(),child,&mut search);
//     }

//     false
// }

fn get_path(name: String, node: &Node<String>,mut path: &mut Vec<String>) -> bool {
    if node.data == name {
        path.push(node.data.clone());
        return true;
    }

    for child in node.iter() {
        if get_path(name.clone(),child,&mut path) == true {
            path.push(node.data.clone());
            return true;
        }
    }
    false
}


fn get_data() -> Vec<String> {
    include_str!("data2.txt").split("\n")
    .map(|i| {
        if i == "" {return "".to_string()}
        let mut i = i.parse::<String>().unwrap();
        i.truncate(i.len() - 1);
        i
    })
    .collect()
}