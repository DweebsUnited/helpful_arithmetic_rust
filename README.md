# helpful_arithmetic_rust
The Rust implementation of an overengineered logic puzzle solver. 

This project aims to solve the Kakuro-style numeric logic puzzle, but as a learning exercise extends this to silly goals.

The endgoal is three services, working together as Docker containers, controlled by Ansible.

## Logger
The logger will act as a RAFT-based distributed consensus log, holding the game board, any proved fields, and serving this information to any active solvers or dashboards.

## Solver
The solver is the worker service that will try to prove logically any new states, and report these back to the logger. They will also respond to a dashboard's status request broadcast.

## Dashboard
The dashboard here will just be a CLI interface to the other services. I plan to implement a proper web dashboard in Go eventually, but that is a different project.

This dashboard will be able to query the status of the other services, submit new game boards to the logger, and dump extended log states of other services.

See Infrastructure below for comments on controlling the Docker containers, and monkeying with the network to test reliability.

## Infrastructure
All services will be started by Ansible as Docker containers.

I plan to eventually explore Swarm mode, and dynamic scaling of service counts, which will probably be implemented as part of Dashboard, but I don't know enough about this yet to comment further.

I also plan to implement some monkeying features, such as segmenting the network, killing/resetting services unexpectedly, etc. These will be used to test the reliability of my implementations.