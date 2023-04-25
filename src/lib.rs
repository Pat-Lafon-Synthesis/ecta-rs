use std::fmt::Debug;
use std::hash::Hash;
use std::{collections::HashMap, fmt::Display};

use itertools::{GroupingMapBy, Itertools};
use petgraph::{
    dot::Dot,
    stable_graph::{EdgeReference, NodeIndex, StableGraph},
    visit::{EdgeRef, IntoEdgeReferences},
};

pub type ECTANode = NodeIndex;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Edge<T, EQ: PathConstraintStringify> {
    pub data: T,
    constraints: Option<Constraints<EQ>>,
    pub edge_num: u8, // Essentially the order of args
    pub nodeidx: ECTANode,
}

impl<T, EQ: PathConstraintStringify> Edge<T, EQ> {
    pub fn new(
        data: T,
        constraints: Option<Constraints<EQ>>,
        edge_num: u8,
        nodeidx: ECTANode,
    ) -> Self {
        Self {
            data,
            edge_num,
            constraints,
            nodeidx,
        }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Edge<U, EQ> {
        Edge {
            data: f(self.data),
            constraints: self.constraints,
            edge_num: self.edge_num,
            nodeidx: self.nodeidx,
        }
    }
}

impl<T: Display, R: PathConstraintStringify> Display for Edge<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = &self.constraints {
            write!(f, "{}_{} : {}", self.data, self.edge_num, c.to_string())
        } else {
            write!(f, "{}_{}", self.data, self.edge_num)
        }
    }
}

pub struct Node {}

impl Node {
    pub fn new() -> Self {
        Node {}
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct ECTA<S, EQ: PathConstraintStringify> {
    g: StableGraph<Node, Edge<S, EQ>>,
    map: HashMap<Vec<Edge<S, EQ>>, ECTANode>,
    pub empty_node: ECTANode,
}

pub trait ECTATrait: Debug + Eq + Hash + Clone {}

impl<T: Debug + Eq + Hash + Clone> ECTATrait for T {}

impl<S: ECTATrait, EQ: ECTATrait + PathConstraintStringify> Default for ECTA<S, EQ> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: ECTATrait, EQ: ECTATrait + PathConstraintStringify> ECTA<S, EQ> {
    pub fn new() -> Self {
        let mut g = StableGraph::new();
        let mut map = HashMap::new();
        let empty_node = g.add_node(Node::default());
        map.insert(vec![], empty_node);

        Self { g, map, empty_node }
    }

    pub fn get_edges<'a>(&'a self, n: ECTANode) -> impl Iterator<Item = &Edge<S, EQ>> + 'a + Clone {
        self.g.edges(n).map(|e| e.weight())
    }

    pub fn add_node(
        &mut self,
        n: Node,
        edge_builder: Vec<(S, Option<Constraints<EQ>>, Vec<ECTANode>)>,
    ) -> ECTANode {
        let edges: Vec<Edge<S, EQ>> = edge_builder
            .into_iter()
            .flat_map(|(data, constraints, nidx)| {
                nidx.into_iter()
                    .enumerate()
                    .map(move |(edge_num, nodeidx)| Edge {
                        data: data.clone(),
                        constraints: constraints.clone(),
                        edge_num: edge_num as u8,
                        nodeidx,
                    })
            })
            .collect();

        self.add_node_inner(n, edges)
    }

    fn add_node_inner(&mut self, n: Node, edges: Vec<Edge<S, EQ>>) -> ECTANode {
        *self.map.entry(edges.clone()).or_insert_with(|| {
            let idx = self.g.add_node(n);
            edges.into_iter().for_each(|e| {
                self.g.add_edge(idx, e.nodeidx, e);
            });
            idx
        })
    }

    pub fn get_dot(&self) -> Dot<&StableGraph<Node, Edge<S, EQ>>> {
        Dot::with_config(&self.g, &[])
    }

    // Currently this just takes all edges from both nodes... But note that
    pub fn union(&mut self, n1: ECTANode, n2: ECTANode) -> ECTANode {
        let edges = self
            .g
            .edges(n1)
            .chain(self.g.edges(n2))
            .map(|e| e.weight().clone())
            .collect();
        self.add_node_inner(Node::default(), edges)
    }

