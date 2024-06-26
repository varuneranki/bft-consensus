use rand::Rng;

#[derive(Debug, Clone)]
enum Message {
    Attack,
    Retreat,
}

struct Server {
    id: u32,
    peers: Vec<u32>,
    received_messages: Vec<Message>,
    is_byzantine: bool, // Flag to indicate Byzantine behavior
}

impl Server {
    fn new(id: u32, peers: Vec<u32>, is_byzantine: bool) -> Self {
        Server { id, peers, received_messages: Vec::new(), is_byzantine }
    }

    fn send_message(&self, message: Message) {
        for peer in self.peers.iter() {
            // Replace this with your communication library
            println!("Sending {:?} to server {}", message, peer);
        }
    }

    fn handle_message(&mut self, message: Message) {
        self.received_messages.push(message);
    }

    fn perform_consensus(&mut self) -> Option<Message> {
        let mut counts = [0; 2];
        for message in self.received_messages.iter() {
            match message {
                Message::Attack => counts[0] += 1,
                Message::Retreat => counts[1] += 1,
            }
        }

        if counts[0] > counts[1] {
            Some(Message::Attack)
        } else if counts[1] > counts[0] {
            Some(Message::Retreat)
        } else {
            None // No consensus yet
        }
    }

    fn run(&mut self) {
        let mut rng = rand::thread_rng();
        let initial_message = if rng.gen_bool(0.5) {
            Message::Attack
        } else {
            Message::Retreat
        };

        self.send_message(initial_message);

        loop {
            let messages_from_peers = // Receive messages from peers
                // Replace this with your communication library
                vec![Message::Attack]; // Example message

            for message in messages_from_peers.iter() {
                self.handle_message(message.clone());
            }

            let consensus = self.perform_consensus();

            if let Some(decision) = consensus {
                println!("Server {} reached consensus: {:?}", self.id, decision);
                break;
            }

            let next_message = if rng.gen_bool(0.5) {
                Message::Attack
            } else {
                Message::Retreat
            };

            // Simulate Byzantine behavior for marked servers
            if self.is_byzantine {
                let random_message = if rng.gen_bool(0.5) {
                    Message::Attack
                } else {
                    Message::Retreat
                };
                self.send_message(random_message); // Send a random message
            } else {
                self.send_message(next_message);
            }
        }
    }
}

fn main() {
    let num_servers = 20; // Adjust as needed
    let num_byzantine = 5; // Number of Byzantine servers (t)

    // Create a vector to store servers
    let mut servers: Vec<Server> = Vec::with_capacity(num_servers);

    // Create all servers
    for i in 0..num_servers {
        let mut rng = rand::thread_rng();
        let mut peers: Vec<u32> = Vec::new();

        // Randomly select a subset of servers as peers (excluding itself)
        for _ in 0..2 { // Select 2 random peers (adjust as needed)
            let random_peer = rng.gen_range(0..num_servers);
            if random_peer != i {
                peers.push(random_peer as u32);
            }
        }

        let is_byzantine = i < num_byzantine; // Mark first 'num_byzantine' servers as Byzantine
        servers.push(Server::new(i as u32, peers, is_byzantine));
    }

    // Run all servers concurrently (replace with actual implementation)
    for server in servers.iter_mut() {
        server.run();
    }
}



