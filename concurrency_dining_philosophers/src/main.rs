// ANCHOR: Philosopher
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    // ANCHOR_END: Philosopher
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
}

// ANCHOR: Philosopher-think
impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }
    // ANCHOR_END: Philosopher-think

    // ANCHOR: Philosopher-eat
    fn eat(&self) {
        // ANCHOR_END: Philosopher-eat
        println!("{} is trying to eat", &self.name);
        let left = self.left_fork.lock().unwrap();
        let right = self.right_fork.lock().unwrap();

        // ANCHOR: Philosopher-eat-end
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // ANCHOR_END: Philosopher-eat-end
    let (tx, rx) = mpsc::sync_channel(10);

    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    for i in 0..forks.len() {
        let tx = tx.clone();
        let mut left_fork = forks[i].clone();
        let mut right_fork = forks[(i + 1) % forks.len()].clone();

        // To avoid a deadlock, we have to break the symmetry
        // somewhere. This will swap the forks without deinitializing
        // either of them.
        if i == forks.len() - 1 {
            std::mem::swap(&mut left_fork, &mut right_fork);
        }

        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork,
            right_fork,
        };

        thread::spawn(move || {
            for _ in 0..100 {
                philosopher.eat();
                philosopher.think();
            }
        });
    }

    drop(tx);
    for thought in rx {
        println!("{thought}");
    }
}