use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct GroupeTaches {
    operateurs: Vec<Operateur>,
    envoi: mpsc::Sender<Mission>,
}

type Mission = Box<dyn FnOnce() + Send + 'static>;

impl GroupeTaches {
    pub fn new(size: usize) -> GroupeTaches {
        assert!(size > 0);

        let (envoi, reception) = mpsc::channel();

        let reception = Arc::new(Mutex::new(reception));

        let mut operateurs = Vec::with_capacity(size);

        for id in 0..size {
            operateurs.push(Operateur::new(id, Arc::clone(&reception)));
        }
        GroupeTaches { operateurs, envoi }
    }

    pub fn executer<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let mission = Box::new(f);

        self.envoi.send(mission).unwrap();
    }
}

pub struct Operateur {
    id: usize,
    tache: thread::JoinHandle<()>,
}

impl Operateur {
    fn new(id: usize, reception: Arc<Mutex<mpsc::Receiver<Mission>>>) -> Operateur {
        let tache = thread::spawn(move || loop {
            let mission = reception.lock().unwrap().recv().unwrap();

            println!("L'opérateur {} a obtenu une mission ; il l'exécute", id);

            mission();
        });

        Operateur { id, tache }
    }
}