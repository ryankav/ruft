//! A simple implementation of the [raft](https://raft.github.io/raft.pdf) algorithm
//!
//! The ruft library is just an educational toy that I'm writing to try and port the raft consensus
//! algorithm paper, in to a simple yet functioning implementation of the consensus algorithm. As
//! the goal of this is to get a better understanding of the consensus algorithm the aim will be to
//! focus on correctness and implementing the simplest implementation possible.

/// The state we actually want to replicate across our servers
///
/// In figure 1 of the paper they show a very simple state which allows for assignment to three
/// variables: X, Y & Z. As the aim of this project is to look into the algorithm more deeply it
/// seems reasonable to start with this state machine.
struct ReplicatedState {
    x: u64,
    y: u64,
    z: u64,
}
