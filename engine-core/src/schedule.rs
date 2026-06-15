use crate::Engine;
use std::collections::HashMap;

pub struct Schedule {
    stages: Vec<String>,
    systems: HashMap<String, Vec<Box<dyn FnMut(&mut Engine) + Send + Sync + 'static>>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            systems: HashMap::new(),
        }
    }

    pub fn add_stage(&mut self, name: impl Into<String>) -> &mut Self {
        let name = name.into();
        if !self.stages.contains(&name) {
            self.stages.push(name.clone());
            self.systems.insert(name, Vec::new());
        }
        self
    }

    pub fn add_system_to_stage<F>(&mut self, stage_name: impl Into<String>, system: F) -> &mut Self
    where
        F: FnMut(&mut Engine) + Send + Sync + 'static,
    {
        let stage_name = stage_name.into();
        if !self.stages.contains(&stage_name) {
            self.add_stage(&stage_name);
        }
        if let Some(systems) = self.systems.get_mut(&stage_name) {
            systems.push(Box::new(system));
        }
        self
    }

    pub fn run(&mut self, engine: &mut Engine) {
        for stage_name in &self.stages {
            self.run_stage(stage_name, engine);
        }
    }

    pub fn run_stage(&mut self, stage_name: &str, engine: &mut Engine) {
        if let Some(systems) = self.systems.get_mut(stage_name) {
            for system in systems.iter_mut() {
                system(engine);
            }
        }
    }

    pub fn stage_order(&self) -> &[String] {
        &self.stages
    }

    pub fn stage_names(&self) -> Vec<&str> {
        self.stages.iter().map(|s| s.as_str()).collect()
    }
}

impl Default for Schedule {
    fn default() -> Self {
        let mut schedule = Self::new();
        schedule.add_stage("STARTUP");
        schedule.add_stage("UPDATE");
        schedule.add_stage("RENDER");
        schedule.add_stage("SHUTDOWN");
        schedule
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn schedule_new() {
        let schedule = Schedule::new();
        assert!(schedule.stages.is_empty());
    }

    #[test]
    fn schedule_add_stage() {
        let mut schedule = Schedule::new();
        schedule.add_stage("UPDATE");
        assert_eq!(schedule.stages, vec!["UPDATE"]);
    }

    #[test]
    fn schedule_add_system_to_stage() {
        let mut schedule = Schedule::new();
        schedule.add_system_to_stage("UPDATE", |_engine| {});
        assert!(schedule.systems.contains_key("UPDATE"));
        assert_eq!(schedule.systems["UPDATE"].len(), 1);
    }

    #[test]
    fn schedule_run_order() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let mut schedule = Schedule::new();
        schedule.add_stage("FIRST");
        schedule.add_stage("SECOND");
        
        schedule.add_system_to_stage("FIRST", move |_engine| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        let counter_clone2 = counter.clone();
        schedule.add_system_to_stage("SECOND", move |_engine| {
            counter_clone2.fetch_add(10, Ordering::SeqCst);
        });

        let mut engine = Engine::new(EngineConfig::default());
        schedule.run(&mut engine);

        assert_eq!(counter.load(Ordering::SeqCst), 11);
    }

    #[test]
    fn schedule_stage_order() {
        let mut schedule = Schedule::new();
        schedule.add_stage("A");
        schedule.add_stage("B");
        schedule.add_stage("C");
        
        let order = schedule.stage_order();
        assert_eq!(order, &["A", "B", "C"]);
    }

    #[test]
    fn schedule_default_stages() {
        let schedule = Schedule::default();
        assert_eq!(schedule.stages, vec!["STARTUP", "UPDATE", "RENDER", "SHUTDOWN"]);
    }
}
