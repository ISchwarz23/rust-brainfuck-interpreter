use std::io;
use std::io::Write;
use crate::parser::Expression;

pub fn execute(expressions: Vec<Expression>) -> Result<(), RuntimeError> {
    let mut memory = Memory::new();
    execute_internal(&mut memory, &expressions)
}

fn execute_internal(
    memory: &mut Memory,
    expressions: &Vec<Expression>,
) -> Result<(), RuntimeError> {
    for expression in expressions {
        match expression {
            Expression::MovePointer(x) => {
                let result = memory.move_pointer(x);
                if result.is_err() {
                    return result;
                }
            }
            Expression::ModifyRegister(x) => {
                memory.modify_current_register_value(x);
            }
            Expression::Loop(exp) => {
                while memory.get_current_register_value() != 0 {
                    let result = execute_internal(memory, &exp);
                    if result.is_err() {
                        return result;
                    }
                }
            }
            Expression::PrintRegister => {
                if let Some(character) = char::from_u32(memory.get_current_register_value() as u32)
                {
                    print!("{}", character);
                } else {
                    return Err(RuntimeError::UnableToPrintValueAsCharacter());
                }
            }
            Expression::ReadToRegister => {
                let value = read_value_from_stdin();
                memory.set_current_register_value(value);
            }
        }
    }
    Ok(())
}

fn read_value_from_stdin() -> isize {
    loop {
        // Prompt the user
        print!("Please enter a value: ");
        io::stdout().flush().unwrap();

        // Create a mutable String to hold input
        let mut input = String::new();

        // Read input from the standard input
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Try to parse the input into an isize
                match input.trim().parse::<isize>() {
                    Ok(number) => return number, // Return the parsed number on success
                    Err(_) => {
                        println!("Invalid input. Please try again.");
                    }
                }
            }
            Err(_) => {
                println!("Failed to read input. Please try again.");
            }
        }
    }
}


#[derive(Debug)]
struct Memory {
    segments: Vec<isize>,
    pointer_index: usize,
}

#[derive(Debug)]
pub enum RuntimeError {
    IllegalMemoryAccess(),
    UnableToPrintValueAsCharacter(),
}

impl RuntimeError {
    pub(crate) fn message(&self) -> String {
        match self {
            RuntimeError::IllegalMemoryAccess() => String::from("Pointer moved out of memory."),
            RuntimeError::UnableToPrintValueAsCharacter() => {
                String::from("Unable to print register value as character.")
            }
        }
    }
}

impl Memory {
    fn new() -> Memory {
        Memory {
            segments: Vec::from([0]),
            pointer_index: 0,
        }
    }

    fn move_pointer(&mut self, value: &isize) -> Result<(), RuntimeError> {
        let new_pointer_index = self.pointer_index as isize + value;
        if new_pointer_index < 0 {
            return Err(RuntimeError::IllegalMemoryAccess());
        }
        if new_pointer_index >= self.segments.len() as isize {
            let mut number_of_new_segments = new_pointer_index - self.segments.len() as isize + 1;
            while number_of_new_segments > 0 {
                self.segments.push(0);
                number_of_new_segments -= 1;
            }
        }
        self.pointer_index = new_pointer_index as usize;

        Ok(())
    }

    fn modify_current_register_value(&mut self, value: &isize) {
        self.segments[self.pointer_index] += value;
    }

    fn get_current_register_value(&self) -> isize {
        self.segments[self.pointer_index]
    }

    fn set_current_register_value(&mut self, value: isize) {
        self.segments[self.pointer_index] = value;
    }
}
