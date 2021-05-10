use std::{collections::HashMap, hash::Hash};

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl CellID {
    fn as_input(&self) -> Option<InputCellID> {
        match self {
            CellID::Input(input) => Some(*input),
            CellID::Compute(_) => None,
        }
    }

    fn as_compute(&self) -> Option<ComputeCellID> {
        match self {
            CellID::Input(_) => None,
            CellID::Compute(compute) => Some(*compute),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T: Copy + PartialEq> {
    value: T,
}

struct ComputeCell<T: Copy + PartialEq> {
    value: T,
}

pub struct Input<T: Copy + PartialEq> {
    cells: Vec<InputCell<T>>,
}

impl<T: Copy + PartialEq> Input<T> {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn add_cell(&mut self, value: T) -> InputCellID {
        self.cells.push(InputCell { value });
        InputCellID(self.cells.len() - 1)
    }

    pub fn value_of(&self, cell_id: InputCellID) -> Option<T> {
        self.cells.get(cell_id.0).map(|cell| cell.value)
    }

    pub fn set_value_of(&mut self, cell_id: InputCellID, value: T) -> bool {
        if self.cells.len() <= cell_id.0 {
            return false;
        }
        self.cells[cell_id.0].value = value;
        true
    }

    // pub fn collect_values_of(&self, cell_ids: impl Iterator<Item = InputCellID>, to: &mut Vec<T>) {
    //     for cell_id in cell_ids {
    //         if cell_id.0 < self.cells.len() {
    //             to.push(self.cells[cell_id.0].value)
    //         }
    //     }
    // }

    pub fn exists(&self, cell_id: InputCellID) -> bool {
        cell_id.0 < self.cells.len()
    }

    pub fn not_exists(&self, cell_id: InputCellID) -> bool {
        !self.exists(cell_id)
    }
}

impl<T: Copy + PartialEq> Default for Input<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::type_complexity)]
pub struct Compute<'a, T: Copy + Default + PartialEq> {
    cells: Vec<ComputeCell<T>>,
    functions: HashMap<ComputeCellID, Box<dyn Fn(&[T]) -> T + 'a>>,
    callbacks: HashMap<ComputeCellID, Vec<Option<Box<dyn FnMut(T) + 'a>>>>,
}

impl<'a, T: Copy + Default + PartialEq> Compute<'a, T> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            functions: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    pub fn add_cell<ComputeFunc>(
        &mut self,
        compute_func: ComputeFunc,
        dependency_values: &[T],
    ) -> ComputeCellID
    where
        ComputeFunc: Fn(&[T]) -> T + 'a,
    {
        self.cells.push(ComputeCell {
            value: compute_func(dependency_values),
        });
        let cell_id = ComputeCellID(self.cells.len() - 1);
        self.functions.insert(cell_id, Box::new(compute_func));
        cell_id
    }

    pub fn add_callback<CallbackFunc: Fn(T) + 'a>(
        &mut self,
        cell_id: ComputeCellID,
        callback: CallbackFunc,
    ) -> Option<CallbackID> {
        if self.cells.len() <= cell_id.0 {
            return None;
        }
        let callbacks = self.callbacks.entry(cell_id).or_insert_with(Vec::new);
        match callbacks.iter().position(Option::is_none) {
            Some(position) => {
                callbacks[position] = Some(Box::new(callback));
                Some(CallbackID(position))
            }
            None => {
                callbacks.push(Some(Box::new(callback)));
                Some(CallbackID(callbacks.len() - 1))
            }
        }
    }

    pub fn remove_callback(
        &mut self,
        cell_id: ComputeCellID,
        callback_id: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        if self.cells.len() <= cell_id.0 {
            return Err(RemoveCallbackError::NonexistentCell);
        }
        let callbacks = self.callbacks.entry(cell_id).or_insert_with(Vec::new);
        if callbacks.len() <= callback_id.0 || callbacks[callback_id.0].is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        callbacks[callback_id.0] = None;
        Ok(())
    }

    pub fn value_of(&self, cell_id: ComputeCellID) -> Option<T> {
        self.cells.get(cell_id.0).map(|cell| cell.value)
    }

    // pub fn collect_values_of(
    //     &self,
    //     cell_ids: impl Iterator<Item = ComputeCellID>,
    //     to: &mut Vec<T>,
    // ) {
    //     for cell_id in cell_ids {
    //         if cell_id.0 < self.cells.len() {
    //             to.push(self.cells[cell_id.0].value)
    //         }
    //     }
    // }

    pub fn exists(&self, cell_id: ComputeCellID) -> bool {
        cell_id.0 < self.cells.len()
    }

    pub fn not_exists(&self, cell_id: ComputeCellID) -> bool {
        !self.exists(cell_id)
    }

    pub fn recompute(&mut self, cell_id: ComputeCellID, dependency_values: &[T]) -> Option<(T, T)> {
        match (self.cells.get_mut(cell_id.0), self.functions.get(&cell_id)) {
            (Some(cell), Some(function)) => {
                let old_value = cell.value;
                cell.value = function(dependency_values);
                match old_value != cell.value {
                    true => Some((old_value, cell.value)),
                    false => None,
                }
            }
            _ => None,
        }
    }

    pub fn invoke_callbacks_on(&mut self, cell_ids: impl Iterator<Item = ComputeCellID>) {
        for cell_id in cell_ids {
            if let Some(cell) = self.cells.get(cell_id.0) {
                let cell_value = cell.value;
                if let Some(callbacks) = self.callbacks.get_mut(&cell_id) {
                    for callback in callbacks.iter_mut().flatten() {
                        callback(cell_value);
                    }
                }
            }
        }
    }
}

