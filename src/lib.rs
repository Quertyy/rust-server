use std::thread;

pub struct GroupeTaches {
    taches: Vec<thread::JoinHandle<()>>,
}

impl GroupeTaches {
    pub fn new(size: usize) -> GroupeTaches {
        assert!(size > 0);

        let mut taches = Vec::with_capacity(size);

        for _ in 0..size {
            
        }
        GroupeTaches { taches }
    }

    pub fn executer<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {

    }
}