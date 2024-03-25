use itertools::{zip_eq, Itertools};

use super::component::FibonacciComponent;
use crate::core::air::{Air, Component};
use crate::core::backend::CPUBackend;
use crate::core::fields::m31::BaseField;

pub struct FibonacciAir {
    pub component: FibonacciComponent,
}

impl FibonacciAir {
    pub fn new(component: FibonacciComponent) -> Self {
        Self { component }
    }
}

impl Air<CPUBackend> for FibonacciAir {
    fn components(&self) -> Vec<&dyn Component<CPUBackend>> {
        vec![&self.component]
    }
}

pub struct MultiFibonacciAir {
    pub components: Vec<FibonacciComponent>,
}

impl MultiFibonacciAir {
    pub fn new(log_sizes: &[u32], claim: &[BaseField]) -> Self {
        let mut components = Vec::new();
        for (log_size, claim) in zip_eq(log_sizes.iter(), claim.iter()) {
            components.push(FibonacciComponent::new(*log_size, *claim));
        }
        Self { components }
    }
}

impl Air<CPUBackend> for MultiFibonacciAir {
    fn components(&self) -> Vec<&dyn Component<CPUBackend>> {
        self.components
            .iter()
            .map(|c| c as &dyn Component<CPUBackend>)
            .collect_vec()
    }
}
