use crate::api_handler::router;

pub mod api_handler;

fn main(){
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
