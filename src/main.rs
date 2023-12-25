mod swapper;
mod cart;
mod joker6;
mod io;

use cart::{Cart4BitsForwardHandler};
use cart::CartHandler;


fn main() {
    let cart_handler = Cart4BitsForwardHandler::new(19);

}