    pub fn intersection(&mut self, n1: ECTANode, n2: ECTANode) -> ECTANode {
        match (n1, n2) {
            (n1, n2) if n1 == n2 => n1,
            (n1, _) | (_, n1) if n1 == self.empty_node => self.empty_node,
            (n1, n2) => {
                let candidate_edges: Vec<_> = self
                    .g
                    .edges(n1)
                    .map(|e| e.weight())
                    .cartesian_product(self.g.edges(n2).map(|e| e.weight()))
                    .filter_map(|(e1, e2)| {
                        if e1.data == e2.data
                            && e1.edge_num == e2.edge_num
                            && e1.constraints == e2.constraints
                        {
                            Some((e1.clone(), e2.nodeidx))
                        } else {
                            None
                        }
                    })
                    .collect();

                let new_edges: Vec<_> = candidate_edges
                    .into_iter()
                    .filter_map(|(e1, idx2)| {
                        let child_intersection = self.intersection(e1.nodeidx, idx2);
                        if child_intersection == self.empty_node
                            && e1.nodeidx != self.empty_node
                            && idx2 != self.empty_node
                        {
                            None
                        } else {
                            Some(Edge {
                                data: e1.data.clone(),
                                constraints: e1.constraints.clone(),
                                edge_num: e1.edge_num,
                                nodeidx: child_intersection,
                            })
                        }
                    })
                    .collect();

                if new_edges.is_empty() {
                    self.empty_node
                } else {
                    self.add_node_inner(Node::default(), new_edges)
                }
            }
        }
    }

    pub fn snip(&mut self, mut e_vec: Vec<EdgeReference<Edge<S, EQ>>>) {
        todo!()
    }

    pub fn static_reduction(
        &mut self,
        root_node: NodeIndex,
        constraints_list: Vec<Constraints<EQ>>,
    ) {
        let constraint_edges = self
            .g
            .edge_references()
            .filter(|e| e.weight().constraints != None)
            .group_by(|e| (e.source(), &e.weight().data));

        for (_, edges) in &constraint_edges {
            let constraint_edges: Vec<_> = edges
                .sorted_by(|e1, e2| e1.weight().edge_num.cmp(&e2.weight().edge_num))
                .collect();
        }

        /*         self.g.edges(root_node); */

        // For each constraint, get all the nodes on each side
        // Just follow the paths for each constraint
        // apply constraint
        // Either intersection or something fancy
        // propagate effects back by updating the graph
        // Removing edges?
        // Invalid edges along the path
        // Possibly just one edge, possible the whole path
        todo!()
    }
}

#[derive(Clone)]
pub struct PathEClass {
    data: fst::raw::Fst<Vec<u8>>,
}

type Id = u64;

pub struct PathIdx<'a> {
    node: fst::raw::Node<'a>,
}

impl PathIdx<'_> {
    pub fn is_final_state(&self) -> bool {
        self.node.is_final()
    }

    pub fn get_final_value(&self) -> Option<u64> {
        if self.is_final_state() {
            Some(self.node.final_output().value())
        } else {
            None
        }
    }
}

impl PathEClass {
    pub fn new(path_data: Vec<(Vec<u8>, Id)>) -> Self {
        let data = fst::Map::from_iter(path_data.into_iter())
            .unwrap()
            .into_fst();
        PathEClass { data }
    }

    pub fn get_root(&self) -> PathIdx {
        PathIdx {
            node: self.data.root(),
        }
    }

    pub fn get_next_node_on_symbol(&self, n: PathIdx<'_>, edge: u8) -> Option<PathIdx<'_>> {
        n.node
            .transitions()
            .find(|t| t.inp == edge)
            .map(|t| PathIdx {
                node: self.data.node(t.addr),
            })
    }
}

#[derive(Clone)]
pub struct Constraints<T> {
    data: PathEClass,
    relationships: T,
}

impl<T: PathConstraintStringify> Constraints<T> {
    pub fn new(path_data: Vec<(Vec<u8>, Id)>, relationships: T) -> Self {
        Constraints {
            data: PathEClass::new(path_data),
            relationships,
        }
    }
}

pub trait PathConstraintStringify {
    fn stringify(&self, map: HashMap<Id, Vec<String>>) -> String;
}

impl PathConstraintStringify for () {
    fn stringify(&self, map: HashMap<Id, Vec<String>>) -> String {
        map.into_iter()
            .map(|(_, path_names)| path_names.into_iter().join(" = "))
            .join("\n")
    }
}

impl<T: PathConstraintStringify> Constraints<T> {
    pub fn to_string(&self) -> String {
        let mut map: HashMap<u64, Vec<String>> = HashMap::new();
        self.data
            .data
            .stream()
            .into_byte_vec()
            .into_iter()
            .for_each(|(path, id)| map.entry(id).or_default().push(path.into_iter().join(".")));
        self.relationships.stringify(map)
    }
}

impl<T: PathConstraintStringify> Display for Constraints<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl<T: PathConstraintStringify> Debug for Constraints<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl<T: PathConstraintStringify> Hash for Constraints<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl<T: PathConstraintStringify> PartialEq for Constraints<T> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl<T: PathConstraintStringify> Eq for Constraints<T> {}
