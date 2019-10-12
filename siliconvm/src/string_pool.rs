use std::collections::HashSet;
use std::rc::Rc;

pub struct StringPool {
    pool: HashSet<Rc<String>>
}

impl StringPool {
    pub(crate) fn new() -> StringPool {
        StringPool {
            pool: Default::default()
        }
    }

    pub(crate) fn pool_string(&mut self, to_pool: String) -> Rc<String> {
        return match self.pool.get(&to_pool) {
            Some(rc) => Rc::clone(rc),
            None => {
                let new_rc: Rc<String> = Rc::from(to_pool);
                self.pool.insert(Rc::clone(&new_rc));
                new_rc
            }
        }
    }

    pub(crate) fn pool_str(&mut self, to_pool: &str) -> Rc<String> {
        self.pool_string(to_pool.to_string())
    }
}