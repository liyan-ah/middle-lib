use std::sync::Arc;
use trait_lib::check_trait as lib_check_trait;
use trait_lib::Check;

struct MiddleCheck {
    value: i32,
}

impl Check for MiddleCheck {
    fn check(&self, arg: i32) -> bool {
        self.value + 1 > arg
    }
}

pub fn new_check(value: i32) -> Arc<Box<dyn Check>> {
    Arc::new(Box::new(MiddleCheck { value }))
}

pub fn check_trait(check: Arc<Box<dyn Check>>, arg: i32) -> bool {
    lib_check_trait(check, arg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let check = new_check(10);
        assert!(check_trait(check, 10));
    }
}
