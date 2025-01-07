use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub energy_cost: f64,
    pub time_cost: u32,
    pub priority: u32,
    pub dependencies: Vec<usize>,
}

impl Task {
    pub fn new(id: usize, energy_cost: f64, time_cost: u32, priority: u32, dependencies: Vec<usize>) -> Self {
        Self {
            id,
            energy_cost,
            time_cost,
            priority,
            dependencies,
        }
    }
}

#[derive(Debug)]
pub struct MissionPlanner {
    tasks: Vec<Task>,
    max_energy: f64,
    max_time: u32,
    memo: HashMap<(usize, u32, u32), (f64, Vec<usize>)>,
}

impl MissionPlanner {
    pub fn new(tasks: Vec<Task>, max_energy: f64, max_time: u32) -> Self {
        Self {
            tasks,
            max_energy,
            max_time,
            memo: HashMap::new(),
        }
    }

    /// Plan optimal task sequence using dynamic programming
    pub fn optimize_mission(&mut self) -> Option<(f64, Vec<usize>)> {
        self.memo.clear();
        
        // Validate task dependencies
        if !self.validate_dependencies() {
            return None;
        }

        // Initialize energy states for discretization
        let energy_states = (0..=100).map(|x| x as f64 * self.max_energy / 100.0).collect::<Vec<_>>();
        
        let result = self.optimize_recursive(0, self.max_time, &energy_states);
        
        result.map(|(total_value, task_sequence)| {
            (total_value, self.ensure_dependencies(task_sequence))
        })
    }

    /// Recursive DP function for optimization
    fn optimize_recursive(
        &mut self,
        current_task: usize,
        remaining_time: u32,
        energy_states: &[f64],
    ) -> Option<(f64, Vec<usize>)> {
        // Base cases
        if current_task >= self.tasks.len() || remaining_time == 0 {
            return Some((0.0, Vec::new()));
        }

        // Check memoization
        let state_key = (current_task, remaining_time, energy_states.len() as u32);
        if let Some(&result) = self.memo.get(&state_key) {
            return Some(result);
        }

        let mut best_value = 0.0;
        let mut best_sequence = Vec::new();

        // Try including current task at different energy levels
        for &energy_level in energy_states {
            let task = &self.tasks[current_task];
            
            if task.time_cost <= remaining_time && energy_level >= task.energy_cost {
                // Recursive case: include current task
                if let Some((sub_value, mut sub_sequence)) = self.optimize_recursive(
                    current_task + 1,
                    remaining_time - task.time_cost,
                    energy_states,
                ) {
                    let total_value = sub_value + (task.priority as f64 * (energy_level / task.energy_cost));
                    
                    if total_value > best_value {
                        best_value = total_value;
                        best_sequence = sub_sequence;
                        best_sequence.insert(0, task.id);
                    }
                }
            }
        }

        // Try skipping current task
        if let Some((skip_value, skip_sequence)) = self.optimize_recursive(
            current_task + 1,
            remaining_time,
            energy_states,
        ) {
            if skip_value > best_value {
                best_value = skip_value;
                best_sequence = skip_sequence;
            }
        }

        let result = (best_value, best_sequence);
        self.memo.insert(state_key, result);
        Some(result)
    }

    /// Validate that task dependencies form a DAG
    fn validate_dependencies(&self) -> bool {
        let mut visited = vec![false; self.tasks.len()];
        let mut stack = vec![false; self.tasks.len()];

        for task_id in 0..self.tasks.len() {
            if !visited[task_id] {
                if self.has_cycle(task_id, &mut visited, &mut stack) {
                    return false;
                }
            }
        }
        true
    }

    fn has_cycle(&self, task_id: usize, visited: &mut [bool], stack: &mut [bool]) -> bool {
        visited[task_id] = true;
        stack[task_id] = true;

        for &dep_id in &self.tasks[task_id].dependencies {
            if !visited[dep_id] {
                if self.has_cycle(dep_id, visited, stack) {
                    return true;
                }
            } else if stack[dep_id] {
                return true;
            }
        }

        stack[task_id] = false;
        false
    }

    /// Ensure all dependencies are scheduled before dependent tasks
    fn ensure_dependencies(&self, task_sequence: Vec<usize>) -> Vec<usize> {
        let mut result = Vec::new();
        let mut scheduled = vec![false; self.tasks.len()];
        
        for &task_id in &task_sequence {
            self.schedule_with_dependencies(task_id, &mut result, &mut scheduled);
        }
        
        result
    }

    fn schedule_with_dependencies(&self, task_id: usize, result: &mut Vec<usize>, scheduled: &mut [bool]) {
        if scheduled[task_id] {
            return;
        }

        // Schedule dependencies first
        for &dep_id in &self.tasks[task_id].dependencies {
            self.schedule_with_dependencies(dep_id, result, scheduled);
        }

        result.push(task_id);
        scheduled[task_id] = true;
    }

    /// Calculate total energy consumption for a sequence of tasks
    pub fn calculate_energy_consumption(&self, task_sequence: &[usize]) -> f64 {
        task_sequence
            .iter()
            .map(|&task_id| self.tasks[task_id].energy_cost)
            .sum()
    }

    /// Calculate total time for a sequence of tasks
    pub fn calculate_total_time(&self, task_sequence: &[usize]) -> u32 {
        task_sequence
            .iter()
            .map(|&task_id| self.tasks[task_id].time_cost)
            .sum()
    }
}