use super::binding::*;

use std::collections::HashMap;
use std::mem::replace;
use std::ptr::hash;

pub struct Scope {
    levels: Vec<ScopeLevel>,
}

struct LoopInfo {
    break_called: bool,
    boundary: bool,
}

struct ScopeLevel {
    pub bindings: HashMap<String, Binding>,
    pub loop_info: Option<LoopInfo>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            levels: vec![ScopeLevel::new()],
        }
    }

    pub fn has_level(&self) -> bool {
        !self.levels.is_empty()
    }

    pub fn has_binding(&self, name: &String) -> bool {
        for level in self.levels.iter().rev() {
            if level.bindings.contains_key(name) {
                return true;
            }
        }

        return false;
    }

    pub fn get_binding(&self, name: &String) -> Option<&Binding> {
        for level in self.levels.iter().rev() {
            if level.bindings.contains_key(name) {
                return level.bindings.get(name);
            }
        }

        None
    }

    pub fn get_binding_mut(&mut self, name: &String) -> Option<&mut Binding> {
        for level in self.levels.iter_mut().rev() {
            if level.bindings.contains_key(name) {
                return level.bindings.get_mut(name);
            }
        }

        None
    }

    pub fn is_in_loop(&self) -> bool {
        for level in self.levels.iter().rev() {
            match &level.loop_info {
                Some(v) => return !v.boundary,
                None => {}
            }
        }

        false
    }

    pub fn is_loop_broken(&self) -> bool {
        for level in self.levels.iter().rev() {
            match &level.loop_info {
                Some(v) => return !v.boundary && v.break_called,
                None => {}
            }
        }

        false
    }

    pub fn break_loop(&mut self) {
        for level in self.levels.iter_mut().rev() {
            match &mut level.loop_info {
                Some(v) => {
                    if !v.boundary {
                        v.break_called = true;
                    }
                    return;
                }
                _ => {}
            }
        }
    }

    pub fn enter(&mut self) -> &mut Self {
        self.levels.push(ScopeLevel::new());
        self
    }

    pub fn enter_loop(&mut self) -> &mut Self {
        self.levels.push(ScopeLevel::new_loop());
        self
    }

    pub fn enter_loop_boundary(&mut self) -> &mut Self {
        self.levels.push(ScopeLevel::new_loop_boundary());
        self
    }

    pub fn leave(&mut self) -> &mut Self {
        self.levels.pop();
        self
    }

    pub fn insert(&mut self, name: String, binding: Binding) -> &mut Self {
        match self.levels.last_mut() {
            Some(level) => {
                level.bindings.insert(name, binding);
            }
            None => panic!("Empty scope stack."),
        }

        self
    }

    pub fn set(&mut self, name: String, binding: Binding) -> &mut Self {
        if self.has_binding(&name) {
            replace(self.get_binding_mut(&name).unwrap(), binding);
        } else {
            self.insert(name, binding);
        }

        self
    }

    pub fn register_special_form(&mut self, name: &str, func: SpecialForm) -> &mut Self {
        self.insert(name.to_string(), Binding::SpecialForm(func))
    }

    pub fn register_eval_form(&mut self, name: &str, func: EvalForm) -> &mut Self {
        self.insert(name.to_string(), Binding::EvalForm(func))
    }
}

impl ScopeLevel {
    fn new() -> ScopeLevel {
        ScopeLevel {
            bindings: HashMap::new(),
            loop_info: None,
        }
    }

    fn new_loop() -> ScopeLevel {
        ScopeLevel {
            bindings: HashMap::new(),
            loop_info: Some(LoopInfo {
                break_called: false,
                boundary: false,
            }),
        }
    }

    fn new_loop_boundary() -> ScopeLevel {
        ScopeLevel {
            bindings: HashMap::new(),
            loop_info: Some(LoopInfo {
                break_called: false,
                boundary: true,
            }),
        }
    }
}
