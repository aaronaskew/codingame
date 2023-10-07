use std::collections::HashMap;
use std::io;

#[allow(unused_variables)]

//
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug)]
struct FactoryGraph {
    //HashMap <factory_a.id, Vec<factory_b.id, distance (weight)>>
    adjacency_list: HashMap<i32, Vec<(i32, i32)>>,
}

impl FactoryGraph {
    fn new() -> Self {
        FactoryGraph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, v1: i32, v2: i32, distance: i32) {
        self.adjacency_list
            .entry(v1)
            .or_insert(Vec::new())
            .push((v2, distance));
        // For undirected graphs, also add the reverse edge with the same distance
        self.adjacency_list
            .entry(v2)
            .or_insert(Vec::new())
            .push((v1, distance));
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Factory {
    id: i32,
    owner: i32, //-1, 0, 1 = them, none, me
    cyborgs: i32,
    production: i32, // [0,3]
}

impl Factory {
    fn new(id: i32) -> Self {
        Factory {
            id,
            owner: -9999,
            cyborgs: -9999,
            production: -9999,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct Troop {
    owner: i32,
    origin_factory: i32,
    dest_factory: i32,
    cyborgs: i32,
    turns_until_dest: i32,
}

impl Troop {
    fn new(
        owner: i32,
        origin_factory: i32,
        dest_factory: i32,
        cyborgs: i32,
        turns_until_dest: i32,
    ) -> Troop {
        Troop {
            owner,
            origin_factory,
            dest_factory,
            cyborgs,
            turns_until_dest,
        }
    }
}

//parse the entity info during input
fn parse_input(
    _factories: &mut HashMap<i32, Factory>,
    _troops: &mut Vec<Troop>,
    _entity_id: i32,
    _entity_type: &str,
    _arg_1: i32,
    _arg_2: i32,
    _arg_3: i32,
    _arg_4: i32,
    _arg_5: i32,
) {
    match _entity_type {
        "FACTORY" => {
            let f: &mut Factory = _factories.get_mut(&_entity_id).unwrap();
            f.owner = _arg_1;
            f.cyborgs = _arg_2;
            f.production = _arg_3;
        }

        "TROOP" => {
            let _t = Troop::new(_arg_1, _arg_2, _arg_3, _arg_4, _arg_5);
            _troops.push(_t);
        }

        _ => {}
    }

    // If entityType equals FACTORY then the arguments are:
    // arg1: player that owns the factory: 1 for you, -1 for your opponent and 0 if neutral
    // arg2: number of cyborgs in the factory
    // arg3: factory production (between 0 and 3)
    // arg4: unused
    // arg5: unused
    // If entityType equals TROOP then the arguments are:
    // arg1: player that owns the troop: 1 for you or -1 for your opponent
    // arg2: identifier of the factory from where the troop leaves
    // arg3: identifier of the factory targeted by the troop
    // arg4: number of cyborgs in the troop (positive integer)
    // arg5: remaining number of turns before the troop arrives (positive integer)
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    //define some globals
    let mut factory_graph = FactoryGraph::new();
    let mut factories: HashMap<i32, Factory> = HashMap::new();
    let mut troops: Vec<Troop> = Vec::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let factory_count = parse_input!(input_line, i32); // the number of factories
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let link_count = parse_input!(input_line, i32); // the number of links between factories

    eprintln!("factories: {factory_count}");
    eprintln!("links: {link_count}");

    for _i in 0..link_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let factory_1_id = parse_input!(inputs[0], i32);
        let factory_2_id = parse_input!(inputs[1], i32);
        let distance = parse_input!(inputs[2], i32);

        let fac1: Factory = Factory::new(factory_1_id);
        let fac2: Factory = Factory::new(factory_2_id);

        factories.insert(fac1.id, fac1);
        factories.insert(fac2.id, fac2);

        factory_graph.add_edge(fac1.id, fac2.id, distance);

        //TODO use dijkstra to get optimum, shortest paths
    }

    eprintln!("factories:\n{:#?}", factories);
    eprintln!("factory_graph:\n{:#?}", factory_graph);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32); // the number of entities (e.g. factories and troops)

        //reset _troops every turn
        troops.clear();

        for _i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let entity_id = parse_input!(inputs[0], i32);
            let entity_type = inputs[1].trim();
            let arg_1 = parse_input!(inputs[2], i32);
            let arg_2 = parse_input!(inputs[3], i32);
            let arg_3 = parse_input!(inputs[4], i32);
            let arg_4 = parse_input!(inputs[5], i32);
            let arg_5 = parse_input!(inputs[6], i32);
            parse_input(
                &mut factories,
                &mut troops,
                entity_id,
                entity_type,
                arg_1,
                arg_2,
                arg_3,
                arg_4,
                arg_5,
            );
        }

        eprintln!("troops:\n{:#?}", troops);
        eprintln!("factories:\n{:#?}", factories);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // Any valid action, such as "WAIT" or "MOVE source destination cyborgs"
        //println!("WAIT");

        do_turn(&mut factories, &mut troops);
    }
}

fn do_turn(factories: &mut HashMap<i32, Factory>, troops: &mut Vec<Troop>) {
    // Output for one game turn
    // The available actions are:
    // MOVE source destination cyborgCount: creates a troop of cyborgCount cyborgs at the factory source and sends that troop towards destination. Example: MOVE 2 4 12 will send 12 cyborgs from factory 2 to factory 4.
    // WAIT: does nothing.
    // If you try to move more cyborgs that there are in the source factory, then all the available units will be sent.

    //get list of target factories sorted by production
    factories.
}
