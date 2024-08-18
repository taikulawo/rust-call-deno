use deno_bindgen::deno_bindgen;

#[deno_bindgen]
pub struct Foo;

#[deno_bindgen]
impl Foo {
    #[constructor]
    pub fn new() -> Self {
        Self
    }

    pub fn bar(&self) {
        // ...
    }
}