//functional code
/*use rand::Rng;

#[derive(Debug, Clone)]
enum Message {
    Attack,
    Retreat,
}

struct Server {
    id: u32,
    peers: Vec<u32>,
    received_messages: Vec<Message>,
}

impl Server {
    fn new(id: u32, peers: Vec<u32>) -> Self {
        Server { id, peers, received_messages: Vec::new() }
    }

    fn send_message(&self, message: Message) {
        for peer in self.peers.iter() {
            // Replace this with your communication library
            println!("Sending {:?} to server {}", message, peer);
        }
    }

    fn handle_message(&mut self, message: Message) {
        self.received_messages.push(message);
    }

    fn perform_consensus(&mut self) -> Option<Message> {
        let mut counts = [0; 2];
        for message in self.received_messages.iter() {
            match message {
                Message::Attack => counts[0] += 1,
                Message::Retreat => counts[1] += 1,
            }
        }

        if counts[0] > counts[1] {
            Some(Message::Attack)
        } else if counts[1] > counts[0] {
            Some(Message::Retreat)
        } else {
            None // No consensus yet
        }
    }

    fn run(&mut self) {
        let mut rng = rand::thread_rng();
        let initial_message = if rng.gen_bool(0.5) {
            Message::Attack
        } else {
            Message::Retreat
        };

        self.send_message(initial_message);

        loop {
            let messages_from_peers = // Receive messages from peers
                // Replace this with your communication library
                vec![Message::Attack]; // Example message

            /*for message in messages_from_peers.iter() {
                self.handle_message(*message);
            }*/

            for message in messages_from_peers.iter() {
                self.handle_message(message.clone());
            }

            let consensus = self.perform_consensus();

            if let Some(decision) = consensus {
                println!("Server {} reached consensus: {:?}", self.id, decision);
                break;
            }

            let next_message = if rng.gen_bool(0.5) {
                Message::Attack
            } else {
                Message::Retreat
            };

            self.send_message(next_message);
        }
    }
}

fn main() {
    let num_servers = 4; // Adjust as needed

    // Create a vector to store servers
    let mut servers: Vec<Server> = Vec::with_capacity(num_servers);

    // Create all servers
    for i in 0..num_servers {
        let mut rng = rand::thread_rng();
        let mut peers: Vec<u32> = Vec::new();

        // Randomly select a subset of servers as peers (excluding itself)
        for _ in 0..2 { // Select 2 random peers (adjust as needed)
            let random_peer = rng.gen_range(0..num_servers);
            if random_peer != i {
                peers.push(random_peer as u32);
            }
        }

        servers.push(Server::new(i as u32, peers));
    }

    // Run all servers concurrently (replace with actual implementation)
    for server in servers.iter_mut() {
        server.run();
    }
}
*/


//Three servers code works flawlessly. Now we test for n servers model.
/*fn main() {

    let server1 = Server::new(1, vec![2, 3]).run();
    let server2 = Server::new(2, vec![1, 3]).run();
    let server3 = Server::new(3, vec![1, 2]).run();
}*/


//Scrap code that is non functional
// fn main() {
//     // Define the number of servers
//     let num_servers = 9;
//
//     // Create a vector to store servers
//     let mut servers: Vec<Server> = Vec::with_capacity(num_servers);
//
//     // Loop to create servers and their connections
//     /*for server_id in 1..=num_servers {
//         //Attempt1
//         // Generate connected server IDs (excluding the current server)
//         /*let connected_servers: Vec<u32> = (1..=num_servers)
//             .filter(|id| *id != server_id)
//             .take(2) // Connect to a maximum of 2 other servers
//             .collect();
//             //.map(|id| *id as u32) // Convert each id to u32 within the iterator
//
//         // Create a new server with its connected servers
//         let server = Server::new(server_id as u32, connected_servers);
//         servers.push(server.run());  // Run the server and add it to the vector */
//
//         //Attempt2
//         /*let mut connected_servers: Vec<u32> = Vec::with_capacity(2);
//         for id in (1..=num_servers).filter(|id| id != server_id).take(2) {
//             connected_servers.push(id as u32); // Convert and push into the vector
//         }
//
//         let server = Server::new(server_id as u32, connected_servers);
//         servers.push(server.run());*/
//
//
//     }*/
//
//     for server_id in 1..=num_servers {
//         let connected_servers: Vec<u32> = (1..=num_servers)
//             .filter(|id| id.eq(&server_id)) // Check for equality with reference
//             .take(2)
//             .collect();
//
//         let server = Server::new(server_id as u32, connected_servers);
//         servers.push(server.run());
//     }
//
//
//     // Now you have a vector 'servers' containing all 9 servers
//     // with their connections defined
// }



