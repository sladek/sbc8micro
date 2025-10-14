const MAX_BREAKPOINTS: usize = 10;

#[derive(Default, Clone)]
pub struct Breakpoints {
    breakpoints: Vec<u16>,
    max_breakpoints: usize,
}

impl Breakpoints {
    pub fn new() -> Self {
        Self {
            breakpoints: Vec::new(),
            max_breakpoints: MAX_BREAKPOINTS,
        }
    }
    /// Check if breakpoint is set
    ///
    /// Checks vector of breakpoints for the presence of specified address
    pub fn check_breakpoint(&self, address: u16) -> bool {
        if self.breakpoints.is_empty() {
            return false;
        }
        self.breakpoints.clone().into_iter().any(|x| x == address)
    }
    /// Gets list of all breakpoints
    pub fn get_breakpoints(&self) -> Vec<u16> {
        self.breakpoints.clone()
    }
    /// Sets or remove a breakpoint
    /// 
    /// If breakpoint is not set it is set otherwise it is removed.
    pub fn set_breakpoint(&mut self, address: u16) -> Result<(), String> {
        if self.breakpoints.len() == self.max_breakpoints {
            return Err(format!(
                "Maximal number of breakpoints ({}) reached.",
                self.max_breakpoints
            )
            .to_string());
        }
        match self.breakpoints.iter().find(|&addr| *addr == address) {
            Some(address) => {
                let addr = *address;
                self.remove_breakpoint(addr)?;
                return Err(format!(
                    "Breakpoint: 0x{:04X} [{}] has been removed.",
                    &addr, &addr
                ));
            }
            None => {
                self.breakpoints.push(address);
                self.breakpoints.sort();
            }
        }
        Ok(())
    }
    /// Clears all the breakpoints
    pub fn clear_breakpoints(&mut self) -> Result<(), String> {
        self.breakpoints.clear();
        Ok(())
    }
    /// Removes specific breakpoint
    fn remove_breakpoint(&mut self, address: u16) -> Result<(), String> {
        match self.breakpoints.iter().position(|x| *x == address) {
            Some(i) => {
                self.breakpoints.swap_remove(i);
                Ok(())
            }
            None => Err(format!("Breakpoint: {address} not defined.")),
        }
    }
}
