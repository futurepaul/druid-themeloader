// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::thread;
use std::time::{Duration, Instant};

use druid::piet::Color;
use druid::widget::{Button, Flex, Label, WidgetExt};
use druid::{
    theme, AppLauncher, ExtEventError, LocalizedString, PlatformError, Widget, WindowDesc,
};

use druid_themeloader::{ThemeLoader, RELOAD_STYLES};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("hello-demo-window-title").with_placeholder("Hello World!"));
    let data = 0_u32;
    let launcher = AppLauncher::with_window(main_window);

    let event_sink = launcher.get_external_handle();

    thread::spawn(move || {
        loop {
            // let time_since_start = Instant::now() - start_time;

            // there is no logic here; it's a very silly way of creating a color.

            // if this fails we're shutting down
            if let Err(_) = event_sink.submit_command(RELOAD_STYLES, "styles.ron", None) {
                break;
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });

    launcher.use_simple_logger().launch(data)?;

    Ok(())
}

fn ui_builder() -> impl Widget<u32> {
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());

    let label = Label::new(text.clone())
        .padding(5.0)
        .border(Color::WHITE, 1.0);

    let button = Button::new("increment", |_ctx, data, _env| *data += 1);

    ThemeLoader::new(
        Flex::column()
            .with_child(label.center(), 1.0)
            .with_child(button.padding(5.0), 1.0),
    )
}
