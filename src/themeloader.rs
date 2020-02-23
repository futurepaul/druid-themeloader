use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

use ron::de::from_reader;
use ron::ser::{to_string_pretty, PrettyConfig};

use serde::{Deserialize, Serialize};

use druid::{
    theme, BoxConstraints, Data, Env, Event, EventCtx, Key, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Selector, UpdateCtx, Value, Widget,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    child: W,
    phantom: PhantomData<T>,
}

impl<T: Data, W: Widget<T>> ThemeLoader<T, W> {
    // pub fn new(child: impl Widget<T> + 'static) -> impl Widget<T> {
    //     child.env_scope(|env, _| {
    //         let style = load_file("fonts.txt").unwrap();
    //         env.set(theme::FONT_NAME, style);
    //     })
    // }

    pub fn new(child: W) -> ThemeLoader<T, W> {
        let mut stylesheet = Stylesheet {
            map: HashMap::new(),
        };
        stylesheet.map.insert(
            "window_background_color".to_string(),
            StyleValue::Color(50, 50, 50, 50),
        );

        stylesheet.map.insert(
            "font_name".to_string(),
            StyleValue::String("Menlo".to_string()),
        );

        stylesheet
            .map
            .insert("text_size_normal".to_string(), StyleValue::Float(14.0));

        let pretty = PrettyConfig::default();
        let s = to_string_pretty(&stylesheet, pretty).expect("Serialization failed");

        println!("{}", s);

        // Actual stuff
        // let style = load_file("fonts.txt").unwrap();

        let style_file = File::open("styles.ron").expect("Failed opening file");

        let stylesheet: Stylesheet = match from_reader(style_file) {
            Ok(style) => style,
            Err(e) => {
                eprintln!("Failed to load stylesheet: {}", e);

                std::process::exit(1);
            }
        };

        dbg!(stylesheet.clone());

        ThemeLoader {
            style: stylesheet,
            child,
            phantom: Default::default(),
        }
    }
}

impl<T: Data, W: Widget<T>> Widget<T> for ThemeLoader<T, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.selector == RELOAD_STYLES => {
                dbg!("command just happened");

                let path = cmd.get_object::<&str>().unwrap().clone();
                dbg!(path);

                // let style = load_file(path).unwrap();

                // let stylesheet_file = load_file("styles.ron").unwrap();

                let style_file = File::open(path).expect("Failed opening file");

                let stylesheet: Stylesheet = match from_reader(style_file) {
                    Ok(style) => style,
                    Err(e) => {
                        eprintln!("Failed to load stylesheet: {}", e);

                        std::process::exit(1);
                    }
                };

                self.style = stylesheet;
                // self.style = style;

                // // let style = load_file("fonts.txt").unwrap();
                // self.s
                // let mut new_env = env.clone();
                // new_env.set(theme::FONT_NAME, style);
                // // self.style = "Menlo".to_string();
                // dbg!("Nice");
                // ctx.request_paint();
                // self.child.event(ctx, event, data, &new_env);
                ctx.request_paint();
            }
            _ => (),
        }

        let new_env = self.style.set_all(&env);

        // new_env.set(theme::FONT_NAME, self.style.clone());

        self.child.event(ctx, event, data, &new_env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        // let mut new_env = env.clone();
        // self.style.set_all(&mut new_env);
        let new_env = self.style.set_all(&env);
        self.child.lifecycle(ctx, event, data, &new_env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        // let mut new_env = env.clone();
        // self.style.set_all(&mut new_env);
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
        // let mut new_env = env.clone();
        // self.style.set_all(&mut new_env);
        let new_env = self.style.set_all(&env);

        self.child.layout(layout_ctx, &bc, data, &new_env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        // let mut new_env = env.clone();
        // self.style.set_all(&mut new_env);
        let new_env = self.style.set_all(&env);
        self.child.paint(paint_ctx, data, &new_env);
    }
}
