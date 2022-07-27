extern crate core;

use crate::api_handler::router;

pub mod api_handler;
pub mod calculation_logic;

fn main() {
    router();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
