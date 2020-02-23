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

use druid::piet::Color;
use druid::widget::{Button, Flex, Label, WidgetExt};
use druid::{AppLauncher, Key, LocalizedString, PlatformError, Widget, WindowDesc};

use druid_themeloader::{watch, ThemeLoader};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("hello-demo-window-title").with_placeholder("Hello World!"));
    let data = 0_u32;
    let launcher = AppLauncher::with_window(main_window);

    let mut _watch = watch("styles.ron", launcher.get_external_handle());

    launcher.use_simple_logger().launch(data)?;

    Ok(())
}

fn ui_builder() -> impl Widget<u32> {
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());

    let custom_key_color: Key<Color> = Key::new("local.border-color");
    let custom_key_width: Key<f64> = Key::new("local.border-width");

    let label = Label::new(text.clone())
        .padding(5.0)
        .border(custom_key_color, custom_key_width);

    let button = Button::new("increment", |_ctx, data, _env| *data += 1);

    ThemeLoader::new(
        Flex::column()
            .with_child(label.center(), 1.0)
            .with_child(button.padding(5.0), 1.0),
        "styles.ron",
    )
}
