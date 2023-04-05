use petgraph_test::{Constraints, Node, ECTA};

pub fn fig_1_d() {
    let mut ecta: ECTA<&'static str, ()> = ECTA::new();
    let end_node = ecta.add_node(Node::new(), vec![]);

    let q1 = ecta.add_node(
        Node::new(),
        vec![
            ("a", None, vec![end_node]),
            ("b", None, vec![end_node]),
            ("c", None, vec![end_node]),
        ],
    );

    let q2 = ecta.add_node(Node::new(), vec![("f", None, vec![q1])]);

    let _root_node = ecta.add_node(
        Node::new(),
        vec![(
            "+",
            Some(Constraints::new(vec![(vec![0, 0], 0), (vec![0, 1], 0)], ())),
            vec![q2, q2],
        )],
    );
    println!("{}", ecta.get_dot());
}

pub fn fig_2() {
    let mut ecta: ECTA<&'static str, ()> = ECTA::new();

    let end_node = ecta.add_node(Node::new(), vec![]);

    let end_node2 = ecta.add_node(Node::new(), vec![]);

    let x_typ = ecta.add_node(Node::new(), vec![("int", None, vec![end_node])]);
    let y_typ = ecta.add_node(Node::new(), vec![("char", None, vec![end_node2])]);

    let scalar = ecta.add_node(
        Node::new(),
        vec![("x", None, vec![x_typ]), ("y", None, vec![y_typ])],
    );

    let bool = ecta.add_node(Node::new(), vec![("bool", None, vec![end_node])]);
    let char = ecta.add_node(Node::new(), vec![("char", None, vec![end_node2])]);
    let int = ecta.add_node(Node::new(), vec![("int", None, vec![end_node2])]);

    let unary = ecta.add_node(
        Node::new(),
        vec![
            ("f", None, vec![bool, bool]),
            ("g", None, vec![int, bool]),
            ("h", None, vec![char, int]),
        ],
    );

    ecta.intersection(scalar, unary);

    let _root_node = ecta.add_node(
        Node::new(),
        vec![(
            "app",
            Some(Constraints::new(vec![(vec![0, 0], 0), (vec![1, 0], 0)], ())),
            vec![unary, scalar],
        )],
    );

    println!("{}", ecta.get_dot());
}
/*
pub fn fig_6_union() {
    let mut ecta: ECTA<&'static str, ()> = ECTA::new();

    let end_node = ecta.add_node(Node::new(), vec![]);

    let f_targ = ecta.add_node(Node::new(), vec![("bool", (), vec![end_node])]);

    let f_ret = ecta.add_node(Node::new(), vec![("char", (), vec![end_node])]);

    let g_targ = ecta.add_node(Node::new(), vec![("int", (), vec![end_node])]);

    let g_ret = ecta.add_node(
        Node::new(),
        vec![
            ("int", (), vec![end_node]),
            ("bool", (), vec![end_node]),
            ("char", (), vec![end_node]),
        ],
    );

    let n_1 = ecta.add_node(
        Node::new(),
        vec![
            ("f", (), vec![f_targ, f_ret]),
            ("g", (), vec![g_targ, g_ret]),
        ],
    );

    let end_node = ecta.add_node(Node::new(), vec![]);

    let g_targ = ecta.add_node(
        Node::new(),
        vec![("bool", (), vec![end_node]), ("int", (), vec![end_node])],
    );

    let g_ret = ecta.add_node(
        Node::new(),
        vec![("int", (), vec![end_node]), ("char", (), vec![end_node])],
    );

    let h_targ = ecta.add_node(Node::new(), vec![("char", (), vec![end_node])]);

    let h_ret = ecta.add_node(Node::new(), vec![("char", (), vec![end_node])]);

    let n_2 = ecta.add_node(
        Node::new(),
        vec![
            ("g", (), vec![g_targ, g_ret]),
            ("h", (), vec![h_targ, h_ret]),
        ],
    );

    ecta.union(n_1, n_2);

    println!("{}", ecta.get_dot());
}

pub fn fig_6_intersection() {
    let mut ecta: ECTA<&'static str, ()> = ECTA::new();

    let end_node = ecta.add_node(Node::new(), vec![]);

    let f_targ = ecta.add_node(Node::new(), vec![("bool", (), vec![end_node])]);

    let f_ret = ecta.add_node(Node::new(), vec![("char", (), vec![end_node])]);

    let g_targ = ecta.add_node(Node::new(), vec![("int", (), vec![end_node])]);

    let g_ret = ecta.add_node(
        Node::new(),
        vec![
            ("int", (), vec![end_node]),
            ("bool", (), vec![end_node]),
            ("char", (), vec![end_node]),
        ],
    );

    let n_1 = ecta.add_node(
        Node::new(),
        vec![
            ("f", (), vec![f_targ, f_ret]),
            ("g", (), vec![g_targ, g_ret]),
        ],
    );

    let end_node = ecta.add_node(Node::new(), vec![]);

    let g_targ = ecta.add_node(
        Node::new(),
        vec![("bool", (), vec![end_node]), ("int", (), vec![end_node])],
    );

    let g_ret = ecta.add_node(
        Node::new(),
        vec![("int", (), vec![end_node]), ("char", (), vec![end_node])],
    );

    let h_targ = ecta.add_node(Node::new(), vec![("char", (), vec![end_node])]);

    let h_ret = ecta.add_node(Node::new(), vec![("char", (), vec![end_node])]);

    let n_2 = ecta.add_node(
        Node::new(),
        vec![
            ("g", (), vec![g_targ, g_ret]),
            ("h", (), vec![h_targ, h_ret]),
        ],
    );

    ecta.intersection(n_1, n_2);

    println!("{}", ecta.get_dot());
}
*/

fn main() {
    /* fig_1_d(); */

    fig_2();
    /* fig_6_union(); */
    /* fig_6_intersection(); */

    /*     let root_node = g.add_node(Node::new());
    let next_node = g.add_node(Node::new());
    assert!(root_node != next_node);

    g.add_edge(next_node, root_node, "App".into());
    println!("{}", Dot::with_config(&g, &[])) */
}
