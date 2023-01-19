pub enum SchedulerType {
    StageredScheduler,
}

// ===

pub trait IterationScheduler {
    fn nb_iteration(&self) -> u32;
    fn get_ordered_list(&self, nb_tests: u32) -> Vec<u32>;
}

// ===

pub struct StageredScheduler {
    nb_iteration: u32,
}

impl StageredScheduler {
    pub fn new(nb_iteration: u32) -> Self {
        Self {
            nb_iteration: nb_iteration,
        }
    }
}

impl IterationScheduler for StageredScheduler {
    fn nb_iteration(&self) -> u32 {
        self.nb_iteration
    }
    fn get_ordered_list(&self, nb_tests: u32) -> Vec<u32> {
        let mut vec = Vec::new();
        for _i in 0..self.nb_iteration {
            for t in 0..nb_tests {
                vec.push(t);
            }
        }
        vec
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iteration_setter() {
        let scheduler = StageredScheduler::new(3);
        assert_eq!(3, scheduler.nb_iteration());
    }

    #[test]
    fn test_ordered_list() {
        let scheduler = StageredScheduler::new(2);
        let list = scheduler.get_ordered_list(3);

        assert_eq!(6, list.len());

        assert_eq!(0, list[0]);
        assert_eq!(1, list[1]);
        assert_eq!(2, list[2]);
        assert_eq!(0, list[3]);
        assert_eq!(1, list[4]);
        assert_eq!(2, list[5]);
    }
}