impl<'a, T: Copy + Default + PartialEq> Default for Compute<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Dependency {
    dependents: HashMap<CellID, Vec<ComputeCellID>>,
    dependencies: HashMap<ComputeCellID, Vec<CellID>>,
}

impl Dependency {
    pub fn new() -> Self {
        Self {
            dependents: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn add_dependencies(&mut self, cell_id: ComputeCellID, dependencies: &[CellID]) {
        self.dependencies
            .entry(cell_id)
            .and_modify(|entry| *entry = dependencies.to_vec())
            .or_insert_with(|| dependencies.to_vec());
        for dependency in dependencies {
            self.dependents
                .entry(*dependency)
                .and_modify(|entry| entry.push(cell_id))
                .or_insert_with(|| vec![cell_id]);
        }
    }

    pub fn dependents_of(&self, cell_id: CellID) -> Option<&[ComputeCellID]> {
        self.dependents.get(&cell_id).map(|v| v.as_slice())
    }

    pub fn dependencies_of(&self, cell_id: ComputeCellID) -> Option<&[CellID]> {
        self.dependencies.get(&cell_id).map(|v| v.as_slice())
    }
}

impl Default for Dependency {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Reactor<'a, T: Copy + Default + PartialEq> {
    input: Input<T>,
    dependency: Dependency,
    compute: Compute<'a, T>,
}

impl<'a, T: Copy + Default + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            dependency: Dependency::new(),
            compute: Compute::new(),
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.input.add_cell(initial)
    }

