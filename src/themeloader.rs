use std::collections::HashMap;
use std::fs::File;

use ron::de::from_reader;

use serde::{Deserialize, Serialize};

use hotwatch::{Event as HotwatchEvent, Hotwatch};

use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, Key, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Selector, UpdateCtx, Widget,
};

fn leak_string(from: String) -> &'static str {
    Box::leak(from.into_boxed_str())
}

use druid::piet::Color;

use druid::kurbo::Size;

use std::marker::PhantomData;

pub const RELOAD_STYLES: Selector = Selector::new("themeloader.reload_styles");

#[derive(Serialize, Deserialize, Clone, Debug)]
enum StyleValue {
    Color(u8, u8, u8, u8),
    Float(f64),
    String(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Stylesheet {
    map: HashMap<String, StyleValue>,
}

impl Stylesheet {
    fn set_all(&self, env: &Env) -> Env {
        let mut new_env = env.clone();
        let map = self.map.clone();

        for (old_key, val) in map.iter() {
            // Is this def necessary?
            let key = leak_string(old_key.clone());
            match val {
                StyleValue::Color(r, g, b, a) => {
                    let druid_key: Key<Color> = Key::new(&key);
                    let druid_value: Color = Color::rgba8(*r, *g, *b, *a);
                    new_env.set(druid_key, druid_value);
                }
                StyleValue::Float(float) => {
                    let druid_key: Key<f64> = Key::new(&key);
                    let druid_value: f64 = *float;
                    new_env.set(druid_key, druid_value);
                }
                StyleValue::String(string) => {
                    let druid_key: Key<&str> = Key::new(&key);
                    let druid_value: &str = string;
                    new_env.set(druid_key, druid_value);
                }
            }
        }

        new_env
    }
}

pub struct ThemeLoader<T: Data, W: Widget<T>> {
    style: Stylesheet,
    style_file: &'static str,
    child: W,
    phantom: PhantomData<T>,
}

pub fn watch(watch_file: &'static str, event_sink: druid::ExtEventSink) -> Hotwatch {
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch
        .watch(watch_file, move |event: HotwatchEvent| {
            if let HotwatchEvent::Write(_) = event {
                //TODO: why can't I send None as the object?
                if let Err(_) = event_sink.submit_command(RELOAD_STYLES, "None", None) {}
            }
        })
        .expect("failed to watch file!");

    hotwatch
}

impl<T: Data, W: Widget<T>> ThemeLoader<T, W> {
    pub fn new(child: W, style_file: &'static str) -> ThemeLoader<T, W> {
        let file = File::open(style_file).expect("Failed opening file");

        let stylesheet: Stylesheet = match from_reader(file) {
            Ok(style) => style,
            Err(e) => {
                eprintln!("Failed to load stylesheet: {}", e);

                std::process::exit(1);
            }
        };

        ThemeLoader {
            style: stylesheet,
            style_file,
            child,
            phantom: Default::default(),
        }
    }
}

impl<T: Data, W: Widget<T>> Widget<T> for ThemeLoader<T, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.selector == RELOAD_STYLES => {
                // let _ = cmd.get_object::<&str>().unwrap().clone();

                let style_file = File::open(self.style_file).expect("Failed opening file");

                let stylesheet: Stylesheet = match from_reader(style_file) {
                    Ok(style) => style,
                    Err(e) => {
                        eprintln!("Failed to load stylesheet: {}", e);

                        self.style.clone()
                    }
                };

                self.style = stylesheet;

                ctx.invalidate();
            }
            _ => (),
        }

        let new_env = self.style.set_all(&env);

        self.child.event(ctx, event, data, &new_env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        let new_env = self.style.set_all(&env);
        self.child.lifecycle(ctx, event, data, &new_env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        let new_env = self.style.set_all(&env);

        self.child.update(ctx, old_data, data, &new_env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        bc.debug_check("ThemeLoader");

        let new_env = self.style.set_all(&env);

        self.child.layout(layout_ctx, &bc, data, &new_env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        let new_env = self.style.set_all(&env);
        self.child.paint(paint_ctx, data, &new_env);
    }
}
