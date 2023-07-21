use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use wasm_bindgen::prelude::*;

mod game;
mod random;

use game::{SnakeGame,Direction};
use web_sys::{window, HtmlDivElement, HtmlElement, KeyboardEvent};

thread_local! {
    static SNAKE_GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(25, 25)));

    static CLOSURE: Closure<dyn FnMut()> =  Closure::wrap(Box::new(|| {
        SNAKE_GAME.with(|snake_game| snake_game.borrow_mut().tick());
        render();
    }) as Box<dyn FnMut()>);

    static HANDLE_KEYBOARD: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new({
        |event: KeyboardEvent| {
            SNAKE_GAME.with(|snake_game| {
                let direction_eq = match &event.key()[..] {
                "ArrowUp" => Some(Direction::Up),
                "ArrowRight" => Some(Direction::Right),
                "ArrowDown" => Some(Direction::Down),
                "ArrowLeft" => Some(Direction::Left),
                _ => None
            };
            if let Some(direction) = direction_eq{
                snake_game.borrow_mut().change_direction(direction)
            }
        });
        }
    }) as Box<dyn FnMut(KeyboardEvent)>)
}

#[wasm_bindgen(start)]
pub fn main() {

    CLOSURE.with(|cl| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                cl.as_ref().dyn_ref::<Function>().unwrap_throw(),
                150,
            )
            .unwrap_throw()
    });

    HANDLE_KEYBOARD.with(|handle| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown",
                handle.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });
    render();
}

pub fn render() {
    let document = window().unwrap_throw().document().unwrap_throw();

    let root_element = document
        .get_element_by_id("root")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();

    root_element.set_inner_html("");
    root_element
        .style()
        .set_property("display", "inline-grid")
        .unwrap_throw();

    let score_count = SNAKE_GAME.with(|g| g.borrow().score);
    let score_el = document.get_element_by_id("score")
    .unwrap_throw()
    .dyn_into::<HtmlElement>()
    .unwrap_throw();
    score_el.set_inner_html(&format!("score:{}",score_count));


    let height = SNAKE_GAME.with(|g| g.borrow().height);
    let width = SNAKE_GAME.with(|g| g.borrow().width);

    root_element
        .style()
        .set_property(
            "grid-template",
            &format!("repeat({}, auto) / repeat({}, auto)", height, width),
        )
        .unwrap_throw();

    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);

            let field_el = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<HtmlDivElement>()
                .unwrap_throw();

            field_el.set_class_name("field");
            field_el.set_inner_text({
                if pos == SNAKE_GAME.with(|g| g.borrow().food) {
                    "🐀"
                }
                else if SNAKE_GAME.with(|g| g.borrow().snake.get(0) == Some(&pos)) {
                    "⚫️"
                }
                else if SNAKE_GAME.with(|g| g.borrow().snake.contains(&pos)) {
                    "⬛️"
                } else {
                    "⬜️"
                }
            });
            root_element.append_child(&field_el).unwrap_throw();
        }
    }
}