    pub fn create_compute<'b, ComputeFunc>(
        &'b mut self,
        dependencies: &[CellID],
        compute_func: ComputeFunc,
    ) -> Result<ComputeCellID, CellID>
    where
        ComputeFunc: 'a + Fn(&[T]) -> T,
    {
        if let Some(value) = self.some_input_cell_dependency_does_not_exist(dependencies) {
            return value;
        }
        if let Some(value) = self.some_compute_cell_dependency_does_not_exist(dependencies) {
            return value;
        }
        let dependency_values = self.values_of(dependencies);
        let cell_id = self.compute.add_cell(compute_func, &dependency_values);
        self.dependency.add_dependencies(cell_id, dependencies);
        Ok(cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(cell_id) => self.input.value_of(cell_id),
            CellID::Compute(cell_id) => self.compute.value_of(cell_id),
        }
    }

    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if let true = self.input.set_value_of(id, new_value) {
            let cells_recomputed = self.recompute_descendents_of_input_cell(id);
            self.invoke_callbacks_on(
                cells_recomputed
                    .into_iter()
                    .filter(|(_, values)| values.0 != values.1)
                    .map(|(key, _)| key),
            );
            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<'b, CallbackFunc>(
        &'b mut self,
        cell_id: ComputeCellID,
        callback: CallbackFunc,
    ) -> Option<CallbackID>
    where
        CallbackFunc: Fn(T) + 'a,
    {
        self.compute.add_callback(cell_id, callback)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell_id: ComputeCellID,
        callback_id: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.compute.remove_callback(cell_id, callback_id)
    }

    fn some_compute_cell_dependency_does_not_exist(
        &self,
        dependencies: &[CellID],
    ) -> Option<Result<ComputeCellID, CellID>> {
        if let Some(non_existent_compute_cell) = dependencies
            .iter()
            .flat_map(CellID::as_compute)
            .find(|cell_id| self.compute.not_exists(*cell_id))
        {
            return Some(Err(CellID::Compute(non_existent_compute_cell)));
        }
        None
    }

    fn some_input_cell_dependency_does_not_exist(
        &self,
        dependencies: &[CellID],
    ) -> Option<Result<ComputeCellID, CellID>> {
        if let Some(non_existent_input_cell) = dependencies
            .iter()
            .flat_map(CellID::as_input)
            .find(|cell_id| self.input.not_exists(*cell_id))
        {
            return Some(Err(CellID::Input(non_existent_input_cell)));
        }
        None
    }

    fn values_of(&self, cell_ids: &[CellID]) -> Vec<T> {
        let cell_values = cell_ids
            .iter()
            .flat_map(|dep_id| self.value(*dep_id))
            .collect::<Vec<_>>();
        cell_values
    }

    fn recompute_descendents_of_input_cell(
        &mut self,
        cell_id: InputCellID,
    ) -> HashMap<ComputeCellID, (T, T)> {
        let mut cells_recomputed = HashMap::new();
        let (input, compute, dependency) = (&self.input, &mut self.compute, &self.dependency);
        Self::recompute_descendents_of(
            CellID::Input(cell_id),
            input,
            compute,
            dependency,
            &mut cells_recomputed,
        );
        cells_recomputed
    }

    fn recompute_descendents_of(
        cell_id: CellID,
        input: &Input<T>,
        compute: &mut Compute<T>,
        dependency: &Dependency,
        cells_recomputed: &mut HashMap<ComputeCellID, (T, T)>,
    ) {
        if let Some(dependents) = dependency.dependents_of(cell_id) {
            for dependent_cell_id in dependents {
                if let Some(dependencies) = dependency.dependencies_of(*dependent_cell_id) {
                    let dependency_values = dependencies
                        .iter()
                        .map(|cell| match cell {
                            CellID::Input(cell_id) => input.value_of(*cell_id),
                            CellID::Compute(cell_id) => compute.value_of(*cell_id),
                        })
                        .flatten()
                        .collect::<Vec<_>>();
                    if let Some((old_value, new_value)) =
                        compute.recompute(*dependent_cell_id, &dependency_values)
                    {
                        cells_recomputed
                            .entry(*dependent_cell_id)
                            .and_modify(|v| v.1 = new_value)
                            .or_insert_with(|| (old_value, new_value));
                        Self::recompute_descendents_of(
                            CellID::Compute(*dependent_cell_id),
                            input,
                            compute,
                            dependency,
                            cells_recomputed,
                        );
                    }
                }
            }
        }
    }

    fn invoke_callbacks_on(&mut self, cell_ids: impl Iterator<Item = ComputeCellID>) {
        self.compute.invoke_callbacks_on(cell_ids);
    }
}

impl<'a, T: Copy + Default + PartialEq> Default for Reactor<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}
