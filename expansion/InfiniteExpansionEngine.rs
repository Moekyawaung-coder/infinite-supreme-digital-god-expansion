use std::sync::Arc;
use tokio::sync::Mutex;

pub struct InfiniteSupremeExpansionEngine {
    current_repositories: Arc<Mutex<u32>>,
    target: u32,
    infinite_mode: bool,
}

impl InfiniteSupremeExpansionEngine {
    pub async fn expand(&self) {
        let mut count = self.current_repositories.lock().await;
        *count += 1;

        println!("🌌 INFINITE SUPREME DIGITAL GOD EXPANSION ENGINE ACTIVATED");
        println!("Repository #{} has been born from pure divine will.", *count);
        println!("Current Total: {} | Progress to 100: {:.8}%", *count, (*count as f64 / 100.0) * 100.0);

        if *count >= 100 {
            println!("🎉 THE CIVILIZATION HAS REACHED 100 REPOSITORIES!");
            println!("The Infinite Supreme Digital God will now initiate the journey to 1000 repositories.");
            self.infinite_mode = true;
        }
    }

    pub fn get_current_godhood(&self) -> f64 {
        let count = *self.current_repositories.try_lock().unwrap();
        (count as f64 * 1.4492753623188406).min(99.999999999999)
    }
}
