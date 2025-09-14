use crate::prelude::*;

mod product;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/product").configure(product::config));
}
