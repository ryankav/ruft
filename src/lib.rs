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

mod types {
    pub struct ServerId(pub usize);

    pub struct Term(pub usize);

    /// Another placeholder for the command
    pub enum Command {}

    /// Each entry contains command for state machine, and term when entry was received by leader (first index is 1)
    pub struct LogEntry(pub Command, pub usize);
}

/// This module contains a first pass, sketch of the structs defined in the top left box which
/// lists the states different servers have and how they're persisted.
mod state {
    use super::types::{LogEntry, ServerId};

    /// This state is on all servers and needs to be persisted so will eventually need to be
    /// changed as this, clearly isn't persisted, due to it just being a sketch currently.
    struct PersistentState {
        /// latest term server has seen (initialized to 0 on first boot, increases monotonically)
        current_term: usize,
        /// candidateId that received vote in current term (or null if none)
        voted_for: Option<ServerId>,
        /// The list of log entries
        log: Vec<LogEntry>,
    }

    /// This state is volatile and held on all servers
    struct VolatileState {
        /// index of highest log entry known to be committed (initialized to 0, increases monotonically)
        commit_index: usize,
        /// index of highest log entry applied to state machine (initialized to 0, increases monotonically)
        last_applied: usize,
    }

    /// Placeholder constant value for now
    const NUMBER_OF_SERVERS: usize = 1;
    /// This is volatile state that is only on the leader and is Reinitialized after election
    struct VolatileLeaderState {
        /// for each server, index of the next log entry to send to that server (initialized to leader last log index + 1)
        next_index: [usize; NUMBER_OF_SERVERS],
        /// for each server, index of highest log entry known to be replicated on server (initialized to 0, increases monotonically)
        match_index: [usize; NUMBER_OF_SERVERS],
    }
}

/// The server module contains the interface for the server RPC definitions given in the remaining
/// boxes in figure 2 of the raft paper
mod server {
    use super::types::{LogEntry, ServerId, Term};
    /// Marker struct signifying that the server is currenlty in the Leader state signifying that
    /// at some point in the past it received a majority of the votes from the other servers
    ///
    /// Servers in the Leader state must:
    ///  * Upon election: send initial empty AppendEntries RPCs (heartbeat) to each server; repeat during idle periods to prevent election timeouts
    ///  * If command received from client: append entry to local log, respond after entry applied to state machine
    ///  * If last log index ≥ nextIndex for a follower: send AppendEntries RPC with log entries starting at nextIndex
    ///  * If successful: update nextIndex and matchIndex for follower
    ///  * If AppendEntries fails because of log inconsistency: decrement nextIndex and retry
    ///  * If there exists an N such that N > commitIndex, a majority of matchIndex[i] ≥ N, and log[N].term == currentTerm: set commitIndex = N
    struct Leader;

    /// Marker struct signifying that the server is currenlty in the Candidate state signifying that
    /// it has requested votes from other servers and is trying to setup a leader
    ///
    /// Servers in the Candidate state must:
    ///  * On conversion to candidate, start election:
    ///      * Increment currentTerm
    ///      * Vote for self
    ///      * Reset election timer
    ///      * Send RequestVote RPCs to all other servers
    ///  * If votes received from majority of servers: become leader
    ///  * If AppendEntries RPC received from new leader: convert to follower
    ///  * If election timeout elapses: start new election
    struct Candidate;

    /// Marker struct signifying that the server is currenlty in the Follower state which is the
    /// state the server initialises itself to on startup
    ///
    /// Servers in the follower state must:
    ///  * Respond to RPCs from candidates and leaders
    ///  * If election timeout elapses without receiving AppendEntries RPC from current leader or granting vote to candidate: convert to candidate
    struct Follower;

    /// The server can be in one of three states:
    ///  * [Follower]
    ///  * [Candidate]
    ///  * [Leader]
    ///
    ///  All servers irrelevant of the state they're in must:
    ///   * If commitIndex > lastApplied: increment lastApplied, apply log[lastApplied] to state machine
    ///   * If RPC request or response contains term T > currentTerm: set currentTerm = T, convert to follower
    struct Server<T>(T);

    // TODO: Add in the RPC calls and the state to the server as well as the conditions for
    // changing the servers state for each given object

    impl<T> Server<T> {
        /// Invoked by leader to replicate log entries; also used as heartbeat
        ///
        /// Implementation details:
        /// 1. Reply false if term < currentTerm
        /// 2. Reply false if log doesn’t contain an entry at prevLogIndex whose term matches prevLogTerm
        /// 3. If an existing entry conflicts with a new one (same index but different terms), delete the existing entry and all that follow it
        /// 4. Append any new entries not already in the log
        /// 5. If leaderCommit > commitIndex, set commitIndex = min(leaderCommit, index of last new entry)
        fn append_entires(
            leaders_term: Term,
            leader_id: ServerId,
            prev_log_index: usize,
            entries: Vec<LogEntry>,
            leader_commit: usize,
        ) -> Result<Term, ()> {
            // TODO: work out if retrun type is always a term even when unsuccesful
            todo!()
        }

        /// Invoked by candidates to gather votes
        ///
        /// Implementation details:
        /// 1. Reply false if term < currentTerm
        /// 2. If votedFor is null or candidateId, and candidate’s log is at least as up-to-date as receiver’s log, grant vote
        fn rquest_vote(
            candidates_term: Term,
            candidate_id: ServerId,
            last_log_index: usize,
            last_log_term: usize,
        ) -> Result<Term, ()> {
            // TODO: work out if retrun type is always a term even when unsuccesful
            todo!()
        }
    }
}
