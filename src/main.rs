extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


fn main()  {
    yew::start_app::<kana_kilo::App>();
}
