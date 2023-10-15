pub mod impls;

use crate::impls::creational::{
    abstract_factory::abstract_factory::abstract_factory,
    factory_method::tools_factory_method,
};

fn main() {
    tools_factory_method();
    abstract_factory()
}
