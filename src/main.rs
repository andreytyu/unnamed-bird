//#[macro_use]
extern crate osmpbfreader;
extern crate petgraph;
use petgraph::Graph;
extern crate log;
extern crate env_logger;

fn count<F: Fn(&osmpbfreader::Tags) -> bool>(filter: F, filename: &std::ffi::OsStr) {

    

    let r = std::fs::File::open(&std::path::Path::new(filename)).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);
    let objs = pbf.get_objs_and_deps(|obj| filter(obj.tags())).unwrap();

    let mut osm_graph = Graph::<&i64, &i64>::new();
    //let mut nb_nodes = 0;
    //let mut sum_lon = 0.;
    //let mut sum_lat = 0.;
    //let mut nb_ways = 0;
    //let mut nb_way_nodes = 0;
    let mut nb_rels = 0;
    //let mut nb_rel_refs = 0;
    for obj in objs.values() {
        //info!("{:?}", obj);
        match *obj {
            osmpbfreader::OsmObj::Node(ref node) => {
                
                osm_graph.add_node(&node.id.0);
                
            }
            osmpbfreader::OsmObj::Way(ref way) => {
                //nb_ways += 1;
                //nb_way_nodes += way.nodes.len();
                println!(
                "way {:?}.",
                way.nodes);
                for vertice in &way.nodes {
                        println!("node {:?}.",vertice.0 + 2)
                }
                break
            }
            osmpbfreader::OsmObj::Relation(ref rel) => {
                nb_rels += 1;
                //nb_rel_refs += rel.refs.len();
            }

            
        }
    }

    //println!(
    //            "graph {:?}.",
    //            osm_graph);

}

fn main() {
    //env_logger::init().unwrap();
    let args: Vec<_> = std::env::args_os().collect();
    match args.len() {
        3 => {
            let key = args[2].to_str().unwrap();
            println!(
                "counting objects with \"{}\" in tags and their depedencies...",
                key
            );
            count(|tags| tags.contains_key(key), &args[1]);
        }
        4 => {
            let key = args[2].to_str().unwrap();
            let val = args[3].to_str().unwrap();
            println!(
                "counting objects with tags[\"{}\"] = \"{}\" and their depedencies...",
                key, val
            );
            count(|tags| tags.contains(key, val), &args[1]);
        }
        _ => println!("usage: count filename key [value]",),
    };
}
