use crate::vm::{Vm, VmMethod, vm_is_subclass};

impl Vm {
    /// Harness-oriented helper that returns all subclasses of `Test`
    /// (excluding `Test` itself) with methods named `test_*`.
    ///
    /// This API exists for the CLI test harness and is not part of
    /// the interpreter's core execution loop responsibilities.
    pub fn collect_test_classes(&self) -> Vec<(String, Vec<(String, VmMethod)>)> {
        let mut result = Vec::new();
        for class_name in self.sorted_class_names() {
            if class_name == "Test" {
                continue;
            }
            if !vm_is_subclass(&self.classes, class_name.as_str(), "Test") {
                continue;
            }
            let tests = self.test_methods_for_class(class_name);
            if !tests.is_empty() {
                result.push((class_name.clone(), tests));
            }
        }
        result
    }

    /// Harness-oriented helper that runs one test method against a fresh
    /// instance of `class_name`, including optional `setup` and `teardown`.
    ///
    /// This API exists for the CLI test harness and is not part of
    /// the interpreter's core execution loop responsibilities.
    pub fn run_single_test(
        &mut self,
        class_name: &str,
        test_method: &VmMethod,
    ) -> Result<(), String> {
        let (instance, methods) = self.build_test_instance(class_name)?;

        if let Some(setup) = methods.get("setup") {
            self.call_method_on_instance(instance.clone(), setup)?;
        }

        self.call_method_on_instance(instance.clone(), test_method)?;

        if let Some(teardown) = methods.get("teardown") {
            self.call_method_on_instance(instance, teardown)?;
        }

        Ok(())
    }
}
