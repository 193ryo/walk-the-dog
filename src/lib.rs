use std::sync::Arc;

use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/*
　三角形を描画する
*/
fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3]){
    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
}

/*
　座標を指定し、三角形を描画する
*/
fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], depth: u8) {

    // 外枠の三角形を描画する
    draw_triangle(&context, points);

    // 各座標の指定
    let [top, left, right] = points;

    // 再帰描画回数
    let depth = depth - 1;

    // 三角形を描画する
    if depth > 0 {
        let left_middle = midpoint(top, left);
        let right_middle = midpoint(top, right);
        let bottom_middle = midpoint(left, right);
        sierpinski(&context, [top, left_middle, right_middle], depth);
        sierpinski(&context, [left_middle, left, bottom_middle], depth);
        sierpinski(&context, [right_middle, bottom_middle, right], depth);
    }
}

// 三角形の中心点を割出す
fn midpoint(point_1: (f64, f64), point_2: (f64, f64)) -> (f64, f64) {
    ((point_1.0 + point_2.0) / 2.0, (point_1.1 + point_2.1) / 2.0)
}

/*
　シェルピンスキーの三角形を描画する
*/
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    // パニック時の内容を取得する
    console_error_panic_hook::set_once();

    // windowからcanvasを取得
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // 描画する
    sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 2);

    Ok(())
}
