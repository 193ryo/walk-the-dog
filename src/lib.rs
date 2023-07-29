use std::sync::Arc;

use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;

use rand::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/*
　三角形を描画する
*/
fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], 
                    color: (u8, u8, u8),) {
    
    // 符号なし整数のタプルをfill_styleの形式にフォーマット変換
    let color_str = format!("rgb({}, {}, {}", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));

    // 描画
    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
    context.fill();
}

/*
　座標を指定し、三角形を描画する
*/
fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], 
                color: (u8, u8, u8), depth: u8) {

    // 外枠の三角形を描画する
    draw_triangle(&context, points, color);

    // 各座標の指定
    let [top, left, right] = points;

    // 再帰描画回数
    let depth = depth - 1;

    // 三角形を描画する
    if depth > 0 {

        let mut rng = thread_rng();

        let next_color = (
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );

        let left_middle = midpoint(top, left);
        let right_middle = midpoint(top, right);
        let bottom_middle = midpoint(left, right);
        sierpinski(&context, [top, left_middle, right_middle], next_color, depth);
        sierpinski(&context, [left_middle, left, bottom_middle], next_color, depth);
        sierpinski(&context, [right_middle, bottom_middle, right], next_color, depth);
    }
}

/*
　三角形の中心点を割出す
*/
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
    wasm_bindgen_futures::spawn_local(async move {
        let (success_tx, success_rx) 
                = futures::channel::oneshot::channel::<()>();
        let image = web_sys::HtmlImageElement::new().unwrap();
        let callback = Closure::once(move || {
            success_tx.send(());
            web_sys::console::log_1(&JsValue::from_str("loaded"));
        });
        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_src("Idle (1).png");
        success_rx.await;
        context.draw_image_with_html_image_element(&image, 0.0, 0.0);
        sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], (0, 255, 0), 5);
    });

    Ok(())
}
