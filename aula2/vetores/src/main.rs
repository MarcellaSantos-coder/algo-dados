fn main() {
    println!("Hello, world!");
}

#para declarar uma estrtutura no rust utilizamos pub struct 

pub struct MyVec {
    data: Vec<i32>,
}

impl MyVec {
    pub fn new() -> MyVec {
        MyVec { data: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&i32> {
        self.data.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}