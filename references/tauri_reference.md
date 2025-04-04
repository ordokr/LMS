
TITLE: Implementing File Streaming with Tauri Channels
DESCRIPTION: Example of using Tauri channels to stream data from Rust to JavaScript. This implementation reads a file in chunks and sends each chunk through the channel to the frontend.

LANGUAGE: rust
CODE:
use tokio::io::AsyncReadExt;

#[tauri::command]
async fn load_image(path: std::path::PathBuf, reader: tauri::ipc::Channel<&[u8]>) {
  // for simplicity this example does not include error handling
  let mut file = tokio::fs::File::open(path).await.unwrap();

  let mut chunk = vec![0; 4096];

  loop {
    let len = file.read(&mut chunk).await.unwrap();
    if len == 0 {
      // Length of zero means end of file.
      break;
    }
    reader.send(&chunk).unwrap();
  }
}

----------------------------------------

TITLE: Defining Basic Tauri Command in Rust
DESCRIPTION: Creates a simple command in the lib.rs file using the #[tauri::command] annotation and registers it with the Tauri application builder. Commands must be unique and cannot be marked as pub in lib.rs.

LANGUAGE: rust
CODE:
#[tauri::command]
fn my_custom_command() {
	println!("I was invoked from JavaScript!");
}

----------------------------------------

TITLE: Invoking Tauri Commands from JavaScript
DESCRIPTION: Shows how to invoke Rust commands from JavaScript using either the Tauri API npm package or the global Tauri script. The invoke function takes the command name and optional parameters.

LANGUAGE: javascript
CODE:
// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/core';

// When using the Tauri global script (if not using the npm package)
// Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.core.invoke;

// Invoke the command
invoke('my_custom_command');

----------------------------------------

TITLE: JavaScript for Custom Titlebar Button Functionality
DESCRIPTION: JavaScript code to make custom titlebar buttons functional, using Tauri's window API to implement minimize, maximize, and close operations.

LANGUAGE: javascript
CODE:
import { getCurrentWindow } from '@tauri-apps/api/window';

// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;

const appWindow = getCurrentWindow();

document
  .getElementById('titlebar-minimize')
  ?.addEventListener('click', () => appWindow.minimize());
document
  .getElementById('titlebar-maximize')
  ?.addEventListener('click', () => appWindow.toggleMaximize());
document
  .getElementById('titlebar-close')
  ?.addEventListener('click', () => appWindow.close());

----------------------------------------

TITLE: Installing Rust on Linux and macOS
DESCRIPTION: Command to install the Rust programming language and its toolchain using rustup on Linux and macOS systems.

LANGUAGE: sh
CODE:
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

----------------------------------------

TITLE: Implementing Updater Command with Channel in Rust for Tauri
DESCRIPTION: This advanced Rust implementation creates a command-based update system with channels for notifying the frontend about download progress. It includes error handling, download event types, and separate functions for fetching and installing updates.

LANGUAGE: rust
CODE:
#[cfg(desktop)]
mod app_updates {
    use std::sync::Mutex;
    use serde::Serialize;
    use tauri::{ipc::Channel, AppHandle, State};
    use tauri_plugin_updater::{Update, UpdaterExt};

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Updater(#[from] tauri_plugin_updater::Error),
        #[error("there is no pending update")]
        NoPendingUpdate,
    }

    impl Serialize for Error {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.to_string().as_str())
        }
    }

    type Result<T> = std::result::Result<T, Error>;

    #[derive(Clone, Serialize)]
    #[serde(tag = "event", content = "data")]
    pub enum DownloadEvent {
        #[serde(rename_all = "camelCase")]
        Started {
            content_length: Option<u64>,
        },
        #[serde(rename_all = "camelCase")]
        Progress {
            chunk_length: usize,
        },
        Finished,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UpdateMetadata {
        version: String,
        current_version: String,
    }

    #[tauri::command]
    pub async fn fetch_update(
        app: AppHandle,
        pending_update: State<'_, PendingUpdate>,
    ) -> Result<Option<UpdateMetadata>> {
        let channel = "stable";
        let url = url::Url::parse(&format!(
            "https://cdn.myupdater.com/{{{{target}}}}-{{{{arch}}}}/{{{{current_version}}}}?channel={channel}",
        )).expect("invalid URL");

      let update = app
          .updater_builder()
          .endpoints(vec![url])?
          .build()?
          .check()
          .await?;

      let update_metadata = update.as_ref().map(|update| UpdateMetadata {
          version: update.version.clone(),
          current_version: update.current_version.clone(),
      });

      *pending_update.0.lock().unwrap() = update;

      Ok(update_metadata)
    }

    #[tauri::command]
    pub async fn install_update(pending_update: State<'_, PendingUpdate>, on_event: Channel<DownloadEvent>) -> Result<()> {
        let Some(update) = pending_update.0.lock().unwrap().take() else {
            return Err(Error::NoPendingUpdate);
        };

        let started = false;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    if !started {
                        let _ = on_event.send(DownloadEvent::Started { content_length });
                        started = true;
                    }

                    let _ = on_event.send(DownloadEvent::Progress { chunk_length });
                },
                || {
                    let _ = on_event.send(DownloadEvent::Finished);
                },
            )
            .await?;

        Ok(())
    }

    struct PendingUpdate(Mutex<Option<Update>>);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                app.handle().plugin(tauri_plugin_updater::Builder::new().build());
                app.manage(app_updates::PendingUpdate(Mutex::new(None)));
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            #[cfg(desktop)]
            app_updates::fetch_update,
            #[cfg(desktop)]
            app_updates::install_update
        ])
}

----------------------------------------

TITLE: Complete Tauri Command Example with Multiple Features
DESCRIPTION: A comprehensive example combining various Tauri command features: async, window access, state management, custom return types, and error handling.

LANGUAGE: rust
CODE:
struct Database;

#[derive(serde::Serialize)]
struct CustomResponse {
	message: String,
	other_val: usize,
}

async fn some_other_function() -> Option<String> {
	Some("response".into())
}

#[tauri::command]
async fn my_custom_command(
	window: tauri::Window,
	number: usize,
	database: tauri::State<'_, Database>,
) -> Result<CustomResponse, String> {
	println!("Called from {}", window.label());
	let result: Option<String> = some_other_function().await;
	if let Some(message) = result {
		Ok(CustomResponse {
			message,
			other_val: 42 + number,
		})
	} else {
		Err("No result".into())
	}
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.manage(Database {})
		.invoke_handler(tauri::generate_handler![my_custom_command])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

----------------------------------------

TITLE: Passing Arguments to Tauri Commands
DESCRIPTION: Shows how to define a Tauri command that accepts arguments. Command arguments are passed as a JSON object from JavaScript to Rust and should implement serde::Deserialize.

LANGUAGE: rust
CODE:
#[tauri::command]
fn my_custom_command(invoke_message: String) {
	println!("I was invoked from JavaScript, with this message: {}", invoke_message);
}

----------------------------------------

TITLE: Registering Tauri Commands in Application Builder
DESCRIPTION: Demonstrates how to register command handlers with the Tauri application builder. The invoke_handler method accepts a list of commands generated by the tauri::generate_handler! macro.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![my_custom_command])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

----------------------------------------

TITLE: Using Stronghold from JavaScript
DESCRIPTION: Complete example showing how to use the Stronghold plugin from JavaScript, including initializing a vault, creating a client, and performing store operations (insert, read, and remove data).

LANGUAGE: javascript
CODE:
import { Client, Stronghold } from '@tauri-apps/plugin-stronghold';
// when using `"withGlobalTauri": true`, you may use
// const { Client, Stronghold } = window.__TAURI__.stronghold;
import { appDataDir } from '@tauri-apps/api/path';
// when using `"withGlobalTauri": true`, you may use
// const { appDataDir } = window.__TAURI__.path;

const initStronghold = async () => {
	const vaultPath = `${await appDataDir()}/vault.hold`;
	const vaultPassword = 'vault password';
	const stronghold = await Stronghold.load(vaultPath, vaultPassword);

	let client: Client;
	const clientName = 'name your client';
	try {
		client = await stronghold.loadClient(clientName);
	} catch {
		client = await stronghold.createClient(clientName);
	}

	return {
		stronghold,
		client,
	};
};

// Insert a record to the store
async function insertRecord(store: any, key: string, value: string) {
	const data = Array.from(new TextEncoder().encode(value));
	await store.insert(key, data);
}

// Read a record from store
async function getRecord(store: any, key: string): Promise<string> {
	const data = await store.get(key);
	return new TextDecoder().decode(new Uint8Array(data));
}

const { stronghold, client } = await initStronghold();

const store = client.getStore();
const key = 'my_key';

// Insert a record to the store
insertRecord(store, key, 'secret value');

// Read a record from store
const value = await getRecord(store, key);
console.log(value); // 'secret value'

// Save your updates
await stronghold.save();

// Remove a record from store
await store.remove(key);

----------------------------------------

TITLE: Base Tauri Configuration for Platform-specific Example
DESCRIPTION: Example of a base tauri.conf.json file that defines the product name, bundle resources, and a plugin configuration to demonstrate platform-specific configuration merging.

LANGUAGE: json
CODE:
{
  "productName": "MyApp",
  "bundle": {
    "resources": ["./resources"]
  },
  "plugins": {
    "deep-link": {}
  }
}

----------------------------------------

TITLE: Registering Multiple Commands in Tauri
DESCRIPTION: Example of defining and registering multiple commands with the Tauri application. All commands must be passed to a single invoke_handler call using the tauri::generate_handler! macro.

LANGUAGE: rust
CODE:
#[tauri::command]
fn cmd_a() -> String {
	"Command a"
}
#[tauri::command]
fn cmd_b() -> String {
	"Command b"
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![cmd_a, cmd_b])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring tasks.json for Tauri UI Commands
DESCRIPTION: This tasks.json file defines background tasks that run the UI development server and build commands. These tasks are referenced by the launch configurations to ensure the UI is properly prepared before debugging.

LANGUAGE: json
CODE:
{
  // See https://go.microsoft.com/fwlink/?LinkId=733558
  // for the documentation about the tasks.json format
  "version": "2.0.0",
  "tasks": [
    {
      "label": "ui:dev",
      "type": "shell",
      // `dev` keeps running in the background
      // ideally you should also configure a `problemMatcher`
      // see https://code.visualstudio.com/docs/editor/tasks#_can-a-background-task-be-used-as-a-prelaunchtask-in-launchjson
      "isBackground": true,
      // change this to your `beforeDevCommand`:
      "command": "yarn",
      "args": ["dev"]
    },
    {
      "label": "ui:build",
      "type": "shell",
      // change this to your `beforeBuildCommand`:
      "command": "yarn",
      "args": ["build"]
    }
  ]
}

----------------------------------------

TITLE: Defining a Capability in JSON for Main Window
DESCRIPTION: This JSON snippet defines a capability named 'main-capability' for the main window, enabling default functionality for core plugins and the window.setTitle API. Capabilities are stored in the src-tauri/capabilities directory.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:path:default",
    "core:event:default",
    "core:window:default",
    "core:app:default",
    "core:resources:default",
    "core:menu:default",
    "core:tray:default",
    "core:window:allow-set-title"
  ]
}

----------------------------------------

TITLE: Accessing State in Async Tauri Commands
DESCRIPTION: Demonstrates how to access and modify state in asynchronous Tauri commands, using Tokio's async Mutex. Asynchronous commands must return a Result type.

LANGUAGE: rust
CODE:
#[tauri::command]
async fn increase_counter(state: State<'_, Mutex<AppState>>) -> Result<u32, ()> {
  let mut state = state.lock().await;
  state.counter += 1;
  Ok(state.counter)
}

----------------------------------------

TITLE: Using CLI Plugin in Rust
DESCRIPTION: Rust implementation showing how to access and use CLI argument matches. It initializes the plugin, accesses the matches in the setup function, and processes them accordingly. Includes detailed comments about the structure of matches.

LANGUAGE: rust
CODE:
use tauri_plugin_cli::CliExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
   tauri::Builder::default()
       .plugin(tauri_plugin_cli::init())
       .setup(|app| {
           match app.cli().matches() {
               // `matches` here is a Struct with { args, subcommand }.
               // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
               // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
               Ok(matches) => {
                   println!("{:?}", matches)
               }
               Err(_) => {}
           }
           Ok(())
       })
       .run(tauri::generate_context!())
       .expect("error while running tauri application");
}

----------------------------------------

TITLE: Complete Multi-Platform Tauri Build and Release Workflow
DESCRIPTION: A complete GitHub Actions workflow that builds and releases a Tauri application for Linux x64, Windows x64, macOS x64, and macOS Arm64 platforms. The workflow installs dependencies, sets up Node.js and Rust with caching, and uses tauri-action to create GitHub releases.

LANGUAGE: yaml
CODE:
name: 'publish'

on:
  workflow_dispatch:
  push:
    branches:
      - release

jobs:
  publish-tauri:
    permissions:
      contents: write
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: 'macos-latest' # for Arm based macs (M1 and above).
            args: '--target aarch64-apple-darwin'
          - platform: 'macos-latest' # for Intel based macs.
            args: '--target x86_64-apple-darwin'
          - platform: 'ubuntu-22.04'
            args: ''
          - platform: 'windows-latest'
            args: ''

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4

      - name: install dependencies (ubuntu only)
        if: matrix.platform == 'ubuntu-22.04' # This must match the platform value defined above.
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

      - name: setup node
        uses: actions/setup-node@v4
        with:
          node-version: lts/*
          cache: 'yarn' # Set this to npm, yarn or pnpm.

      - name: install Rust stable
        uses: dtolnay/rust-toolchain@stable # Set this to dtolnay/rust-toolchain@nightly
        with:
          # Those targets are only used on macos runners so it's in an `if` to slightly speed up windows and linux builds.
          targets: ${{ matrix.platform == 'macos-latest' && 'aarch64-apple-darwin,x86_64-apple-darwin' || '' }}

      - name: Rust cache
        uses: swatinem/rust-cache@v2
        with:
          workspaces: './src-tauri -> target'

      - name: install frontend dependencies
        # If you don't have `beforeBuildCommand` configured you may want to build your frontend here too.
        run: yarn install # change this to npm or pnpm depending on which one you use.

      - uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: app-v__VERSION__ # the action automatically replaces \_\_VERSION\_\_ with the app version.
          releaseName: 'App v__VERSION__'
          releaseBody: 'See the assets to download this version and install.'
          releaseDraft: true
          prerelease: false
          args: ${{ matrix.args }}

----------------------------------------

TITLE: Error Handling in Tauri Commands
DESCRIPTION: Demonstrates how to handle errors in Tauri commands by returning a Result type. This allows the command to either resolve or reject the JavaScript promise based on the operation's success.

LANGUAGE: rust
CODE:
#[tauri::command]
fn login(user: String, password: String) -> Result<String, String> {
	if user == "tauri" && password == "tauri" {
		// resolve
		Ok("logged_in".to_string())
	} else {
		// reject
		Err("invalid credentials".to_string())
	}
}

----------------------------------------

TITLE: Checking and Installing Updates with JavaScript in Tauri
DESCRIPTION: This snippet demonstrates how to check for updates, download and install them, and track the download progress using the Tauri updater plugin in JavaScript. It handles different download events and relaunches the application after installation.

LANGUAGE: javascript
CODE:
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

const update = await check();
if (update) {
  console.log(
    `found update ${update.version} from ${update.date} with notes ${update.body}`
  );
  let downloaded = 0;
  let contentLength = 0;
  // alternatively we could also call update.download() and update.install() separately
  await update.downloadAndInstall((event) => {
    switch (event.event) {
      case 'Started':
        contentLength = event.data.contentLength;
        console.log(`started downloading ${event.data.contentLength} bytes`);
        break;
      case 'Progress':
        downloaded += event.data.chunkLength;
        console.log(`downloaded ${downloaded} from ${contentLength}`);
        break;
      case 'Finished':
        console.log('download finished');
        break;
    }
  });

  console.log('update installed');
  await relaunch();
}

----------------------------------------

TITLE: Selecting UI Template for TypeScript/JavaScript Frontend
DESCRIPTION: Command prompt showing the UI template options available for TypeScript/JavaScript frontends in a Tauri project, followed by language flavor selection between TypeScript and JavaScript.

LANGUAGE: bash
CODE:
? Choose your UI template ›
Vanilla
Vue
Svelte
React
Solid
Angular
Preact

? Choose your UI flavor ›
TypeScript
JavaScript

----------------------------------------

TITLE: Accessing State with Manager Trait in Event Handlers
DESCRIPTION: Demonstrates how to access application state outside of commands, such as in window event handlers, using the Manager trait. This is useful when state access is needed in contexts where command injection isn't available.

LANGUAGE: rust
CODE:
use std::sync::Mutex;
use tauri::{Builder, Window, WindowEvent, Manager};

#[derive(Default)]
struct AppState {
  counter: u32,
}

// In an event handler:
fn on_window_event(window: &Window, _event: &WindowEvent) {
    // Get a handle to the app so we can get the global state.
    let app_handle = window.app_handle();
    let state = app_handle.state::<Mutex<AppState>>();

    // Lock the mutex to mutably access the state.
    let mut state = state.lock().unwrap();
    state.counter += 1;
}

fn main() {
  Builder::default()
    .setup(|app| {
      app.manage(Mutex::new(AppState::default()));
      Ok(())
    })
    .on_window_event(on_window_event)
    .run(tauri::generate_context!())
    .unwrap();
}

----------------------------------------

TITLE: Implementing Mutable State with Mutex in Tauri
DESCRIPTION: Demonstrates how to create mutable application state using Rust's interior mutability pattern with Mutex. This allows safe state modification across multiple threads.

LANGUAGE: rust
CODE:
use std::sync::Mutex;

use tauri::{Builder, Manager};

#[derive(Default)]
struct AppState {
  counter: u32,
}

fn main() {
  Builder::default()
    .setup(|app| {
      app.manage(Mutex::new(AppState::default()));
      Ok(())
    })
    .run(tauri::generate_context!())
    .unwrap();
}

----------------------------------------

TITLE: Creating Multi-Level Menu in Rust
DESCRIPTION: Implements a multi-level menu in Rust with file submenu, language selection submenu with checkboxes, and an icon item. Demonstrates how to create and organize nested menu structures with different item types.

LANGUAGE: rust
CODE:
use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, SubmenuBuilder};

fn main() {
  tauri::Builder::default()
        .setup(|app| {
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("open", "Open")
                .text("quit", "Quit")
                .build()?;

            let lang_str = "en";
            let check_sub_item_1 = CheckMenuItemBuilder::new("English")
                .id("en")
                .checked(lang_str == "en")
                .build(app)?;

            let check_sub_item_2 = CheckMenuItemBuilder::new("Chinese")
                .id("en")
                .checked(lang_str == "en")
                .enabled(false)
                .build(app)?;

             // Load icon from path
            let icon_image = Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap();

            let icon_item = IconMenuItemBuilder::new("icon")
                .icon(icon_image)
                .build(app)?;

            let other_item = SubmenuBuilder::new(app, "language")
                .item(&check_sub_item_1)
                .item(&check_sub_item_2)
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&file_menu, &other_item,&icon_item])
                .build()?;

            app.set_menu(menu)?;

            Ok(())
        })
}

----------------------------------------

TITLE: Returning Data from Tauri Commands
DESCRIPTION: Shows how to return data from a Tauri command to the JavaScript frontend. Return values must implement serde::Serialize to be properly serialized to JSON.

LANGUAGE: rust
CODE:
#[tauri::command]
fn my_custom_command() -> String {
	"Hello from Rust!".into()
}

----------------------------------------

TITLE: Implementing Async Command in Rust with Result Return Type
DESCRIPTION: Example of an asynchronous Tauri command that uses Result return type to handle borrowed parameters like &str. This pattern works for all types including those that cannot be easily converted to owned types.

LANGUAGE: rust
CODE:
// Return a Result<String, ()> to bypass the borrowing issue
#[tauri::command]
async fn my_custom_command(value: &str) -> Result<String, ()> {
	// Call another async function and wait for it to finish
	some_async_function().await;
	// Note that the return value must be wrapped in `Ok()` now.
	Ok(format!(value))
}

----------------------------------------

TITLE: TypeScript Error Handling for Tauri Commands
DESCRIPTION: Shows how to handle custom error types in TypeScript by defining a type that matches the serialized Rust error structure. This provides better type safety when handling errors from Tauri commands.

LANGUAGE: typescript
CODE:
type ErrorKind = {
  kind: 'io' | 'utf8';
  message: string;
};

invoke('read').catch((e: ErrorKind) => {});

----------------------------------------

TITLE: Optimizing Tauri App Size with Cargo Configuration for Stable Rust Toolchain
DESCRIPTION: This configuration adds optimized Cargo profiles to a Tauri project using the stable Rust toolchain. It includes settings for both development and release builds, focusing on binary size reduction and build optimization.

LANGUAGE: toml
CODE:
# src-tauri/Cargo.toml
[profile.dev]
incremental = true # Compile your binary in smaller steps.

[profile.release]
codegen-units = 1 # Allows LLVM to perform better optimization.
lto = true # Enables link-time-optimizations.
opt-level = "s" # Prioritizes small binary size. Use `3` if you prefer speed.
panic = "abort" # Higher performance by disabling panic handlers.
strip = true # Ensures debug symbols are removed.

----------------------------------------

TITLE: Checking and Installing Updates with Rust in Tauri
DESCRIPTION: This Rust implementation shows how to check for updates using the Tauri updater plugin, download and install updates, and track download progress. It demonstrates setting up an async task to handle the update process and restarting the application after installation.

LANGUAGE: rust
CODE:
use tauri_plugin_updater::UpdaterExt;

pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        update(handle).await.unwrap();
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .unwrap();
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;

    // alternatively we could also call update.download() and update.install() separately
    update
      .download_and_install(
        |chunk_length, content_length| {
          downloaded += chunk_length;
          println!("downloaded {downloaded} from {content_length:?}");
        },
        || {
          println!("download finished");
        },
      )
      .await?;

    println!("update installed");
    app.restart();
  }

  Ok(())
}

----------------------------------------

TITLE: Basic PKGBUILD Template for Tauri Applications
DESCRIPTION: A basic PKGBUILD template for Tauri applications that defines package metadata, dependencies, and source locations. This file is essential for creating AUR packages and includes Tauri-specific dependencies.

LANGUAGE: ini
CODE:
pkgname=<pkgname>
pkgver=1.0.0
pkgrel=1
pkgdesc="Description of your app"
arch=('x86_64' 'aarch64')
url="https://github.com/<user>/<project>"
license=('MIT')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')
options=('!strip' '!emptydirs')
install=${pkgname}.install
source_x86_64=("${url}/releases/download/v${pkgver}/appname_${pkgver}_amd64.deb")
source_aarch64=("${url}/releases/download/v${pkgver}/appname_"${pkgver}_arm64.deb")

----------------------------------------

TITLE: Invoking a Complex Command from JavaScript
DESCRIPTION: Example of invoking a complex Tauri command from JavaScript, passing parameters and handling the response with promises.

LANGUAGE: javascript
CODE:
import { invoke } from '@tauri-apps/api/core';

// Invocation from JavaScript
invoke('my_custom_command', {
  number: 42,
})
  .then((res) =>
    console.log(`Message: ${res.message}, Other Val: ${res.other_val}`)
  )
  .catch((e) => console.error(e));

----------------------------------------

TITLE: Using Autostart Plugin in Rust
DESCRIPTION: Comprehensive example showing how to initialize and use the autostart plugin within a Tauri Rust application, including enabling, checking, and disabling autostart functionality.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    Some(vec!["--flag1", "--flag2"]),
                ));

                // Get the autostart manager
                let autostart_manager = app.autolaunch();
                // Enable autostart
                let _ = autostart_manager.enable();
                // Check enable state
                println!("registered for autostart? {}", autostart_manager.is_enabled().unwrap());
                // Disable autostart
                let _ = autostart_manager.disable();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Using Development and Debug Mode Checks in Rust for Tauri
DESCRIPTION: Examples of how to conditionally execute code based on whether the application is running in development or debug mode in Tauri. Shows multiple approaches including compile-time cfg attributes and runtime checks.

LANGUAGE: rust
CODE:
fn main() {
  // Whether the current instance was started with `tauri dev` or not.
  #[cfg(dev)]
  {
    // `tauri dev` only code
  }
  if cfg!(dev) {
    // `tauri dev` only code
  } else {
    // `tauri build` only code
  }
  let is_dev: bool = tauri::is_dev();

  // Whether debug assertions are enabled or not. This is true for `tauri dev` and `tauri build --debug`.
  #[cfg(debug_assertions)]
  {
    // Debug only code
  }
  if cfg!(debug_assertions) {
    // Debug only code
  } else {
    // Production only code
  }
}

----------------------------------------

TITLE: Handling Command Return Values in JavaScript
DESCRIPTION: Demonstrates how to handle return values from Tauri commands in JavaScript. The invoke function returns a promise that resolves with the returned value from the Rust function.

LANGUAGE: javascript
CODE:
invoke('my_custom_command').then((message) => console.log(message));

----------------------------------------

TITLE: Creating a Default Deny Set for Security Protections
DESCRIPTION: Example of creating a permission set that combines multiple deny permissions to protect sensitive Tauri-related files and folders by default across different platforms.

LANGUAGE: toml
CODE:
[[set]]
identifier = "deny-default"
description = '''
This denies access to dangerous Tauri relevant files and
folders by default.
'''
permissions = ["deny-webview-data-linux", "deny-webview-data-windows"]

----------------------------------------

TITLE: Setting Default Permissions in TOML
DESCRIPTION: This TOML configuration defines the default permission set for a plugin, which specifies permissions that should be enabled by default for the plugin to function properly.

LANGUAGE: toml
CODE:
"$schema" = "schemas/schema.json"
[default]
description = "Allows making HTTP requests"
permissions = ["allow-request"]

----------------------------------------

TITLE: Implementing Async Command in Rust with String Parameters
DESCRIPTION: Example of an asynchronous Tauri command that uses String instead of &str to avoid borrowing issues. This pattern works when you can convert borrowed types to owned types.

LANGUAGE: rust
CODE:
// Declare the async function using String instead of &str, as &str is borrowed and thus unsupported
#[tauri::command]
async fn my_custom_command(value: String) -> String {
	// Call another async function and wait for it to finish
	some_async_function().await;
	value
}

----------------------------------------

TITLE: Creating Basic Application State in Tauri with Rust
DESCRIPTION: Sets up a basic immutable application state using Tauri's Manager API. This example creates an AppData struct with a welcome message and manages it through the app instance during setup.

LANGUAGE: rust
CODE:
use tauri::{Builder, Manager};

struct AppData {
  welcome_message: &'static str,
}

fn main() {
  Builder::default()
    .setup(|app| {
      app.manage(AppData {
        welcome_message: "Welcome to Tauri!",
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .unwrap();
}

----------------------------------------

TITLE: Accessing Managed State in Tauri
DESCRIPTION: Example of how to manage and access application state in Tauri. The state is registered with the Builder and can be accessed in commands using tauri::State.

LANGUAGE: rust
CODE:
struct MyState(String);

#[tauri::command]
fn my_custom_command(state: tauri::State<MyState>) {
	assert_eq!(state.0 == "some state value", true);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.manage(MyState("some state value".into()))
		.invoke_handler(tauri::generate_handler![my_custom_command])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

----------------------------------------

TITLE: Structured Event Payloads in Tauri Events
DESCRIPTION: This code shows how to use structured data as event payloads by defining serializable types. It enhances the download example with more detailed information in each event.

LANGUAGE: rust
CODE:
use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DownloadStarted<'a> {
  url: &'a str,
  download_id: usize,
  content_length: usize,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DownloadProgress {
  download_id: usize,
  chunk_length: usize,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DownloadFinished {
  download_id: usize,
}

#[tauri::command]
fn download(app: AppHandle, url: String) {
  let content_length = 1000;
  let download_id = 1;

  app.emit("download-started", DownloadStarted {
    url: &url,
    download_id,
    content_length
  }).unwrap();

  for chunk_length in [15, 150, 35, 500, 300] {
    app.emit("download-progress", DownloadProgress {
      download_id,
      chunk_length,
    }).unwrap();
  }

  app.emit("download-finished", DownloadFinished { download_id }).unwrap();
}

----------------------------------------

TITLE: Advanced Tauri App Size Optimization with Cargo Configuration for Nightly Rust
DESCRIPTION: This configuration provides an optimized Cargo profile for Tauri projects using the nightly Rust toolchain. It includes additional flags like thread control and path trimming that aren't available in stable Rust.

LANGUAGE: toml
CODE:
# src-tauri/Cargo.toml
[profile.dev]
incremental = true # Compile your binary in smaller steps.
rustflags = ["-Zthreads=8"] # Better compile performance.

[profile.release]
codegen-units = 1 # Allows LLVM to perform better optimization.
lto = true # Enables link-time-optimizations.
opt-level = "s" # Prioritizes small binary size. Use `3` if you prefer speed.
panic = "abort" # Higher performance by disabling panic handlers.
strip = true # Ensures debug symbols are removed.
trim-paths = "all" # Removes potentially privileged information from your binaries.
rustflags = ["-Cdebuginfo=0", "-Zthreads=8"] # Better compile performance.

----------------------------------------

TITLE: Implementing Backend Task Management in Rust
DESCRIPTION: Rust code for the backend that manages setup tasks, tracks completion state, and handles window transitions when all initialization is complete using Tokio for asynchronous operations.

LANGUAGE: rust
CODE:
// /src-tauri/src/lib.rs
// Import functionalities we'll be using
use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Manager, State};
use tokio::time::{sleep, Duration};

// Create a struct we'll use to track the completion of
// setup related tasks
struct SetupState {
    frontend_task: bool,
    backend_task: bool,
}

// Our main entrypoint in a version 2 mobile compatible app
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Don't write code before Tauri starts, write it in the
    // setup hook instead!
    tauri::Builder::default()
        // Register a `State` to be managed by Tauri
        // We need write access to it so we wrap it in a `Mutex`
        .manage(Mutex::new(SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        // Add a command we can use to check
        .invoke_handler(tauri::generate_handler![greet, set_complete])
        // Use the setup hook to execute setup related tasks
        // Runs before the main loop, so no windows are yet created
        .setup(|app| {
            // Spawn setup as a non-blocking task so the windows can be
            // created and ran while it executes
            spawn(setup(app.handle().clone()));
            // The hook expects an Ok result
            Ok(())
        })
        // Run the app
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello {name} from Rust!")
}

// A custom task for setting the state of a setup task
#[tauri::command]
async fn set_complete(
    app: AppHandle,
    state: State<'_, Mutex<SetupState>>,
    task: String,
) -> Result<(), ()> {
    // Lock the state without write access
    let mut state_lock = state.lock().unwrap();
    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        _ => panic!("invalid task completed!"),
    }
    // Check if both tasks are completed
    if state_lock.backend_task && state_lock.frontend_task {
        // Setup is complete, we can close the splashscreen
        // and unhide the main window!
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
    Ok(())
}

// An async function that does some heavy setup task
async fn setup(app: AppHandle) -> Result<(), ()> {
    // Fake performing some heavy action for 3 seconds
    println!("Performing really heavy backend setup task...");
    sleep(Duration::from_secs(3)).await;
    println!("Backend setup task completed!");
    // Set the backend task as being completed
    // Commands can be ran as regular functions as long as you take
    // care of the input arguments yourself
    set_complete(
        app.clone(),
        app.state::<Mutex<SetupState>>(),
        "backend".to_string(),
    )
    .await?;
    Ok(())
}

----------------------------------------

TITLE: Building Non-Blocking Ask Dialog in Rust
DESCRIPTION: Rust implementation of a non-blocking dialog with custom button labels and callback for handling the result.

LANGUAGE: rust
CODE:
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

app.dialog()
    .message("Tauri is Awesome")
    .title("Tauri is Awesome")
   .buttons(MessageDialogButtons::OkCancelCustom("Absolutely", "Totally"))
    .show(|result| match result {
        true => // do something,
        false =>// do something,
    });

----------------------------------------

TITLE: Invoking Async Commands from JavaScript
DESCRIPTION: Example of how to invoke an asynchronous Tauri command from JavaScript. The invoke function returns a Promise that resolves when the command completes.

LANGUAGE: javascript
CODE:
invoke('my_custom_command', { value: 'Hello, Async!' }).then(() =>
  console.log('Completed!')
);

----------------------------------------

TITLE: Creating a Read-Only Permission Set for Application Data
DESCRIPTION: Example of a complete permission set that allows read-only access to files in the application local data folder while maintaining security protections for sensitive data.

LANGUAGE: toml
CODE:
[[set]]
identifier = "read-files-applocaldata"
description = '''
This set allows file read access to the `APPLOCALDATA` folder and
subfolders except for linux,
while it denies access to dangerous Tauri relevant files and
folders by default on windows.'''
permissions = ["scope-applocaldata-reasonable", "allow-read-file"]

----------------------------------------

TITLE: Using the File System Plugin in Rust
DESCRIPTION: Rust code demonstrating how to use the Tauri file system plugin to configure directory permissions by allowing access to specific directories.

LANGUAGE: rust
CODE:
use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
      .plugin(tauri_plugin_fs::init())
      .setup(|app| {
          // allowed the given directory
          let scope = app.fs_scope();
        	scope.allow_directory("/path/to/directory", false);
          dbg!(scope.allowed());

          Ok(())
       })
       .run(tauri::generate_context!())
       .expect("error while running tauri application");
}

----------------------------------------

TITLE: Accessing Raw Request in Tauri Commands
DESCRIPTION: Example of accessing the full tauri::ipc::Request object in a command, which includes the raw body payload and request headers. This allows for more complex command implementations.

LANGUAGE: rust
CODE:
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error("unexpected request body")]
  RequestBodyMustBeRaw,
  #[error("missing `{0}` header")]
  MissingHeader(&'static str),
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

#[tauri::command]
fn upload(request: tauri::ipc::Request) -> Result<(), Error> {
  let tauri::ipc::InvokeBody::Raw(upload_data) = request.body() else {
    return Err(Error::RequestBodyMustBeRaw);
  };
  let Some(authorization_header) = request.headers().get("Authorization") else {
    return Err(Error::MissingHeader("Authorization"));
  };

  // upload...

  Ok(())
}

----------------------------------------

TITLE: Vite Configuration for Tauri Integration
DESCRIPTION: Comprehensive vite.config.js configuration for Tauri projects, handling development server settings, environment variables, and platform-specific build targets. Includes special handling for iOS physical devices via TAURI_DEV_HOST.

LANGUAGE: js
CODE:
import { defineConfig } from 'vite';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  // prevent vite from obscuring rust errors
  clearScreen: false,
  server: {
    port: 1420,
    // Tauri expects a fixed port, fail if that port is not available
    strictPort: true,
    // if the host Tauri is expecting is set, use it
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,

    watch: {
      // tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
  // Env variables starting with the item of `envPrefix` will be exposed in tauri's source code through `import.meta.env`.
  envPrefix: ['VITE_', 'TAURI_ENV_*'],
  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target:
      process.env.TAURI_ENV_PLATFORM == 'windows'
        ? 'chrome105'
        : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
});

----------------------------------------

TITLE: Implementing Setup Lifecycle Hook in Tauri Plugin
DESCRIPTION: Shows how to implement the setup lifecycle hook in a Tauri plugin to initialize state and start background tasks when the plugin is initialized.

LANGUAGE: rust
CODE:
use tauri::{Manager, plugin::Builder};
use std::{collections::HashMap, sync::Mutex, time::Duration};

struct DummyStore(Mutex<HashMap<String, String>>);

Builder::new("<plugin-name>")
  .setup(|app, api| {
    app.manage(DummyStore(Default::default()));

    let app_ = app.clone();
    std::thread::spawn(move || {
      loop {
        app_.emit("tick", ());
        std::thread::sleep(Duration::from_secs(1));
      }
    });

    Ok(())
  })

----------------------------------------

TITLE: Implementing an Asynchronous Command in Android with Coroutines
DESCRIPTION: Demonstrates how to implement an asynchronous command in an Android Tauri plugin using Kotlin coroutines with a custom scope for background operations.

LANGUAGE: kotlin
CODE:
import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin

// Change to Dispatchers.IO if it is intended for fetching data
val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
  @Command
  fun openCamera(invoke: Invoke) {
    scope.launch {
      openCameraInner(invoke)
    }
  }

  private suspend fun openCameraInner(invoke: Invoke) {
    val ret = JSObject()
    ret.put("path", "/path/to/photo.jpg")
    invoke.resolve(ret)
  }
}

----------------------------------------

TITLE: Registering Commands from Separate Module in Tauri
DESCRIPTION: Demonstrates how to register commands from a separate module in the main application builder. The module must be defined and the command referenced with its full path including the module name.

LANGUAGE: rust
CODE:
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![commands::my_custom_command])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

----------------------------------------

TITLE: Writing to NFC Tags in Rust
DESCRIPTION: Rust code to write NFC records to tags using the write method. This demonstrates the low-level interface for creating and writing NFC records in Rust.

LANGUAGE: rust
CODE:
tauri::Builder::default()
  .setup(|app| {
    #[cfg(mobile)]
    {
      use tauri_plugin_nfc::NfcExt;

      app.handle().plugin(tauri_plugin_nfc::init());

      app
        .nfc()
        .write(vec![
          tauri_plugin_nfc::NfcRecord {
            format: tauri_plugin_nfc::NFCTypeNameFormat::NfcWellKnown,
            kind: vec![0x55], // URI record
            id: vec![],
            payload: vec![], // insert payload here
          }
        ])?;
    }
    Ok(())
  })

----------------------------------------

TITLE: Defining a Command with AppHandle and Window Access in Rust
DESCRIPTION: This snippet demonstrates how to create a Tauri command that accesses the AppHandle and Window instances through dependency injection, and handles input parameters including a Channel for progress updates.

LANGUAGE: rust
CODE:
use tauri::{command, ipc::Channel, AppHandle, Runtime, Window};

#[command]
async fn upload<R: Runtime>(app: AppHandle<R>, window: Window<R>, on_progress: Channel, url: String) {
  // implement command logic here
  on_progress.send(100).unwrap();
}

----------------------------------------

TITLE: Getting File Metadata with Tauri FS Plugin
DESCRIPTION: Retrieves metadata about a file using the stat function. This provides information such as file size, creation time, and modification time. The stat function follows symlinks.

LANGUAGE: javascript
CODE:
import { stat, BaseDirectory } from '@tauri-apps/plugin-fs';
const metadata = await stat('app.db', {
  baseDir: BaseDirectory.AppLocalData,
});

----------------------------------------

TITLE: Mocking Basic IPC Requests in Tauri with Vitest
DESCRIPTION: Demonstrates how to intercept a simple IPC request using mockIPC function from @tauri-apps/api/mocks. The example creates a mock for a Rust command 'add' that adds two numbers and includes setup for WebCrypto in jsdom.

LANGUAGE: javascript
CODE:
import { beforeAll, expect, test } from "vitest";
import { randomFillSync } from "crypto";

import { mockIPC } from "@tauri-apps/api/mocks";
import { invoke } from "@tauri-apps/api/core";

// jsdom doesn't come with a WebCrypto implementation
beforeAll(() => {
  Object.defineProperty(window, 'crypto', {
    value: {
      // @ts-ignore
      getRandomValues: (buffer) => {
        return randomFillSync(buffer);
      },
    },
  });
});


test("invoke simple", async () => {
  mockIPC((cmd, args) => {
    // simulated rust command called "add" that just adds two numbers
    if(cmd === "add") {
      return (args.a as number) + (args.b as number);
    }
  });
});

----------------------------------------

TITLE: Configuring Trunk for Leptos Development
DESCRIPTION: TOML configuration for Trunk.toml that sets up the development environment for Leptos. It configures the build target, watch settings, and server configuration including the WebSocket protocol for mobile development.

LANGUAGE: toml
CODE:
// Trunk.toml
[build]
target = "./index.html"

[watch]
ignore = ["./src-tauri"]

[serve]
port = 1420
open = false
ws_protocol = "ws"


----------------------------------------

TITLE: Referencing Capabilities in Tauri Configuration
DESCRIPTION: This JSON snippet shows how to reference predefined capabilities in the tauri.conf.json file. This is a common configuration method where only the capability identifiers are referenced, requiring the capability files to be defined in the capabilities directory.

LANGUAGE: json
CODE:
{
  "app": {
    "security": {
      "capabilities": ["my-capability", "main-capability"]
    }
  }
}

----------------------------------------

TITLE: Platform-Specific Mobile Capability Configuration
DESCRIPTION: This JSON snippet defines a capability specifically for mobile platforms (iOS, Android). It enables permissions on plugins that are only available on mobile, such as NFC scanning, biometric authentication, and barcode scanning.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/mobile-schema.json",
  "identifier": "mobile-capability",
  "windows": ["main"],
  "platforms": ["iOS", "android"],
  "permissions": [
    "nfc:allow-scan",
    "biometric:allow-authenticate",
    "barcode-scanner:allow-scan"
  ]
}

----------------------------------------

TITLE: Configuring Trunk for Leptos Development
DESCRIPTION: TOML configuration for Trunk.toml that sets up the development environment for Leptos. It configures the build target, watch settings, and server configuration including the WebSocket protocol for mobile development.

LANGUAGE: toml
CODE:
// Trunk.toml
[build]
target = "./index.html"

[watch]
ignore = ["./src-tauri"]

[serve]
port = 1420
open = false
ws_protocol = "ws"


----------------------------------------

TITLE: Defining Tauri Commands in Separate Module
DESCRIPTION: Shows how to organize commands in a separate module to avoid bloating the lib.rs file. Commands defined in separate modules should be marked as pub and referenced with their module path in the handler registration.

LANGUAGE: rust
CODE:
#[tauri::command]
pub fn my_custom_command() {
	println!("I was invoked from JavaScript!");
}

----------------------------------------

TITLE: Configuring Command-Specific FS Permissions in Tauri
DESCRIPTION: Example showing how to configure file system permissions for specific commands in a Tauri application. It demonstrates allowing rename operations for home directory files while denying them for .config files, and allowing exists checks for app data files.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "fs:allow-rename",
      "allow": [{ "path": "$HOME/**" }]
    },
    {
      "identifier": "fs:allow-rename",
      "deny": [{ "path": "$HOME/.config/**" }]
    },
    {
      "identifier": "fs:allow-exists",
      "allow": [{ "path": "$APPDATA/*" }]
    }
  ]
}

----------------------------------------

TITLE: Modifying Mutable State with Mutex in Tauri
DESCRIPTION: Shows how to access and modify state that's wrapped in a Mutex by locking it first. The state can be modified while locked and will be automatically unlocked when the guard is dropped.

LANGUAGE: rust
CODE:
let state = app.state::<Mutex<AppState>>();

// Lock the mutex to get mutable access:
let mut state = state.lock().unwrap();

// Modify the state:
state.counter += 1;

----------------------------------------

TITLE: Implementing Selenium WebDriver Tests for Tauri Applications
DESCRIPTION: A complete test script using Selenium WebDriver with Mocha and Chai to test a Tauri application. It includes setup code for tauri-driver, WebDriver session management, and tests for checking UI elements and styling.

LANGUAGE: javascript
CODE:
const os = require('os');
const path = require('path');
const { expect } = require('chai');
const { spawn, spawnSync } = require('child_process');
const { Builder, By, Capabilities } = require('selenium-webdriver');

// create the path to the expected application binary
const application = path.resolve(
  __dirname,
  '..',
  '..',
  '..',
  'target',
  'release',
  'hello-tauri-webdriver'
);

// keep track of the webdriver instance we create
let driver;

// keep track of the tauri-driver process we start
let tauriDriver;

before(async function () {
  // set timeout to 2 minutes to allow the program to build if it needs to
  this.timeout(120000);

  // ensure the program has been built
  spawnSync('cargo', ['build', '--release']);

  // start tauri-driver
  tauriDriver = spawn(
    path.resolve(os.homedir(), '.cargo', 'bin', 'tauri-driver'),
    [],
    { stdio: [null, process.stdout, process.stderr] }
  );

  const capabilities = new Capabilities();
  capabilities.set('tauri:options', { application });
  capabilities.setBrowserName('wry');

  // start the webdriver client
  driver = await new Builder()
    .withCapabilities(capabilities)
    .usingServer('http://127.0.0.1:4444/')
    .build();
});

after(async function () {
  // stop the webdriver session
  await driver.quit();

  // kill the tauri-driver process
  tauriDriver.kill();
});

describe('Hello Tauri', () => {
  it('should be cordial', async () => {
    const text = await driver.findElement(By.css('body > h1')).getText();
    expect(text).to.match(/^[hH]ello/);
  });

  it('should be excited', async () => {
    const text = await driver.findElement(By.css('body > h1')).getText();
    expect(text).to.match(/!$/);
  });

  it('should be easy on the eyes', async () => {
    // selenium returns color css values as rgb(r, g, b)
    const text = await driver
      .findElement(By.css('body'))
      .getCssValue('background-color');

    const rgb = text.match(/^rgb\((?<r>\d+), (?<g>\d+), (?<b>\d+)\)$/).groups;
    expect(rgb).to.have.all.keys('r', 'g', 'b');

    const luma = 0.2126 * rgb.r + 0.7152 * rgb.g + 0.0722 * rgb.b;
    expect(luma).to.be.lessThan(100);
  });
});

----------------------------------------

TITLE: Opening Files in Read-Only Mode with Tauri FS Plugin
DESCRIPTION: Opens a file in read-only mode, reads its contents into a buffer, and then closes the file. This example demonstrates how to read binary data from a file and convert it to text.

LANGUAGE: javascript
CODE:
import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
const file = await open('foo/bar.txt', {
  read: true,
  baseDir: BaseDirectory.AppData,
});

const stat = await file.stat();
const buf = new Uint8Array(stat.size);
await file.read(buf);
const textContents = new TextDecoder().decode(buf);
await file.close();

----------------------------------------

TITLE: Executing SQLite/PostgreSQL Queries with Parameters
DESCRIPTION: JavaScript code showing how to execute parameterized SQLite or PostgreSQL queries using the positional $1, $2, $3 syntax.

LANGUAGE: javascript
CODE:
const result = await db.execute(
  "INSERT into todos (id, title, status) VALUES ($1, $2, $3)",
  [todos.id, todos.title, todos.status],
);

const result = await db.execute(
"UPDATE todos SET title = $1, status = $2 WHERE id = $3",
[todos.title, todos.status, todos.id],
);

----------------------------------------

TITLE: Filtering Events to Multiple Webviews in Tauri
DESCRIPTION: This code demonstrates how to emit events to a filtered set of webviews using emit_filter. It sends an open-file event only to webviews with specific labels.

LANGUAGE: rust
CODE:
use tauri::{AppHandle, Emitter, EventTarget};

#[tauri::command]
fn open_file(app: AppHandle, path: std::path::PathBuf) {
  app.emit_filter("open-file", path, |target| match target {
    EventTarget::WebviewWindow { label } => label == "main" || label == "file-viewer",
    _ => false,
  }).unwrap();
}

----------------------------------------

TITLE: Returning Array Buffers from Tauri Commands
DESCRIPTION: Shows how to optimize returning large binary data like files by using tauri::ipc::Response instead of JSON serialization. This approach is more efficient for transferring binary data between Rust and JavaScript.

LANGUAGE: rust
CODE:
use tauri::ipc::Response;
#[tauri::command]
fn read_file() -> Response {
	let data = std::fs::read("/path/to/file").unwrap();
	tauri::ipc::Response::new(data)
}

----------------------------------------

TITLE: Implementing Navigation Lifecycle Hook in Tauri Plugin
DESCRIPTION: Demonstrates how to implement the on_navigation lifecycle hook to validate or track URL changes in a web view, with the ability to cancel navigation.

LANGUAGE: rust
CODE:
use tauri::plugin::Builder;

Builder::new("<plugin-name>")
  .on_navigation(|window, url| {
    println!("window {} is navigating to {}", window.label(), url);
    // Cancels the navigation if forbidden
    url.scheme() != "forbidden"
  })

----------------------------------------

TITLE: Creating and Writing to a File in JavaScript
DESCRIPTION: JavaScript code showing how to create a file and write content to it using the Tauri file system plugin, including proper file closing.

LANGUAGE: javascript
CODE:
import { create, BaseDirectory } from '@tauri-apps/plugin-fs';
const file = await create('foo/bar.txt', { baseDir: BaseDirectory.AppData });
await file.write(new TextEncoder().encode('Hello world'));
await file.close();

----------------------------------------

TITLE: Accessing State in Tauri Commands
DESCRIPTION: Shows how to access managed state within a Tauri command function. The state is injected as a parameter and can be used within the command implementation.

LANGUAGE: rust
CODE:
#[tauri::command]
fn increase_counter(state: State<'_, Mutex<AppState>>) -> u32 {
  let mut state = state.lock().unwrap();
  state.counter += 1;
  state.counter
}

----------------------------------------

TITLE: Configuring Dynamic Update Endpoints in Rust for Tauri
DESCRIPTION: This snippet shows how to set dynamic update endpoints at runtime in Rust, enabling features like release channels. It demonstrates constructing the URL with proper variable formatting based on channel selection.

LANGUAGE: rust
CODE:
use tauri_plugin_updater::UpdaterExt;
let channel = if beta { "beta" } else { "stable" };
let update_url = format!("https://{channel}.myserver.com/{{{{target}}}}-{{{{arch}}}}/{{{{current_version}}}}{{{{current_version}}}}");

let update = app
  .updater_builder()
  .endpoints(vec![update_url])?
  .build()?
  .check()
  .await?;

----------------------------------------

TITLE: Tauri Configuration in JSON5 Format
DESCRIPTION: Example of a Tauri configuration file using JSON5 format to define build settings, bundle options, window properties, and plugin configurations.

LANGUAGE: json5
CODE:
{
  build: {
    devUrl: 'http://localhost:3000',
    // start the dev server
    beforeDevCommand: 'npm run dev',
  },
  bundle: {
    active: true,
    icon: ['icons/app.png'],
  },
  app: {
    windows: [
      {
        title: 'MyApp',
      },
    ],
  },
  plugins: {
    updater: {
      pubkey: 'updater pub key',
      endpoints: ['https://my.app.updater/{{target}}/{{current_version}}'],
    },
  },
}

----------------------------------------

TITLE: Defining Allow Scope for Application Local Data in Fs Plugin
DESCRIPTION: Example showing how to define a permission with an allow scope that grants recursive access to the application local data folder. The scope uses glob patterns to define path access.

LANGUAGE: toml
CODE:
[[permission]]
identifier = "scope-applocaldata-recursive"
description = '''
This scope recursive access to the complete `$APPLOCALDATA` folder,
including sub directories and files.
'''

[[permission.scope.allow]]
path = "$APPLOCALDATA/**"

----------------------------------------

TITLE: Using Clipboard Plugin in Rust
DESCRIPTION: Example demonstrating how to use the clipboard plugin in Rust to write text to the clipboard and read text from it. It shows importing the ClipboardExt trait and using the clipboard methods.

LANGUAGE: rust
CODE:
use tauri_plugin_clipboard_manager::ClipboardExt;

app.clipboard().write_text("Tauri is awesome!".to_string()).unwrap();

// Read content from clipboard
let content = app.clipboard().read_text();
println!("{:?}", content.unwrap());
// Prints "Tauri is awesome!" to the terminal


----------------------------------------

TITLE: Configuring WebSocket Plugin Permissions in JSON
DESCRIPTION: JSON configuration for enabling WebSocket plugin permissions in a Tauri application's capabilities file.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["websocket:default"]
}

----------------------------------------

TITLE: Using Channels for Efficient Data Streaming in Tauri
DESCRIPTION: This code demonstrates using Tauri channels for efficient data streaming instead of the event system. It shows how to define an enum for different event types and send them through a channel.

LANGUAGE: rust
CODE:
use tauri::{AppHandle, ipc::Channel};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
enum DownloadEvent<'a> {
  #[serde(rename_all = "camelCase")]
  Started {
    url: &'a str,
    download_id: usize,
    content_length: usize,
  },
  #[serde(rename_all = "camelCase")]
  Progress {
    download_id: usize,
    chunk_length: usize,
  },
  #[serde(rename_all = "camelCase")]
  Finished {
    download_id: usize,
  },
}

#[tauri::command]
fn download(app: AppHandle, url: String, on_event: Channel<DownloadEvent>) {
  let content_length = 1000;
  let download_id = 1;

  on_event.send(DownloadEvent::Started {
    url: &url,
    download_id,
    content_length,
  }).unwrap();

  for chunk_length in [15, 150, 35, 500, 300] {
    on_event.send(DownloadEvent::Progress {
      download_id,
      chunk_length,
    }).unwrap();
  }

  on_event.send(DownloadEvent::Finished { download_id }).unwrap();
}

----------------------------------------

TITLE: Initializing a Tauri Project Using create-tauri-app Command Prompts
DESCRIPTION: The command prompt flow for initializing a new Tauri project with create-tauri-app, showing the project name and identifier inputs required to start the project creation process.

LANGUAGE: bash
CODE:
? Project name (tauri-app) ›
? Identifier (com.tauri-app.app) ›

----------------------------------------

TITLE: Advanced IPC Request Mocking with Spying in Tauri
DESCRIPTION: Shows how to combine mockIPC with Vitest's spying tools to track IPC call information such as invocation count. The example creates a mock for a Rust command 'add' and verifies it was called correctly.

LANGUAGE: javascript
CODE:
import { beforeAll, expect, test, vi } from "vitest";
import { randomFillSync } from "crypto";

import { mockIPC } from "@tauri-apps/api/mocks";
import { invoke } from "@tauri-apps/api/core";

// jsdom doesn't come with a WebCrypto implementation
beforeAll(() => {
  Object.defineProperty(window, 'crypto', {
    value: {
      // @ts-ignore
      getRandomValues: (buffer) => {
        return randomFillSync(buffer);
      },
    },
  });
});


test("invoke", async () => {
  mockIPC((cmd, args) => {
    // simulated rust command called "add" that just adds two numbers
    if(cmd === "add") {
      return (args.a as number) + (args.b as number);
    }
  });

  // we can use the spying tools provided by vitest to track the mocked function
  const spy = vi.spyOn(window.__TAURI_INTERNALS__, "invoke");

  expect(invoke("add", { a: 12, b: 15 })).resolves.toBe(27);
  expect(spy).toHaveBeenCalled();
});

----------------------------------------

TITLE: Base CLI Configuration in Tauri
DESCRIPTION: JSON configuration for the CLI plugin in tauri.conf.json. It defines the CLI description, arguments, and subcommands. The example includes a verbose flag and a 'run' subcommand with debug and release options.

LANGUAGE: json
CODE:
{
  "plugins": {
    "cli": {
      "description": "Tauri CLI Plugin Example",
      "args": [
        {
          "short": "v",
          "name": "verbose",
          "description": "Verbosity level"
        }
      ],
      "subcommands": {
        "run": {
          "description": "Run the application",
          "args": [
            {
              "name": "debug",
              "description": "Run application in debug mode"
            },
            {
              "name": "release",
              "description": "Run application in release mode"
            }
          ]
        }
      }
    }
  }
}

----------------------------------------

TITLE: Using WebSocket Plugin in JavaScript
DESCRIPTION: Example showing how to connect to a WebSocket server, add a message listener, send a message, and disconnect using the Tauri WebSocket plugin.

LANGUAGE: javascript
CODE:
import WebSocket from '@tauri-apps/plugin-websocket';
// when using `"withGlobalTauri": true`, you may use
// const WebSocket = window.__TAURI__.websocket;

const ws = await WebSocket.connect('ws://127.0.0.1:8080');

ws.addListener((msg) => {
  console.log('Received Message:', msg);
});

await ws.send('Hello World!');

await ws.disconnect();

----------------------------------------

TITLE: Configuring launch.json for Tauri Debugging with LLDB Extension
DESCRIPTION: This configuration sets up VS Code to debug Tauri applications using the LLDB extension. It includes separate configurations for development and production modes, with appropriate cargo commands for building the Rust application.

LANGUAGE: json
CODE:
{
  // Use IntelliSense to learn about possible attributes.
  // Hover to view descriptions of existing attributes.
  // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Tauri Development Debug",
      "cargo": {
        "args": [
          "build",
          "--manifest-path=./src-tauri/Cargo.toml",
          "--no-default-features"
        ]
      },
      // task for the `beforeDevCommand` if used, must be configured in `.vscode/tasks.json`
      "preLaunchTask": "ui:dev"
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Tauri Production Debug",
      "cargo": {
        "args": ["build", "--release", "--manifest-path=./src-tauri/Cargo.toml"]
      },
      // task for the `beforeBuildCommand` if used, must be configured in `.vscode/tasks.json`
      "preLaunchTask": "ui:build"
    }
  ]
}

----------------------------------------

TITLE: Connecting to SQLite Database in JavaScript
DESCRIPTION: JavaScript code that loads and interacts with an SQLite database using the Tauri SQL plugin.

LANGUAGE: javascript
CODE:
import Database from '@tauri-apps/plugin-sql';
// when using `"withGlobalTauri": true`, you may use
// const Database = window.__TAURI__.sql;

const db = await Database.load('sqlite:test.db');
await db.execute('INSERT INTO ...');

----------------------------------------

TITLE: Invoking a Command with Raw Request Body and Headers in JavaScript
DESCRIPTION: Example of invoking a Tauri command from JavaScript with a raw request body (ArrayBuffer/Uint8Array) and custom headers.

LANGUAGE: javascript
CODE:
const data = new Uint8Array([1, 2, 3]);
await __TAURI__.core.invoke('upload', data, {
  headers: {
    Authorization: 'apikey',
  },
});

----------------------------------------

TITLE: Implementing a File Writing Command in Rust
DESCRIPTION: Rust implementation of a new command 'write_custom_file' that writes user input to a file in the temporary directory. The command is decorated with Tauri's command macro to expose it as an API endpoint.

LANGUAGE: rust
CODE:
use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::TestExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.test1().ping(payload)
}

#[command]
pub(crate) async fn write_custom_file<R: Runtime>(
    user_input: String,
    app: AppHandle<R>,
) -> Result<String> {
    std::fs::write(app.path().temp_dir().unwrap(), user_input)?;
    Ok("success".to_string())
}

----------------------------------------

TITLE: Creating System Trays at Runtime in Tauri
DESCRIPTION: Example of using the runtime system tray API in Tauri 1.1.0. The code shows how to create a system tray with a custom menu and handle events such as menu item clicks.

LANGUAGE: rust
CODE:
use tauri::{Builder, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
Builder::default()
    .setup(|app| {
        let handle = app.handle();
        SystemTray::new()
            .with_id("main")
            .with_menu(
                SystemTrayMenu::new().add_item(CustomMenuItem::new("quit", "Quit"))
            )
            .on_event(move |event| {
                let tray_handle = handle.tray_handle_by_id("main").unwrap();
                if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                    if id == "quit" {
                        tray_handle.destroy().unwrap();
                    }
                }
            })
            .build(&handle)
            .expect("unable to create tray");
    });

----------------------------------------

TITLE: Creating and Using Channels from TypeScript in Tauri
DESCRIPTION: This TypeScript code demonstrates how to create a channel on the frontend side and provide it to a Rust command. It defines the event types matching the Rust side and handles received messages.

LANGUAGE: typescript
CODE:
import { invoke, Channel } from '@tauri-apps/api/core';

type DownloadEvent =
  | {
      event: 'started';
      data: {
        url: string;
        downloadId: number;
        contentLength: number;
      };
    }
  | {
      event: 'progress';
      data: {
        downloadId: number;
        chunkLength: number;
      };
    }
  | {
      event: 'finished';
      data: {
        downloadId: number;
      };
    };

const onEvent = new Channel<DownloadEvent>();
onEvent.onmessage = (message) => {
  console.log(`got download event ${message.event}`);
};

await invoke('download', {
  url: 'https://raw.githubusercontent.com/tauri-apps/tauri/dev/crates/tauri-schema-generator/schemas/config.schema.json',
  onEvent,
});

----------------------------------------

TITLE: Exposing Commands in Tauri Plugin Builder
DESCRIPTION: This code shows how to expose a command to the webview by hooking into the invoke_handler call in lib.rs, which must be done to make the command available to the frontend.

LANGUAGE: rust
CODE:
Builder::new("<plugin-name>")
    .invoke_handler(tauri::generate_handler![commands::upload])

----------------------------------------

TITLE: Saving Files with Non-Blocking Dialog in Rust
DESCRIPTION: Rust example demonstrating how to create a non-blocking file save dialog with custom file extension filters.

LANGUAGE: rust
CODE:
use tauri_plugin_dialog::DialogExt;

app.dialog()
    .file()
    .add_filter("My Filter", &["png", "jpeg"])
    .pick_file(|file_path| {
        // return a file_path `Option`, or `None` if the user closes the dialog
    });

----------------------------------------

TITLE: Configuring Positional Arguments in Tauri CLI
DESCRIPTION: JSON configuration for positional arguments in tauri.conf.json. It defines two arguments (source and destination) with specific indices that are identified by their position in the command.

LANGUAGE: json
CODE:
{
  "args": [
    {
      "name": "source",
      "index": 1,
      "takesValue": true
    },
    {
      "name": "destination",
      "index": 2,
      "takesValue": true
    }
  ]
}

----------------------------------------

TITLE: Configuring SQL Plugin Preload in Tauri Configuration
DESCRIPTION: JSON configuration that specifies database connections to be preloaded when the Tauri SQL plugin is initialized.

LANGUAGE: json
CODE:
{
  "plugins": {
    "sql": {
      "preload": ["sqlite:mydatabase.db"]
    }
  }
}

----------------------------------------

TITLE: Implementing Global Shortcuts in Rust with Event States
DESCRIPTION: Comprehensive example of registering and handling global shortcuts in Rust, including different shortcut states (pressed/released). Creates a Ctrl+N shortcut with separate handling for press and release events.

LANGUAGE: rust
CODE:
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
                        println!("{:?}", shortcut);
                        if shortcut == &ctrl_n_shortcut {
                            match event.state() {
                              ShortcutState::Pressed => {
                                println!("Ctrl-N Pressed!");
                              }
                              ShortcutState::Released => {
                                println!("Ctrl-N Released!");
                              }
                            }
                        }
                    })
                    .build(),
                )?;

                app.global_shortcut().register(ctrl_n_shortcut)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Mixing Inline and Predefined Capabilities in Configuration
DESCRIPTION: This JSON snippet demonstrates how to mix inline-defined capabilities with pre-defined ones in the tauri.conf.json file. The inline capability includes a description, window targets, and permissions.

LANGUAGE: json
CODE:
{
  "app": {
    "security": {
      "capabilities": [
        {
          "identifier": "my-capability",
          "description": "My application capability used for all windows",
          "windows": ["*"],
          "permissions": ["fs:default", "allow-home-read-extended"]
        },
        "my-second-capability"
      ]
    }
  }
}

----------------------------------------

TITLE: Streaming Text File Lines with Tauri FS Plugin
DESCRIPTION: Reads a text file line by line using readTextFileLines, which is efficient for large files. This example shows how to process each line of a file in a memory-efficient way using async iteration.

LANGUAGE: typescript
CODE:
import { readTextFileLines, BaseDirectory } from '@tauri-apps/plugin-fs';
const lines = await readTextFileLines('app.logs', {
  baseDir: BaseDirectory.AppLog,
});
for await (const line of lines) {
  console.log(line);
}

----------------------------------------

TITLE: Accessing Resource Files in Rust with a Command Handler
DESCRIPTION: Rust code example showing how to access bundled resource files in a Tauri command handler. This demonstrates using the AppHandle to resolve resource paths and returning data to the frontend.

LANGUAGE: rust
CODE:
#[tauri::command]
fn hello(handle: tauri::AppHandle) -> String {
    let resource_path = handle.path().resolve("lang/de.json", BaseDirectory::Resource)?;

    let file = std::fs::File::open(&resource_path).unwrap();
    let lang_de: serde_json::Value = serde_json::from_reader(file).unwrap();

    lang_de.get("hello").unwrap()
}

----------------------------------------

TITLE: Using Barcode Scanner Plugin in JavaScript
DESCRIPTION: JavaScript code demonstrating how to import and use the barcode scanner plugin to scan QR codes with a transparent camera view in the application.

LANGUAGE: javascript
CODE:
import { scan, Format } from '@tauri-apps/plugin-barcode-scanner';
// when using `"withGlobalTauri": true`, you may use
// const { scan, Format } = window.__TAURI__.barcodeScanner;

// `windowed: true` actually sets the webview to transparent
// instead of opening a separate view for the camera
// make sure your user interface is ready to show what is underneath with a transparent element
scan({ windowed: true, formats: [Format.QRCode] });

----------------------------------------

TITLE: Writing WebdriverIO Test Specs for Tauri Application
DESCRIPTION: Example test spec file that tests the Tauri application UI. It checks for specific text content and CSS properties using WebdriverIO's selector and assertion APIs.

LANGUAGE: javascript
CODE:
// calculates the luma from a hex color `#abcdef`
function luma(hex) {
  if (hex.startsWith('#')) {
    hex = hex.substring(1);
  }

  const rgb = parseInt(hex, 16);
  const r = (rgb >> 16) & 0xff;
  const g = (rgb >> 8) & 0xff;
  const b = (rgb >> 0) & 0xff;
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

describe('Hello Tauri', () => {
  it('should be cordial', async () => {
    const header = await $('body > h1');
    const text = await header.getText();
    expect(text).toMatch(/^[hH]ello/);
  });

  it('should be excited', async () => {
    const header = await $('body > h1');
    const text = await header.getText();
    expect(text).toMatch(/!$/);
  });

  it('should be easy on the eyes', async () => {
    const body = await $('body');
    const backgroundColor = await body.getCSSProperty('background-color');
    expect(luma(backgroundColor.parsed.hex)).toBeLessThan(100);
  });
});

----------------------------------------

TITLE: Tauri Configuration in TOML Format
DESCRIPTION: Example of a Tauri configuration file using TOML format to define the same settings as the JSON5 example, demonstrating TOML's kebab-case naming convention.

LANGUAGE: toml
CODE:
[build]
dev-url = "http://localhost:3000"
# start the dev server
before-dev-command = "npm run dev"

[bundle]
active = true
icon = ["icons/app.png"]

[[app.windows]]
title = "MyApp"

[plugins.updater]
pubkey = "updater pub key"
endpoints = ["https://my.app.updater/{{target}}/{{current_version}}"]

----------------------------------------

TITLE: Unlistening from Events in JavaScript with Tauri
DESCRIPTION: Demonstrates how to stop listening to an event in a Tauri application. The listen function returns an unlisten function that can be called to unregister the event listener when it's no longer needed.

LANGUAGE: javascript
CODE:
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('download-started', (event) => {});
unlisten();

----------------------------------------

TITLE: Configuring Nuxt for Tauri Integration
DESCRIPTION: TypeScript configuration for nuxt.config.ts that enables SSG mode, configures the development server for iOS physical device testing, and sets up Vite with Tauri-specific requirements including environment variables and strict port usage.

LANGUAGE: typescript
CODE:
export default defineNuxtConfig({
  // （可选） 启用 Nuxt 调试工具
  devtools: { enabled: true },
  // 启用 SSG
  ssr: false,
  // 使开发服务器能够被其他设备发现，以便在 iOS 物理机运行。
  devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },
  vite: {
    // 为 Tauri 命令输出提供更好的支持
    clearScreen: false,
    // 启用环境变量
    // 其他环境变量可以在如下网页中获知：
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      // Tauri需要一个确定的端口
      strictPort: true,
    },
  },
});

----------------------------------------

TITLE: Installing the HTTP Plugin via Command Line
DESCRIPTION: Command to add the tauri-plugin-http dependency to your project using Cargo in the src-tauri folder.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-http

----------------------------------------

TITLE: Invoking Tauri Command with Arguments from JavaScript
DESCRIPTION: Demonstrates how to pass arguments from JavaScript to a Rust command. Arguments are passed as a JSON object with camelCase keys by default.

LANGUAGE: javascript
CODE:
invoke('my_custom_command', { invokeMessage: 'Hello!' });

----------------------------------------

TITLE: Sending Notifications in JavaScript
DESCRIPTION: JavaScript implementation for checking notification permissions, requesting them if needed, and sending a notification to the user.

LANGUAGE: javascript
CODE:
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';
// when using `"withGlobalTauri": true`, you may use
// const { isPermissionGranted, requestPermission, sendNotification, } = window.__TAURI__.notification;

// Do you have permission to send a notification?
let permissionGranted = await isPermissionGranted();

// If not we need to request it
if (!permissionGranted) {
  const permission = await requestPermission();
  permissionGranted = permission === 'granted';
}

// Once permission has been granted we can send the notification
if (permissionGranted) {
  sendNotification({ title: 'Tauri', body: 'Tauri is awesome!' });
}

----------------------------------------

TITLE: Registering Deep Links at Runtime in Rust
DESCRIPTION: Rust code to register URL schemes with the operating system at runtime.

LANGUAGE: rust
CODE:
use tauri_plugin_deep_link::DeepLinkExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            #[cfg(desktop)]
            app.deep_link().register("my-app")?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring Deep Link Plugin Permissions in Tauri
DESCRIPTION: JSON configuration for enabling deep link permissions in a Tauri application. This capabilities configuration grants the necessary permissions for the deep-link plugin to function.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/mobile-schema.json",
  "identifier": "mobile-capability",
  "windows": ["main"],
  "platforms": ["iOS", "android"],
  "permissions": [
    // Usually you will need core:event:default to listen to the deep-link event
    "core:event:default",
    "deep-link:default"
  ]
}

----------------------------------------

TITLE: Handling Deep Links in Rust
DESCRIPTION: Rust code to listen for deep link events in the backend of the application.

LANGUAGE: rust
CODE:
use tauri_plugin_deep_link::DeepLinkExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            app.deep_link().on_open_url(|event| {
                println!("deep link URLs: {:?}", event.urls());
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Reading Binary Files with Tauri FS Plugin
DESCRIPTION: Demonstrates how to read a binary file using the readFile function. This function returns the file content as a Uint8Array which is suitable for binary data handling.

LANGUAGE: javascript
CODE:
import { readFile, BaseDirectory } from '@tauri-apps/plugin-fs';
const icon = await readFile('icon.png', {
  baseDir: BaseDirectory.Resources,
});

----------------------------------------

TITLE: Listening to Events Once in JavaScript with Tauri
DESCRIPTION: Shows how to listen to an event exactly once in a Tauri application. The 'once' function automatically unregisters the listener after the first event is received, for both global and webview-specific events.

LANGUAGE: javascript
CODE:
import { once } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

once('ready', (event) => {});

const appWebview = getCurrentWebviewWindow();
appWebview.once('ready', () => {});

----------------------------------------

TITLE: Restoring window state in Rust
DESCRIPTION: Rust code example demonstrating how to manually restore a window's state from disk using the restore_state method exposed by the WindowExt trait.

LANGUAGE: rust
CODE:
use tauri_plugin_window_state::{WindowExt, StateFlags};

// all `Window` types now have the following additional method
window.restore_state(StateFlags::all()); // will restore the window's state from disk

----------------------------------------

TITLE: Defining Permission Sets in TOML
DESCRIPTION: This TOML configuration defines a permission set that groups related individual permissions, providing a higher level of abstraction for managing plugin access.

LANGUAGE: toml
CODE:
"$schema" = "schemas/schema.json"
[[set]]
identifier = "allow-websocket"
description = "Allows connecting and sending messages through a WebSocket"
permissions = ["allow-connect", "allow-send"]

----------------------------------------

TITLE: Remote API Access Capability Configuration
DESCRIPTION: This JSON snippet configures a capability that allows remote sources to access certain Tauri Commands. It permits all subdomains of tauri.app to scan for NFC tags and use the barcode scanner on mobile platforms.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/remote-schema.json",
  "identifier": "remote-tag-capability",
  "windows": ["main"],
  "remote": {
    "urls": ["https://*.tauri.app"]
  },
  "platforms": ["iOS", "android"],
  "permissions": ["nfc:allow-scan", "barcode-scanner:allow-scan"]
}

----------------------------------------

TITLE: Configuring Global FS Scope in Tauri Capabilities
DESCRIPTION: Example configuration for setting global file system scope permissions in a Tauri application. This allows access to the app data directory and all its subdirectories.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "fs:scope",
      "allow": [{ "path": "$APPDATA" }, { "path": "$APPDATA/**" }]
    }
  ]
}

----------------------------------------

TITLE: Uploading Files with JavaScript
DESCRIPTION: Example of using the Upload plugin's JavaScript API to upload a file to a remote server, including progress tracking and custom headers.

LANGUAGE: javascript
CODE:
import { upload } from '@tauri-apps/plugin-upload';
// when using `"withGlobalTauri": true`, you may use
// const { upload } = window.__TAURI__.upload;

upload(
  'https://example.com/file-upload',
  './path/to/my/file.txt',
  ({ progress, total }) =>
    console.log(`Uploaded ${progress} of ${total} bytes`), // a callback that will be called with the upload progress
  { 'Content-Type': 'text/plain' } // optional headers to send with the request
);

----------------------------------------

TITLE: Registering Global Shortcuts in JavaScript
DESCRIPTION: Example of how to register a global keyboard shortcut in JavaScript using the Tauri global-shortcut plugin. This registers Command/Control+Shift+C and defines a callback function.

LANGUAGE: javascript
CODE:
import { register } from '@tauri-apps/plugin-global-shortcut';
// when using `"withGlobalTauri": true`, you may use
// const { register } = window.__TAURI__.globalShortcut;

await register('CommandOrControl+Shift+C', () => {
  console.log('Shortcut triggered');
});

----------------------------------------

TITLE: Implementing Dynamic Menu Status Changes in Rust with Tauri
DESCRIPTION: Creates a Tauri application with menu items that can dynamically change their properties. Demonstrates creating various menu item types and updating their text, icon, checked status, and accelerator keys in response to menu events.

LANGUAGE: rust
CODE:
// change-menu-status
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri:{
    image::Image,
    menu::{CheckMenuItemBuilder, IconMenuItem, MenuBuilder, MenuItem, SubmenuBuilder},
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let check_sub_item_en = CheckMenuItemBuilder::with_id("en", "EN")
                .checked(true)
                .build(app)?;

            let check_sub_item_zh = CheckMenuItemBuilder::with_id("zh", "ZH")
                .checked(false)
                .build(app)?;

            let text_menu = MenuItem::with_id(
                app,
                "change_text",
                &"Change menu".to_string(),
                true,
                Some("Ctrl+Z"),
            )
            .unwrap();

            let icon_menu = IconMenuItem::with_id(
                app,
                "change_icon",
                &"Change icon menu",
                true,
                Some(Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap()),
                Some("Ctrl+F"),
            )
            .unwrap();

            let menu_item = SubmenuBuilder::new(app, "Change menu")
                .item(&text_menu)
                .item(&icon_menu)
                .items(&[&check_sub_item_en, &check_sub_item_zh])
                .build()?;
            let menu = MenuBuilder::new(app).items(&[&menu_item]).build()?;
            app.set_menu(menu)?;
            app.on_menu_event(move |_app_handle: &tauri::AppHandle, event| {
                match event.id().0.as_str() {
                    "change_text" => {
                        text_menu
                            .set_text("changed menu text")
                            .expect("Change text error");

                        text_menu
                            .set_text("changed menu text")
                            .expect("Change text error");
                    }
                    "change_icon" => {
                        icon_menu
                            .set_text("changed menu-icon text")
                            .expect("Change text error");
                        icon_menu
                            .set_icon(Some(
                                Image::from_bytes(include_bytes!("../icons/icon-2.png")).unwrap(),
                            ))
                            .expect("Change icon error");
                    }

                    "en" | "zh" => {
                        check_sub_item_en
                            .set_checked(event.id().0.as_str() == "en")
                            .expect("Change check error");
                        check_sub_item_zh
                            .set_checked(event.id().0.as_str() == "zh")
                            .expect("Change check error");
                        check_sub_item_zh.set_accelerator(Some("Ctrl+L"))
                        .expect("Change accelerator error");
                    }
                    _ => {
                        println!("unexpected menu event");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Handling Menu Events in Rust
DESCRIPTION: Implements menu event handling in Rust using Tauri's on_menu_event API. Listens for menu events and processes them based on the event ID, with custom handlers for open and close events.

LANGUAGE: rust
CODE:
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::menu::{MenuBuilder};

fn main() {
  tauri::Builder::default()
        .setup(|app| {
            let menu = MenuBuilder::new(app)
                .text("open", "Open")
                .text("close", "Close")
                .build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {

                println!("menu event: {:?}", event.id());

                match event.id().0.as_str() {
                    "open" => {
                        println!("open event");
                    }
                    "close" => {
                        println!("close event");
                    }
                    _ => {
                        println!("unexpected menu event");
                    }
                }
            });

            Ok(())
        })
}

----------------------------------------

TITLE: Loading Database with Migrations in JavaScript
DESCRIPTION: JavaScript code showing how to load a database connection which automatically applies migrations defined in the plugin.

LANGUAGE: javascript
CODE:
import Database from '@tauri-apps/plugin-sql';
const db = await Database.load('sqlite:mydatabase.db');

----------------------------------------

TITLE: Creating Files if Not Exists with Tauri FS Plugin
DESCRIPTION: Opens a file with the create option, which creates the file if it doesn't exist. This example shows how to safely write to a file that may not yet exist on the filesystem.

LANGUAGE: javascript
CODE:
import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
const file = await open('foo/bar.txt', {
  write: true,
  create: true,
  baseDir: BaseDirectory.AppData,
});
await file.write(new TextEncoder().encode('world'));
await file.close();

----------------------------------------

TITLE: Making HTTP Requests with JavaScript in Tauri
DESCRIPTION: Example of using the fetch method from the HTTP plugin in JavaScript, which aims to be compliant with the Web API fetch standard.

LANGUAGE: javascript
CODE:
import { fetch } from '@tauri-apps/plugin-http';

// Send a GET request
const response = await fetch('http://test.tauri.app/data.json', {
  method: 'GET',
});
console.log(response.status); // e.g. 200
console.log(response.statusText); // e.g. "OK"

----------------------------------------

TITLE: Adding a Menu to Tray Icon in Rust
DESCRIPTION: Shows how to create a menu with a quit option and attach it to a tray icon in Rust. The menu is built with MenuItem and Menu structs.

LANGUAGE: rust
CODE:
use tauri:{
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
};

let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
let menu = Menu::with_items(app, &[&quit_i])?;

let tray = TrayIconBuilder::new()
  .menu(&menu)
  .menu_on_left_click(true)
  .build(app)?;

----------------------------------------

TITLE: Defining and Parsing Command Arguments in iOS Plugin
DESCRIPTION: Shows how to define and parse command arguments in an iOS Tauri plugin using Decodable structs for required and optional values, demonstrating nested object support.

LANGUAGE: swift
CODE:
class OpenAppArgs: Decodable {
  let name: String
  var timeout: Int?
}

class OpenArgs: Decodable {
  let requiredArg: String
  var allowEdit: Bool?
  var quality: UInt8?
  var app: OpenAppArgs?
}

class ExamplePlugin: Plugin {
	@objc public func openCamera(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(OpenArgs.self)

    invoke.resolve(["path": "/path/to/photo.jpg"])
	}
}

----------------------------------------

TITLE: Configuring and Using Localhost Plugin with Custom Port
DESCRIPTION: Full implementation example showing how to configure the localhost plugin with a specific port and create a webview window that connects to the localhost server.

LANGUAGE: rust
CODE:
use tauri::{webview::WebviewWindowBuilder, WebviewUrl};

pub fn run() {
  let port: u16 = 9527;

  tauri::Builder::default()
      .plugin(tauri_plugin_localhost::Builder::new(port).build())
      .setup(move |app| {
          let url = format!("http://localhost:{}", port).parse().unwrap();
          WebviewWindowBuilder::new(app, "main".to_string(), WebviewUrl::External(url))
              .title("Localhost Example")
              .build()?
          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring WebdriverIO with Tauri Driver in JavaScript
DESCRIPTION: WebdriverIO configuration file (wdio.conf.js) that sets up the test environment for Tauri applications. It handles spinning up the tauri-driver process before tests and cleaning up afterward.

LANGUAGE: javascript
CODE:
const os = require('os');
const path = require('path');
const { spawn, spawnSync } = require('child_process');

// keep track of the `tauri-driver` child process
let tauriDriver;

exports.config = {
  specs: ['./develop/tests/specs/**/*.js'],
  maxInstances: 1,
  capabilities: [
    {
      maxInstances: 1,
      'tauri:options': {
        application: '../../target/release/hello-tauri-webdriver',
      },
    },
  ],
  reporters: ['spec'],
  framework: 'mocha',
  mochaOpts: {
    ui: 'bdd',
    timeout: 60000,
  },

  // ensure the rust project is built since we expect this binary to exist for the webdriver sessions
  onPrepare: () => spawnSync('cargo', ['build', '--release']),

  // ensure we are running `tauri-driver` before the session starts so that we can proxy the webdriver requests
  beforeSession: () =>
    (tauriDriver = spawn(
      path.resolve(os.homedir(), '.cargo', 'bin', 'tauri-driver'),
      [],
      { stdio: [null, process.stdout, process.stderr] }
    )),

  // clean up the `tauri-driver` process we spawned at the start of the session
  afterSession: () => tauriDriver.kill(),
};

----------------------------------------

TITLE: Configuring Content Security Policy in Tauri Configuration File
DESCRIPTION: Example configuration for Content Security Policy in a Tauri application. This JSON snippet demonstrates how to set up CSP rules in the tauri.conf.json file to control what resources can be loaded by the application's webview.

LANGUAGE: json
CODE:
  "csp": {
        "default-src": "'self' customprotocol: asset:",
        "connect-src": "ipc: http://ipc.localhost",
        "font-src": ["https://fonts.gstatic.com"],
        "img-src": "'self' asset: http://asset.localhost blob: data:",
        "style-src": "'unsafe-inline' 'self' https://fonts.googleapis.com"
      },

----------------------------------------

TITLE: Prompting Biometric Authentication in JavaScript
DESCRIPTION: JavaScript code to prompt the user for biometric authentication with customizable options for both Android and iOS platforms.

LANGUAGE: javascript
CODE:
import { authenticate } from '@tauri-apps/plugin-biometric';

const options = {
  // Set true if you want the user to be able to authenticate using phone password
  allowDeviceCredential: false,
  cancelTitle: "Feature won't work if Canceled",

  // iOS only feature
  fallbackTitle: 'Sorry, authentication failed',

  // Android only features
  title: 'Tauri feature',
  subtitle: 'Authenticate to access the locked Tauri function',
  confirmationRequired: true,
};

try {
  await authenticate('This feature is locked', options);
  console.log(
    'Hooray! Successfully Authenticated! We can now perform the locked Tauri function!'
  );
} catch (err) {
  console.log('Oh no! Authentication failed because ' + err.message);
}

----------------------------------------

TITLE: Configuring Basic package.json for Tauri Applications
DESCRIPTION: A minimal package.json configuration for a Tauri project that includes development scripts and essential Tauri dependencies. This configuration enables frontend development workflow with commands for development mode and building.

LANGUAGE: json
CODE:
{
  "scripts": {
    "dev": "command to start your app development mode",
    "build": "command to build your app frontend",
    "tauri": "tauri"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.0.0.0",
    "@tauri-apps/cli": "^2.0.0.0"
  }
}

----------------------------------------

TITLE: Configuring Log Rotation Strategy
DESCRIPTION: Setting a rotation strategy to keep all log files when the maximum size is reached.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
  .build()

----------------------------------------

TITLE: Executing MySQL Queries with Parameters
DESCRIPTION: JavaScript code showing how to execute parameterized MySQL queries using the question mark syntax for parameter placeholders.

LANGUAGE: javascript
CODE:
const result = await db.execute(
  "INSERT into todos (id, title, status) VALUES (?, ?, ?)",
  [todos.id, todos.title, todos.status],
);

const result = await db.execute(
  "UPDATE todos SET title = ?, status = ? WHERE id = ?",
  [todos.title, todos.status, todos.id],
);

----------------------------------------

TITLE: Building Message Dialog in Rust
DESCRIPTION: Rust implementation of a blocking message dialog with error styling using the DialogExt trait.

LANGUAGE: rust
CODE:
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

let ans = app.dialog()
    .message("File not found")
    .kind(MessageDialogKind::Error)
    .title("Warning")
    .blocking_show();

----------------------------------------

TITLE: Debugging Key Bindings for Neovim
DESCRIPTION: Example keyboard mappings for controlling debugging sessions in Neovim, including starting/stopping debugging, stepping through code, toggling breakpoints, and managing the overseer task panel.

LANGUAGE: lua
CODE:
vim.keymap.set('n', '<F5>', function() dap.continue() end)
vim.keymap.set('n', '<F6>', function() dap.disconnect({ terminateDebuggee = true }) end)
vim.keymap.set('n', '<F10>', function() dap.step_over() end)
vim.keymap.set('n', '<F11>', function() dap.step_into() end)
vim.keymap.set('n', '<F12>', function() dap.step_out() end)
vim.keymap.set('n', '<Leader>b', function() dap.toggle_breakpoint() end)
vim.keymap.set('n', '<Leader>o', function() overseer.toggle() end)
vim.keymap.set('n', '<Leader>R', function() overseer.run_template() end)


----------------------------------------

TITLE: Configuring HTTP Headers in Tauri Configuration File
DESCRIPTION: Example configuration of HTTP headers in the Tauri configuration file. This demonstrates how to set various header types using different formats including strings, arrays, and key-value objects.

LANGUAGE: javascript
CODE:
{
 //...
  "app":{
    //...
    "security": {
      //...
      "headers": {
        "Cross-Origin-Opener-Policy": "same-origin",
        "Cross-Origin-Embedder-Policy": "require-corp",
        "Timing-Allow-Origin": [
          "https://developer.mozilla.org",
          "https://example.com",
        ],
        "X-Content-Type-Options": null, // gets ignored
        "Access-Control-Expose-Headers": "Tauri-Custom-Header",
        "Tauri-Custom-Header": {
          "key1": "'value1' 'value2'",
          "key2": "'value3'"
        }
      },
      // notice how the CSP is not defined under headers
      "csp": "default-src 'self'; connect-src ipc: http://ipc.localhost",
    }
  }
}

----------------------------------------

TITLE: Configuring Store Plugin Permissions in Capabilities JSON
DESCRIPTION: JSON configuration example showing how to add the necessary permission for the Store plugin to the capabilities configuration.

LANGUAGE: json
CODE:
{
  "permissions": [
    ...,
    "store:default",
  ]
}

----------------------------------------

TITLE: Setting Custom Target for Updates in Rust for Tauri
DESCRIPTION: This Rust implementation demonstrates two approaches to set a custom target for updates: using the plugin builder or the updater builder. This is useful for specialized builds like Universal macOS binaries.

LANGUAGE: rust
CODE:
tauri_plugin_updater::Builder::new().target("macos-universal").build()

LANGUAGE: rust
CODE:
use tauri_plugin_updater::UpdaterExt;
let update = app
  .updater_builder()
  .target("macos-universal")
  .build()?
  .check()
  .await?;

----------------------------------------

TITLE: Reading a File with Path API in JavaScript
DESCRIPTION: JavaScript code demonstrating how to read a file using the Tauri file system plugin with the path API for path manipulations.

LANGUAGE: javascript
CODE:
import { readFile } from '@tauri-apps/plugin-fs';
import * as path from '@tauri-apps/api/path';
const home = await path.homeDir();
const contents = await readFile(await path.join(home, 'avatars/tauri.png'));

----------------------------------------

TITLE: Accessing Resource Files in JavaScript
DESCRIPTION: JavaScript code example showing how to access bundled resource files from the frontend. This demonstrates resolving resource paths and reading text files using the Tauri API and plugin-fs.

LANGUAGE: javascript
CODE:
import { resolveResource } from '@tauri-apps/api/path';
import { readTextFile } from '@tauri-apps/plugin-fs';

const resourcePath = await resolveResource('lang/de.json');
const langDe = JSON.parse(await readTextFile(resourcePath));
console.log(langDe.hello); // This will print 'Guten Tag!' to the devtools console

----------------------------------------

TITLE: Using LazyStore in JavaScript/TypeScript
DESCRIPTION: Example of how to use the high-level LazyStore API in JavaScript or TypeScript, which only loads the store on first access.

LANGUAGE: typescript
CODE:
import { LazyStore } from '@tauri-apps/plugin-store';

const store = new LazyStore('settings.json');

----------------------------------------

TITLE: Autogenerating Permissions in build.rs
DESCRIPTION: This build script shows how to autogenerate permissions for commands by defining the command names in the COMMANDS constant, which will create allow and deny permissions for each command.

LANGUAGE: rust
CODE:
const COMMANDS: &[&str] = &["upload"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}

----------------------------------------

TITLE: Renaming Files with Tauri FS Plugin
DESCRIPTION: Renames a file from a source path to a destination path. Like copyFile, you can specify different base directories for the source and destination paths, allowing for moves between directories.

LANGUAGE: javascript
CODE:
import { rename, BaseDirectory } from '@tauri-apps/plugin-fs';
await rename('user.db.bk', 'user.db', {
  fromPathBaseDir: BaseDirectory.AppLocalData,
  toPathBaseDir: BaseDirectory.Temp,
});

----------------------------------------

TITLE: Connecting to PostgreSQL Database in JavaScript
DESCRIPTION: JavaScript code that loads and interacts with a PostgreSQL database using the Tauri SQL plugin.

LANGUAGE: javascript
CODE:
import Database from '@tauri-apps/plugin-sql';
// when using `"withGlobalTauri": true`, you may use
// const Database = window.__TAURI__.sql;

const db = await Database.load('postgres://user:password@host/test');
await db.execute('INSERT INTO ...');

----------------------------------------

TITLE: Reading Directory Contents with Tauri FS Plugin
DESCRIPTION: Lists all entries in a directory recursively. This function returns information about all files and subdirectories within the specified directory.

LANGUAGE: typescript
CODE:
import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs';
const entries = await readDir('users', { baseDir: BaseDirectory.AppLocalData });

----------------------------------------

TITLE: Configuring nvim-dap for Rust Debugging in Neovim
DESCRIPTION: Basic configuration for the nvim-dap plugin to enable Rust debugging in Neovim. This setup configures CodeLLDB as the debugging adapter and provides a launch configuration for debugging Rust applications.

LANGUAGE: lua
CODE:
local dap = require("dap")

dap.adapters.codelldb = {
  type = 'server',
  port = "${port}",
  executable = {
    -- Change this to your path!
    command = '/opt/codelldb/adapter/codelldb',
    args = {"--port", "${port}"},
  }
}

dap.configurations.rust= {
  {
    name = "Launch file",
    type = "codelldb",
    request = "launch",
    program = function()
      return vim.fn.input('Path to executable: ', vim.fn.getcwd() .. '/target/debug/', 'file')
    end,
    cwd = '${workspaceFolder}',
    stopOnEntry = false
  },
}

----------------------------------------

TITLE: Attaching Console to Log Stream in JavaScript
DESCRIPTION: JavaScript code to connect the webview console to the log stream from the Rust backend.

LANGUAGE: javascript
CODE:
import { attachConsole } from '@tauri-apps/plugin-log';
const detach = await attachConsole();
// call detach() if you do not want to print logs to the console anymore

----------------------------------------

TITLE: Picking Files with Blocking Dialog in Rust
DESCRIPTION: Rust example showing how to open a blocking file picker dialog and handle the selected file path.

LANGUAGE: rust
CODE:
use tauri_plugin_dialog::DialogExt;

let file_path = app.dialog().file().blocking_pick_file();
// return a file_path `Option`, or `None` if the user closes the dialog

----------------------------------------

TITLE: Configuring Tauri JSON Settings for Trunk Integration
DESCRIPTION: This JSON configuration sets up Tauri to work with Trunk. It defines build commands for development and production, specifies the development URL, sets the frontend distribution path, and enables the global Tauri object for WASM-bindgen access.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "trunk serve",
    "beforeBuildCommand": "trunk build",
    "devUrl": "http://localhost:8080",
    "frontendDist": "../dist"
  },
  "app": {
    "withGlobalTauri": true
  }
}

----------------------------------------

TITLE: Running Sidecar from Rust with Tauri Shell Plugin
DESCRIPTION: Rust code example demonstrating how to use the tauri_plugin_shell to spawn a sidecar process, handle its output events, and write to its stdin.

LANGUAGE: rust
CODE:
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

let sidecar_command = app.shell().sidecar("my-sidecar").unwrap();
let (mut rx, mut _child) = sidecar_command
  .spawn()
  .expect("Failed to spawn sidecar");

tauri::async_runtime::spawn(async move {
  // read events such as stdout
  while let Some(event) = rx.recv().await {
    if let CommandEvent::Stdout(line_bytes) = event {
      let line = String::from_utf8_lossy(&line_bytes);
      window
        .emit("message", Some(format!("'{}'", line)))
        .expect("failed to emit event");
      // write to stdin
      child.write("message from Rust\n".as_bytes()).unwrap();
    }
  }
});

----------------------------------------

TITLE: Forwarding Console Messages to Log Plugin
DESCRIPTION: TypeScript utility function to forward standard console messages to the Tauri log plugin for unified logging.

LANGUAGE: typescript
CODE:
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

function forwardConsole(
  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  logger: (message: string) => Promise<void>
) {
  const original = console[fnName];
  console[fnName] = (message) => {
    original(message);
    logger(message);
  };
}

forwardConsole('log', trace);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

----------------------------------------

TITLE: Truncating Files to Zero Length with Tauri FS Plugin
DESCRIPTION: Truncates a file to zero length, effectively emptying it while keeping the file itself. This is useful when you want to clear a file's contents without deleting and recreating it.

LANGUAGE: typescript
CODE:
import { truncate } from '@tauri-apps/plugin-fs';
await truncate('my_file.txt', 0, { baseDir: BaseDirectory.AppLocalData });

----------------------------------------

TITLE: Handling Menu Events in JavaScript
DESCRIPTION: Creates a menu with custom items in JavaScript and defines action handlers for when menu items are clicked. Uses the Menu API to set event listeners for open and close actions.

LANGUAGE: javascript
CODE:
import { Menu } from '@tauri-apps/api/menu';

const menu = await Menu.new({
  items: [
    {
      id: 'Open',
      text: 'open',
      action: () => {
        console.log('open pressed');
      },
    },
    {
      id: 'Close',
      text: 'close',
      action: () => {
        console.log('close pressed');
      },
    },
  ],
});

await menu.setAsAppMenu();

----------------------------------------

TITLE: Picking Files with Non-Blocking Dialog in Rust
DESCRIPTION: Rust example demonstrating how to open a non-blocking file picker dialog with a callback to process the result.

LANGUAGE: rust
CODE:
use tauri_plugin_dialog::DialogExt;

app.dialog().file().pick_file(|file_path| {
    // return a file_path `Option`, or `None` if the user closes the dialog
    })

----------------------------------------

TITLE: Configuring Next.js for Tauri Compatibility
DESCRIPTION: TypeScript configuration for next.conf.mjs that sets up Next.js to use static exports (SSG) instead of server-side rendering (SSR). Includes configuration for asset prefixes and unoptimized images.

LANGUAGE: typescript
CODE:
// next.conf.mjs
const isProd = process.env.NODE_ENV === 'production';

const internalHost = process.env.TAURI_DEV_HOST || 'localhost';

/** @type {import('next').NextConfig} */
const nextConfig = {
  // Ensure Next.js uses SSG instead of SSR
  // https://nextjs.org/docs/pages/building-your-application/deploying/static-exports
  output: 'export',
  // Note: This feature is required to use the Next.js Image component in SSG mode.
  // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
  images: {
    unoptimized: true,
  },
  // Configure assetPrefix or else the server won't properly resolve your assets.
  assetPrefix: isProd ? undefined : `http://${internalHost}:3000`,
};

export default nextConfig;

----------------------------------------

TITLE: Checking if Directories Exist with Tauri FS Plugin
DESCRIPTION: Checks if a directory exists at the specified path. This function returns a boolean and works the same way as checking if a file exists.

LANGUAGE: javascript
CODE:
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';
const tokenExists = await exists('images', {
  baseDir: BaseDirectory.AppLocalData,
});

----------------------------------------

TITLE: Using Process Plugin in Rust
DESCRIPTION: Rust code example demonstrating how to exit the app with a status code and restart the application using the AppHandle instance.

LANGUAGE: rust
CODE:
// exits the app with the given status code
app.exit(0);

// restarts the app
app.restart();

----------------------------------------

TITLE: Initializing Tauri Log Plugin in Rust
DESCRIPTION: Basic initialization of the log plugin in the Tauri application's entry point. Adds the plugin to the Tauri builder chain.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring package.json for Selenium Testing with Tauri
DESCRIPTION: A package.json file for Selenium WebDriver testing with Tauri applications, specifying Mocha as the test framework, Chai as the assertion library, and selenium-webdriver for browser automation.

LANGUAGE: json
CODE:
{
  "name": "selenium",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "test": "mocha"
  },
  "dependencies": {
    "chai": "^4.3.4",
    "mocha": "^9.0.3",
    "selenium-webdriver": "^4.0.0-beta.4"
  }
}

----------------------------------------

TITLE: Accessing Resource Files in Rust with App Setup
DESCRIPTION: Rust code example showing how to access bundled resource files during app setup. This demonstrates loading a JSON file from the resources directory and parsing its contents.

LANGUAGE: rust
CODE:
tauri::Builder::default()
  .setup(|app| {
    // The path specified must follow the same syntax as defined in
    // `tauri.conf.json > bundle > resources`
    let resource_path = app.path().resolve("lang/de.json", BaseDirectory::Resource)?;

    let file = std::fs::File::open(&resource_path).unwrap();
    let lang_de: serde_json::Value = serde_json::from_reader(file).unwrap();

    // This will print 'Guten Tag!' to the terminal
    println!("{}", lang_de.get("hello").unwrap());

    Ok(())
  })

----------------------------------------

TITLE: Using Clipboard Plugin in JavaScript
DESCRIPTION: Example showing how to use the clipboard plugin in JavaScript to write text to the clipboard and read text from it. It demonstrates importing the functions and using them with async/await.

LANGUAGE: javascript
CODE:
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';
// when using `"withGlobalTauri": true`, you may use
// const { writeText, readText } = window.__TAURI__.clipboardManager;

// Write content to clipboard
await writeText('Tauri is awesome!');

// Read content from clipboard
const content = await readText();
console.log(content);
// Prints "Tauri is awesome!" to the console

----------------------------------------

TITLE: Configuring Next.js for Tauri Compatibility
DESCRIPTION: TypeScript configuration for next.conf.mjs that sets up Next.js to use static exports (SSG) instead of server-side rendering (SSR). Includes configuration for asset prefixes and unoptimized images.

LANGUAGE: typescript
CODE:
// next.conf.mjs
const isProd = process.env.NODE_ENV === 'production';

const internalHost = process.env.TAURI_DEV_HOST || 'localhost';

/** @type {import('next').NextConfig} */
const nextConfig = {
  // Ensure Next.js uses SSG instead of SSR
  // https://nextjs.org/docs/pages/building-your-application/deploying/static-exports
  output: 'export',
  // Note: This feature is required to use the Next.js Image component in SSG mode.
  // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
  images: {
    unoptimized: true,
  },
  // Configure assetPrefix or else the server won't properly resolve your assets.
  assetPrefix: isProd ? undefined : `http://${internalHost}:3000`,
};

export default nextConfig;

----------------------------------------

TITLE: Dynamic Update Server Response Format for Tauri Updater
DESCRIPTION: JSON response format for a dynamic update server implementation with the Tauri updater plugin. Includes version, publication date, download URL, signature, and release notes.

LANGUAGE: json
CODE:
{
  "version": "",
  "pub_date": "",
  "url": "",
  "signature": "",
  "notes": ""
}

----------------------------------------

TITLE: Scanning NFC Tags in Rust
DESCRIPTION: Rust code to scan NFC tags using the scan method from the NFC plugin. It demonstrates how to configure a scan request for NDEF formatted tags.

LANGUAGE: rust
CODE:
tauri::Builder::default()
  .setup(|app| {
    #[cfg(mobile)]
    {
      use tauri_plugin_nfc::NfcExt;

      app.handle().plugin(tauri_plugin_nfc::init());

      let tag = app
        .nfc()
        .scan(tauri_plugin_nfc::ScanRequest {
            kind: tauri_plugin_nfc::ScanKind::Ndef {
                mime_type: None,
                uri: None,
                tech_list: None,
            },
            keep_session_alive: false,
        })?
        .tag;
    }
    Ok(())
  })

----------------------------------------

TITLE: Sending Notifications in Rust
DESCRIPTION: Rust implementation for sending a notification using the Tauri notification plugin builder pattern.

LANGUAGE: rust
CODE:
tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .setup(|app| {
        use tauri_plugin_notification::NotificationExt;
        app.notification()
            .builder()
            .title("Tauri")
            .body("Tauri is awesome")
            .show()
            .unwrap();

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

----------------------------------------

TITLE: Saving File Dialog in JavaScript
DESCRIPTION: Example showing how to open a file save dialog with custom file filters for specific file extensions.

LANGUAGE: javascript
CODE:
import { save } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { save } = window.__TAURI__.dialog;

// Prompt to save a 'My Filter' with extension .png or .jpeg
const path = await save({
  filters: [
    {
      name: 'My Filter',
      extensions: ['png', 'jpeg'],
    },
  ],
});
console.log(path);
// Prints the chosen path

----------------------------------------

TITLE: Writing Binary Data to a File in JavaScript
DESCRIPTION: JavaScript code showing how to write binary data to a file using the Tauri file system plugin's writeFile API with a Uint8Array.

LANGUAGE: javascript
CODE:
import { writeFile, BaseDirectory } from '@tauri-apps/plugin-fs';
const contents = new Uint8Array(); // fill a byte array
await writeFile('config', contents, {
  baseDir: BaseDirectory.AppConfig,
});

----------------------------------------

TITLE: Creating Ok/Cancel Dialog in JavaScript
DESCRIPTION: Example showing how to create an Ok/Cancel confirmation dialog with warning styling using the confirm function.

LANGUAGE: javascript
CODE:
import { confirm } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { confirm } = window.__TAURI__.dialog;

// Creates a confirmation Ok/Cancel dialog
const confirmation = await confirm(
  'This action cannot be reverted. Are you sure?',
  { title: 'Tauri', kind: 'warning' }
);

console.log(confirmation);
// Prints boolean to the console

----------------------------------------

TITLE: Implementing Mobile Plugin in Swift for iOS
DESCRIPTION: Example showing how to create a Tauri plugin for iOS by implementing a Swift subclass. This pattern allows developers to extend Tauri functionality with native Swift code that can be exposed to the frontend.

LANGUAGE: swift
CODE:
YourPluginClass: Plugin

----------------------------------------

TITLE: Writing Text to a File in JavaScript
DESCRIPTION: JavaScript code demonstrating how to write text content to a file using the Tauri file system plugin's writeTextFile API.

LANGUAGE: javascript
CODE:
import { writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
const contents = JSON.stringify({ notifications: true });
await writeTextFile('config.json', contents, {
  baseDir: BaseDirectory.AppConfig,
});

----------------------------------------

TITLE: Emitting Global Events from Rust in Tauri
DESCRIPTION: This code demonstrates how to trigger global events from Rust to the frontend using the Emitter trait. It shows a download function that emits events for download progress updates.

LANGUAGE: rust
CODE:
use tauri::{AppHandle, Emitter};

#[tauri::command]
fn download(app: AppHandle, url: String) {
  app.emit("download-started", &url).unwrap();
  for progress in [1, 15, 50, 80, 100] {
    app.emit("download-progress", progress).unwrap();
  }
  app.emit("download-finished", &url).unwrap();
}

----------------------------------------

TITLE: Installing Flatpak Tools on Debian
DESCRIPTION: Command to install the required flatpak and flatpak-builder tools on Debian-based distributions.

LANGUAGE: shell
CODE:
sudo apt install flatpak flatpak-builder

----------------------------------------

TITLE: Handling Notification Action Events in JavaScript
DESCRIPTION: Code to listen for and handle user interactions with notification actions.

LANGUAGE: javascript
CODE:
import { onAction } from '@tauri-apps/plugin-notification';

await onAction((notification) => {
  console.log('Action performed:', notification);
});

----------------------------------------

TITLE: Configuring Android App Links JSON
DESCRIPTION: JSON configuration for Android app links to associate URLs with your application.

LANGUAGE: json
CODE:
[
  {
    "relation": ["delegate_permission/common.handle_all_urls"],
    "target": {
      "namespace": "android_app",
      "package_name": "$APP_BUNDLE_ID",
      "sha256_cert_fingerprints": [
        $CERT_FINGERPRINT
      ]
    }
  }
]

----------------------------------------

TITLE: Creating Yes/No Dialog in JavaScript
DESCRIPTION: Example of creating a Yes/No dialog with warning styling using the Dialog plugin's ask function.

LANGUAGE: javascript
CODE:
import { ask } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { ask } = window.__TAURI__.dialog;

// Create a Yes/No dialog
const answer = await ask('This action cannot be reverted. Are you sure?', {
  title: 'Tauri',
  kind: 'warning',
});

console.log(answer);
// Prints boolean to the console

----------------------------------------

TITLE: Using the Shell Plugin in JavaScript
DESCRIPTION: Example of using the shell plugin in JavaScript to create and execute a command that runs a shell script.

LANGUAGE: javascript
CODE:
import { Command } from '@tauri-apps/plugin-shell';
// when using `"withGlobalTauri": true`, you may use
// const { Command } = window.__TAURI__.shell;

let result = await Command.create('exec-sh', [
  '-c',
  "echo 'Hello World!'",
]).execute();
console.log(result);

----------------------------------------

TITLE: Checking Biometric Authentication Status in JavaScript
DESCRIPTION: JavaScript code to check if biometric authentication is available on the device using the Tauri biometric plugin.

LANGUAGE: javascript
CODE:
import { checkStatus } from '@tauri-apps/plugin-biometric';

const status = await checkStatus();
if (status.isAvailable) {
  console.log('Yes! Biometric Authentication is available');
} else {
  console.log(
    'No! Biometric Authentication is not available due to ' + status.error
  );
}

----------------------------------------

TITLE: Configuring Tauri Application Settings
DESCRIPTION: A JSON configuration file for Tauri that specifies the frontend directory, bundle identifier, icon, and window settings. It disables all Tauri features in the allowlist as they are not needed for this minimal example.

LANGUAGE: json
CODE:
{
  "build": {
    "distDir": "dist"
  },
  "tauri": {
    "bundle": {
      "identifier": "studio.tauri.hello_tauri_webdriver",
      "icon": ["icon.png"]
    },
    "allowlist": {
      "all": false
    },
    "windows": [
      {
        "width": 800,
        "height": 600,
        "resizable": true,
        "fullscreen": false
      }
    ]
  }
}

----------------------------------------

TITLE: Configuring Tauri with Deno for Nuxt Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using Deno as the package manager. Sets up build commands, development URL, and frontend distribution path for a Nuxt project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "deno task dev",
    "beforeBuildCommand": "deno task generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Reading a File with Base Directory in JavaScript
DESCRIPTION: JavaScript code showing how to read a file using the Tauri file system plugin with a base directory as the working directory for the operation.

LANGUAGE: javascript
CODE:
import { readFile } from '@tauri-apps/plugin-fs';
const contents = await readFile('avatars/tauri.png', {
  baseDir: BaseDirectory.Home,
});

----------------------------------------

TITLE: Installing JavaScript Guest Bindings
DESCRIPTION: Commands to install the JavaScript guest bindings for the positioner plugin using different package managers.

LANGUAGE: sh
CODE:
npm install @tauri-apps/plugin-positioner

LANGUAGE: sh
CODE:
yarn add @tauri-apps/plugin-positioner

LANGUAGE: sh
CODE:
pnpm add @tauri-apps/plugin-positioner

LANGUAGE: sh
CODE:
deno add npm:@tauri-apps/plugin-positioner

LANGUAGE: sh
CODE:
bun add @tauri-apps/plugin-positioner

----------------------------------------

TITLE: Mocking Sidecar and Shell Command IPC in Tauri
DESCRIPTION: Demonstrates how to mock IPC requests for sidecar or shell commands by capturing the event handler ID and emitting events that the backend would normally send back like Stdout and Terminated events.

LANGUAGE: javascript
CODE:
mockIPC(async (cmd, args) => {
  if (args.message.cmd === 'execute') {
    const eventCallbackId = `_${args.message.onEventFn}`;
    const eventEmitter = window[eventCallbackId];

    // 'Stdout' event can be called multiple times
    eventEmitter({
      event: 'Stdout',
      payload: 'some data sent from the process',
    });

    // 'Terminated' event must be called at the end to resolve the promise
    eventEmitter({
      event: 'Terminated',
      payload: {
        code: 0,
        signal: 'kill',
      },
    });
  }
});

----------------------------------------

TITLE: Truncating Files with Tauri FS Plugin
DESCRIPTION: Opens a file with the truncate option, which empties the file if it exists before writing to it. This example demonstrates how to clear a file's contents before writing new data.

LANGUAGE: javascript
CODE:
import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
const file = await open('foo/bar.txt', {
  write: true,
  truncate: true,
  baseDir: BaseDirectory.AppData,
});
await file.write(new TextEncoder().encode('world'));
await file.close();

----------------------------------------

TITLE: Initializing Dialog Plugin in Rust
DESCRIPTION: Code snippet showing how to initialize the Tauri Dialog plugin in the Rust application entry point.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Using the Shell Plugin in Rust
DESCRIPTION: Example of using the shell plugin in Rust to execute a command and handle its output asynchronously.

LANGUAGE: rust
CODE:
use tauri_plugin_shell::ShellExt;

let shell = app_handle.shell();
let output = tauri::async_runtime::block_on(async move {
		shell
				.command("echo")
				.args(["Hello from Rust!"])
				.output()
				.await
				.unwrap()
});
if output.status.success() {
		println!("Result: {:?}", String::from_utf8(output.stdout));
} else {
		println!("Exit with code: {}", output.status.code().unwrap());
}

----------------------------------------

TITLE: Listing Notification Channels in JavaScript
DESCRIPTION: Code to retrieve a list of existing notification channels.

LANGUAGE: javascript
CODE:
import { channels } from '@tauri-apps/plugin-notification';

const existingChannels = await channels();

----------------------------------------

TITLE: Using Store Plugin in JavaScript/TypeScript
DESCRIPTION: Example of how to use the Store plugin in JavaScript or TypeScript, demonstrating loading a store, setting and getting values, and saving changes.

LANGUAGE: typescript
CODE:
import { load } from '@tauri-apps/plugin-store';
// when using `"withGlobalTauri": true`, you may use
// const { load } = window.__TAURI__.store;

// Create a new store or load the existing one,
// note that the options will be ignored if a `Store` with that path has already been created
const store = await load('store.json', { autoSave: false });

// Set a value.
await store.set('some-key', { value: 5 });

// Get a value.
const val = await store.get<{ value: number }>('some-key');
console.log(val); // { value: 5 }

// You can manually save the store after making changes.
// Otherwise, it will save upon graceful exit
// And if you set `autoSave` to a number or left empty,
// it will save the changes to disk after a debounce delay, 100ms by default.
await store.save();

----------------------------------------

TITLE: Handling Deep Links in JavaScript
DESCRIPTION: JavaScript code to listen for deep link events in the frontend of the application.

LANGUAGE: javascript
CODE:
import { onOpenUrl } from '@tauri-apps/plugin-deep-link';
// when using `"withGlobalTauri": true`, you may use
// const { onOpenUrl } = window.__TAURI__.deepLink;

await onOpenUrl((urls) => {
  console.log('deep link:', urls);
});

----------------------------------------

TITLE: Implementing a Command in Android Plugin
DESCRIPTION: Shows how to implement a command in an Android Tauri plugin that can be called from Rust code, demonstrating the basic structure with return value handling.

LANGUAGE: kotlin
CODE:
import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
  @Command
  fun openCamera(invoke: Invoke) {
    val ret = JSObject()
    ret.put("path", "/path/to/photo.jpg")
    invoke.resolve(ret)
  }
}

----------------------------------------

TITLE: Initializing DevTools Plugin in Tauri Application
DESCRIPTION: Rust code snippet showing how to initialize and integrate the DevTools plugin with a Tauri application. The code conditionally enables the plugin only in debug builds and initializes it early in the application's execution for optimal instrumentation.

LANGUAGE: rust
CODE:
fn main() {
    // This should be called as early in the execution of the app as possible
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring Resource Access Permissions in Capabilities JSON
DESCRIPTION: Configuration for the Tauri access control list to enable file system access to resources. This example shows how to configure permissions for reading text files and recursive resource access in the capabilities file.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "path:default",
    "event:default",
    "window:default",
    "app:default",
    "resources:default",
    "menu:default",
    "tray:default",
    "fs:allow-read-text-file",
    "fs:allow-resource-read-recursive"
  ]
}

----------------------------------------

TITLE: Creating Predefined Menu Items in Rust
DESCRIPTION: Implements predefined menu items in Rust using Tauri's MenuBuilder and PredefinedMenuItem. Shows both direct method calls like .copy() and using the PredefinedMenuItem class with custom text.

LANGUAGE: rust
CODE:
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::menu::{MenuBuilder, PredefinedMenuItem};

fn main() {
  tauri::Builder::default()
        .setup(|app| {
      let menu = MenuBuilder::new(app)
                .copy()
                .separator()
                .undo()
                .redo()
                .cut()
                .paste()
                .select_all()
                .item(&PredefinedMenuItem::copy(app, Some("custom text"))?)
                .build()?;
            app.set_menu(menu)?;

            Ok(())
        })
}

----------------------------------------

TITLE: Initializing Single Instance Plugin in Tauri Application
DESCRIPTION: Code snippet showing how to modify the lib.rs file to initialize the Single Instance plugin within a Tauri application setup.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Adding PostgreSQL Engine Support with Cargo
DESCRIPTION: Command to add PostgreSQL support to the tauri-plugin-sql dependency.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-sql --features postgres

----------------------------------------

TITLE: Adding Notification Plugin to Cargo Dependencies
DESCRIPTION: Shows how to add the Notification plugin to your Cargo.toml dependencies for a Tauri project.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-notification = "2"

----------------------------------------

TITLE: Sending a Notification Using a Channel in JavaScript
DESCRIPTION: Code to send a notification through a specific channel, allowing for grouped notification behaviors.

LANGUAGE: javascript
CODE:
import { sendNotification } from '@tauri-apps/plugin-notification';

sendNotification({
  title: 'New Message',
  body: 'You have a new message',
  channelId: 'messages',
});

----------------------------------------

TITLE: Using Opener Plugin in JavaScript
DESCRIPTION: Example of how to use the opener plugin in JavaScript to open files with default or specified programs.

LANGUAGE: javascript
CODE:
import { openPath } from '@tauri-apps/plugin-opener';
// when using `"withGlobalTauri": true`, you may use
// const { openPath } = window.__TAURI__.opener;

// opens a file using the default program:
await openPath('/path/to/file');
// opens a file using `vlc` command on Windows:
await openPath('C:/path/to/file', 'vlc');

----------------------------------------

TITLE: Building Non-Blocking Message Dialog in Rust
DESCRIPTION: Rust implementation of a non-blocking message dialog with info styling and a custom button label.

LANGUAGE: rust
CODE:
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

app.dialog()
    .message("Tauri is Awesome")
    .kind(MessageDialogKind::Info)
    .title("Information")
    .buttons(MessageDialogButtons::OkCustom("Absolutely"))
    .show(|result| match result {
        true => // do something,
        false => // do something,
    });

----------------------------------------

TITLE: Adding SQLite Engine Support with Cargo
DESCRIPTION: Command to add SQLite support to the tauri-plugin-sql dependency.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-sql --features sqlite

----------------------------------------

TITLE: Implementing Windows Before Exit Hook for Updates
DESCRIPTION: Code snippet showing how to implement a hook that runs before the application exits on Windows during an update installation. This is necessary because Windows installers require the application to be closed before updates can be installed.

LANGUAGE: rust
CODE:
use tauri_plugin_updater::UpdaterExt;

let update = app
  .updater_builder()
  .on_before_exit(|| {
    println!("app is about to exit on Windows!");
  })
  .build()?
  .check()
  .await?;

----------------------------------------

TITLE: Initializing Stronghold with argon2 password hash function
DESCRIPTION: Example of initializing the Stronghold plugin with the built-in argon2 password hashing algorithm, using a salt file stored in the app's local data directory.

LANGUAGE: rust
CODE:
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Opening File Selector Dialog in JavaScript
DESCRIPTION: Example demonstrating how to open a file selection dialog with options for multiple selection and directory selection.

LANGUAGE: javascript
CODE:
import { open } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { open } = window.__TAURI__.dialog;

// Open a dialog
const file = await open({
  multiple: false,
  directory: false,
});
console.log(file);
// Prints file path or URI

----------------------------------------

TITLE: Changing Menu Status with JavaScript in Tauri
DESCRIPTION: Creates an application menu with various menu item types and demonstrates how to dynamically update their properties like text, icons, and checked status. Uses the Tauri API's menu components to create and manipulate menu items.

LANGUAGE: javascript
CODE:
import {
  Menu,
  CheckMenuItem,
  IconMenuItem,
  MenuItem,
} from '@tauri-apps/api/menu';
import { Image } from '@tauri-apps/api/image';

let currentLanguage = 'en';

const check_sub_item_en = await CheckMenuItem.new({
  id: 'en',
  text: 'English',
  checked: currentLanguage === 'en',
  action: () => {
    currentLanguage = 'en';
    check_sub_item_en.setChecked(currentLanguage === 'en');
    check_sub_item_zh.setChecked(currentLanguage === 'cn');
    console.log('English pressed');
  },
});

const check_sub_item_zh = await CheckMenuItem.new({
  id: 'zh',
  text: 'Chinese',
  checked: currentLanguage === 'zh',
  action: () => {
    currentLanguage = 'zh';
    check_sub_item_en.setChecked(currentLanguage === 'en');
    check_sub_item_zh.setChecked(currentLanguage === 'zh');
    check_sub_item_zh.setAccelerator('Ctrl+L');
    console.log('Chinese pressed');
  },
});

// Load icon from path
const icon = await Image.fromPath('../src/icon.png');
const icon2 = await Image.fromPath('../src/icon-2.png');

const icon_item = await IconMenuItem.new({
  id: 'icon_item',
  text: 'Icon Item',
  icon: icon,
  action: () => {
    icon_item.setIcon(icon2);
    console.log('icon pressed');
  },
});

const text_item = await MenuItem.new({
  id: 'text_item',
  text: 'Text Item',
  action: () => {
    text_item.setText('Text Item Changed');
    console.log('text pressed');
  },
});

const menu = await Menu.new({
  items: [
    {
      id: 'change menu',
      text: 'change_menu',
      items: [text_item, check_sub_item_en, check_sub_item_zh, icon_item],
    },
  ],
});

await menu.setAsAppMenu();

----------------------------------------

TITLE: Installing Tauri Log Plugin with Cargo
DESCRIPTION: Command to add the log plugin to the project's dependencies in Cargo.toml using cargo add.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-log

----------------------------------------

TITLE: Setting Public Key at Runtime in Rust for Tauri Updates
DESCRIPTION: This code demonstrates two approaches to set a public key at runtime for Tauri updates: using the plugin builder or the updater builder. This allows for implementing key rotation logic for more secure updates.

LANGUAGE: rust
CODE:
tauri_plugin_updater::Builder::new().pubkey("<your public key>").build()

LANGUAGE: rust
CODE:
use tauri_plugin_updater::UpdaterExt;

let update = app
  .updater_builder()
  .pubkey("<your public key>")
  .build()?
  .check()
  .await?;

----------------------------------------

TITLE: Initializing Single Instance Plugin with Deep Link
DESCRIPTION: Rust code to initialize the single instance plugin with deep link integration.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
          println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");
          // when defining deep link schemes at runtime, you must also check `argv` here
        }));
    }

    builder = builder.plugin(tauri_plugin_deep_link::init());
}

----------------------------------------

TITLE: Using Autostart Plugin in JavaScript
DESCRIPTION: Example showing how to enable, check, and disable autostart functionality in JavaScript using the Tauri autostart plugin.

LANGUAGE: javascript
CODE:
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
// when using `"withGlobalTauri": true`, you may use
// const { enable, isEnabled, disable } = window.__TAURI__.autostart;

// Enable autostart
await enable();
// Check enable state
console.log(`registered for autostart? ${await isEnabled()}`);
// Disable autostart
disable();

----------------------------------------

TITLE: Enabling Unsafe Headers in Cargo.toml
DESCRIPTION: Toml configuration to enable the unsafe-headers feature flag in the HTTP plugin, allowing the use of normally forbidden request headers.

LANGUAGE: toml
CODE:
[dependencies]
tauri-plugin-http = { version = "2", features = ["unsafe-headers"] }

----------------------------------------

TITLE: Using SubmenuBuilder in Tauri 2.0
DESCRIPTION: Demonstrates how to create submenus with the new SubmenuBuilder API in Tauri 2.0, which replaces the previous Submenu API.

LANGUAGE: rust
CODE:
use tauri::menu::{MenuBuilder, SubmenuBuilder};

tauri::Builder::default()
    .setup(|app| {
        let submenu = SubmenuBuilder::new(app, "Sub")
            .text("Tauri")
            .separator()
            .check("Is Awesome")
            .build()?;
        let menu = MenuBuilder::new(app).item(&submenu).build()?;
        Ok(())
    })

----------------------------------------

TITLE: Configuring Offline Installer for Microsoft Store
DESCRIPTION: JSON configuration that sets the Windows installer to use the offline installer Webview2 installation option, which is required for Microsoft Store distribution.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "webviewInstallMode": {
        "type": "offlineInstaller"
      }
    }
  }
}

----------------------------------------

TITLE: Setting Custom Target for Updates in JavaScript for Tauri
DESCRIPTION: This JavaScript example shows how to set a custom target for updates, which is useful when distributing specialized builds like Universal macOS binaries or different build flavors.

LANGUAGE: javascript
CODE:
import { check } from '@tauri-apps/plugin-updater';

const update = await check({
  target: 'macos-universal',
});

----------------------------------------

TITLE: Recursively Removing Directories with Tauri FS Plugin
DESCRIPTION: Removes a directory and all its contents recursively. This is necessary when deleting non-empty directories, and should be used with caution.

LANGUAGE: javascript
CODE:
import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';
await remove('images', {
  baseDir: BaseDirectory.AppLocalData,
  recursive: true,
});

----------------------------------------

TITLE: Installing Process Plugin with Shell Command
DESCRIPTION: Command to add the process plugin to a Tauri project's dependencies in Cargo.toml using the cargo command.

LANGUAGE: shell
CODE:
cargo add tauri-plugin-process

----------------------------------------

TITLE: Setting Environment Variables for Update Signing on Windows
DESCRIPTION: PowerShell commands to set environment variables for the Tauri signing process on Windows. These variables define the private key path or content and optional password.

LANGUAGE: powershell
CODE:
$env:TAURI_SIGNING_PRIVATE_KEY="Path or content of your private key"
<# optionally also add a password #>
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""

----------------------------------------

TITLE: Installing Tauri CLI with cargo-binstall
DESCRIPTION: Commands to install the Tauri CLI using cargo-binstall, a tool that downloads and installs pre-built Rust binaries. The example shows installation and running a Tauri command.

LANGUAGE: bash
CODE:
$ cargo install cargo-binstall
$ cargo binstall tauri-cli
$ cargo tauri dev # run any Tauri command!

----------------------------------------

TITLE: Registering Deep Links in Tauri Application Setup
DESCRIPTION: Rust code for initializing and registering deep links in a Tauri application. It demonstrates how to use the tauri_plugin_deep_link plugin and register all configured schemes on Windows and Linux platforms.

LANGUAGE: rust
CODE:
use tauri_plugin_deep_link::DeepLinkExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            #[cfg(any(windows, target_os = "linux"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                app.deep_link().register_all()?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Visualizing Tauri Architecture with D2 Diagram
DESCRIPTION: A sketch diagram illustrating the relationships between core Tauri components and upstream dependencies. It shows how tauri-runtime connects to tauri-runtime-wry, which then connects to WRY, and how WRY connects to TAO.

LANGUAGE: d2
CODE:
direction: up

Core: {
  shape: rectangle
  "tauri": {
    "tauri-runtime"
    "tauri-macros"
    "tauri-utils"
  }

  "tauri-build"
  "tauri-codegen"
  "tauri-runtime-wry"
}

Upstream: {
  shape: rectangle
  direction: right
  WRY
  TAO
}

Core."tauri"."tauri-runtime" -> Core."tauri-runtime-wry"{style.animated: true}

Upstream.WRY -> Upstream.TAO{style.animated: true}
Core."tauri-runtime-wry" -> Upstream.Wry {style.animated: true}

----------------------------------------

TITLE: Initializing Stronghold with custom password hash function
DESCRIPTION: Example showing how to provide a custom password hashing implementation using the rust-argon2 crate. The hash function must return exactly 32 bytes to meet Stronghold requirements.

LANGUAGE: rust
CODE:
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_stronghold::Builder::new(|password| {
                // Hash the password here with e.g. argon2, blake2b or any other secure algorithm
                // Here is an example implementation using the `rust-argon2` crate for hashing the password
                use argon2::{hash_raw, Config, Variant, Version};

                let config = Config {
                    lanes: 4,
                    mem_cost: 10_000,
                    time_cost: 10,
                    variant: Variant::Argon2id,
                    version: Version::Version13,
                    ..Default::default()
                };
                let salt = "your-salt".as_bytes();
                let key = hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

                key.to_vec()
            })
            .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Using File System Plugin in JavaScript
DESCRIPTION: Shows how to use the File System plugin in JavaScript to create directories in application data locations.

LANGUAGE: javascript
CODE:
import { mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';
await mkdir('db', { baseDir: BaseDirectory.AppLocalData });

----------------------------------------

TITLE: Enabling Detailed Backtraces for Rust Errors in Tauri (Windows PowerShell)
DESCRIPTION: Commands for enabling detailed Rust backtraces when debugging Tauri applications on Windows using PowerShell, providing more information about errors.

LANGUAGE: powershell
CODE:
$env:RUST_BACKTRACE=1
tauri dev

----------------------------------------

TITLE: Configuring Resource Files in tauri.conf.json with Map Notation
DESCRIPTION: Advanced configuration example for resources using the map object notation. This allows specifying both source files and their destination paths within the resources directory of the application bundle.

LANGUAGE: json
CODE:
{
  "bundle": {
    "resources": {
      "/absolute/path/to/textfile.txt": "resources/textfile.txt",
      "relative/path/to/jsonfile.json": "resources/jsonfile.json",
      "resources/**/*": "resources/"
    }
  }
}

----------------------------------------

TITLE: Configuring Update Artifacts in tauri.conf.json (v2)
DESCRIPTION: JSON configuration for enabling update artifacts creation in Tauri v2. This tells the bundler to create the necessary files for updates.

LANGUAGE: json
CODE:
{
  "bundle": {
    "createUpdaterArtifacts": true
  }
}

----------------------------------------

TITLE: Configuring Cargo Dependencies for Tauri Project
DESCRIPTION: A Cargo.toml file that configures the necessary dependencies for a Tauri application, including tauri-build for build-time operations and tauri with the custom-protocol feature. It also includes release profile optimizations.

LANGUAGE: toml
CODE:
[package]
name = "hello-tauri-webdriver"
version = "0.1.0"
edition = "2021"
rust-version = "1.56"

# Needed to set up some things for Tauri at build time
[build-dependencies]
tauri-build = "1"

# The actual Tauri dependency, along with `custom-protocol` to serve the pages.
[dependencies]
tauri = { version = "1", features = ["custom-protocol"] }

# Make --release build a binary that is small (opt-level = "s") and fast (lto = true).
# This is completely optional, but shows that testing the application as close to the
# typical release settings is possible. Note: this will slow down compilation.
[profile.release]
incremental = false
codegen-units = 1
panic = "abort"
opt-level = "s"
lto = true

----------------------------------------

TITLE: Initializing Tauri Updater Plugin in Rust
DESCRIPTION: Rust code modification for lib.rs to initialize the updater plugin in a Tauri application. The plugin is only initialized for desktop configurations.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_updater::Builder::new().build());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Initializing NFC Plugin in Rust
DESCRIPTION: Modification to the lib.rs file to initialize the NFC plugin in a Tauri application. The plugin is only initialized on mobile platforms.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(mobile)]
            app.handle().plugin(tauri_plugin_nfc::init());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring HTTP Headers in Vite for JavaScript/TypeScript Frameworks
DESCRIPTION: Adding HTTP headers to the Vite configuration file for JavaScript/TypeScript frameworks like Qwik, React, Solid, Svelte, and Vue. Required for development environments to emulate production settings.

LANGUAGE: typescript
CODE:
import { defineConfig } from 'vite';

export default defineConfig({
  // ...
  server: {
      // ...
      headers: {
        'Cross-Origin-Opener-Policy': 'same-origin',
        'Cross-Origin-Embedder-Policy': 'require-corp',
        'Timing-Allow-Origin': 'https://developer.mozilla.org, https://example.com',
        'Access-Control-Expose-Headers': 'Tauri-Custom-Header',
        'Tauri-Custom-Header': "key1 'value1' 'value2'; key2 'value3'"
      },
    },
})

----------------------------------------

TITLE: Basic Plugin Initialization with Callback Function
DESCRIPTION: Example of the basic structure for initializing the Single Instance plugin with a callback function that handles new instance attempts.

LANGUAGE: rust
CODE:
.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
  // Write your code here...
}))

----------------------------------------

TITLE: Visualizing Command Invocation Flow in Tauri Using D2
DESCRIPTION: A sequence diagram showing the complete command invocation lifecycle from the frontend to the backend, including the invoke handler processing and response serialization. Illustrates the request-response pattern used by Commands.

LANGUAGE: d2
CODE:
shape: sequence_diagram


Frontend: {
  label: "Webview\nFrontend"
}

Core: {
  label: "Core\nBackend"
}
InvokeHandler: {
  label: "Invoke\nHandler"
}

Frontend -> Core: "IPC Request"{style.animated: true}
Core -> InvokeHandler: "Invoke command"{style.animated: true}
InvokeHandler -> Core: "Serialize return"{style.animated: true}
Core -> Frontend: "Response"{style.animated: true}

----------------------------------------

TITLE: Using Store Plugin in Rust
DESCRIPTION: Example of how to use the Store plugin in Rust, showing how to create or load a store, set and get values, and manage the store resource.

LANGUAGE: rust
CODE:
use tauri::Wry;
use tauri_plugin_store::StoreExt;
use serde_json::json;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // Create a new store or load the existing one
            // this also put the store in the app's resource table
            // so your following calls `store` calls (from both rust and js)
            // will reuse the same store
            let store = app.store("store.json")?;

            // Note that values must be serde_json::Value instances,
            // otherwise, they will not be compatible with the JavaScript bindings.
            store.set("some-key", json!({ "value": 5 }));

            // Get a value from the store.
            let value = store.get("some-key").expect("Failed to get value from store");
            println!("{}", value); // {"value":5}

            // Remove the store from the resource table
            store.close_resource();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring Update Requests with JavaScript in Tauri
DESCRIPTION: This snippet shows how to configure custom request parameters when checking for updates in JavaScript. It demonstrates setting a proxy URL, request timeout, and custom headers for authentication.

LANGUAGE: javascript
CODE:
import { check } from '@tauri-apps/plugin-updater';

const update = await check({
  proxy: '<proxy url>',
  timeout: 30000 /* milliseconds */,
  headers: {
    Authorization: 'Bearer <token>',
  },
});

----------------------------------------

TITLE: Creating Link Cards for Feature Implementation Resources
DESCRIPTION: This code displays a grid of link cards for tutorials on implementing specific Tauri features, including splashscreen and Node.js sidecar integration.

LANGUAGE: markdown
CODE:
<CardGrid>
  <LinkCard title="Splashcreen" href="/learn/splashscreen/" />
  <LinkCard title="Node.js as a Sidecar" href="/learn/sidecar-nodejs/" />
</CardGrid>

----------------------------------------

TITLE: Creating New Files Only with Tauri FS Plugin
DESCRIPTION: Opens a file with the createNew option, which fails if the file already exists. This is useful when you want to ensure you're not overwriting an existing file.

LANGUAGE: javascript
CODE:
import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
const file = await open('foo/bar.txt', {
  write: true,
  createNew: true,
  baseDir: BaseDirectory.AppData,
});
await file.write(new TextEncoder().encode('world'));
await file.close();

----------------------------------------

TITLE: Installing Dependencies and Running the Project
DESCRIPTION: Commands to set up the initial project dependencies and validate the setup by running the application in development mode.

LANGUAGE: sh
CODE:
# Make sure you're in the right directory
cd splashscreen-lab
# Install dependencies
pnpm install
# Build and run the app
pnpm tauri dev

----------------------------------------

TITLE: Installing tauri-driver using Cargo
DESCRIPTION: Command to install or update tauri-driver, which is required for WebDriver testing in Tauri applications. The --locked flag ensures reproducible builds by using the exact dependency versions from Cargo.lock.

LANGUAGE: shell
CODE:
cargo install tauri-driver --locked

----------------------------------------

TITLE: Defining File Read Permission
DESCRIPTION: Example of a permission configuration that enables all file read-related commands without pre-configured accessible paths. This defines a command-based permission.

LANGUAGE: toml
CODE:
[[permission]]
identifier = "read-files"
description = """This enables all file read related
commands without any pre-configured accessible paths."""
commands.allow = [
    "read_file",
    "read",
    "open",
    "read_text_file",
    "read_text_file_lines",
    "read_text_file_lines_next"
]

----------------------------------------

TITLE: Configuring Named Arguments in Tauri CLI
DESCRIPTION: JSON configuration for named arguments in tauri.conf.json. It defines a 'type' argument that can take multiple values from a predefined set of possible values using both short and long form flags.

LANGUAGE: json
CODE:
{
  "args": [
    {
      "name": "type",
      "short": "t",
      "takesValue": true,
      "multiple": true,
      "possibleValues": ["foo", "bar"]
    }
  ]
}

----------------------------------------

TITLE: Configuring Subcommands in Tauri CLI
DESCRIPTION: JSON configuration for subcommands in tauri.conf.json. It defines two subcommands (branch and push) that can have their own arguments and configurations.

LANGUAGE: json
CODE:
{
  "cli": {
    ...
    "subcommands": {
      "branch": {
        "args": []
      },
      "push": {
        "args": []
      }
    }
  }
}

----------------------------------------

TITLE: Configuring Nuxt for Tauri Integration
DESCRIPTION: TypeScript configuration for Nuxt to work optimally with Tauri. Disables SSR, configures the development server for iOS device compatibility, and sets up Vite with Tauri-specific settings for environment variables and port handling.

LANGUAGE: typescript
CODE:
export default defineNuxtConfig({
  // (optional) Enable the Nuxt devtools
  devtools: { enabled: true },
  // Enable SSG
  ssr: false,
  // Enables the development server to be discoverable by other devices when running on iOS physical devices
  devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },
  vite: {
    // Better support for Tauri CLI output
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      // Tauri requires a consistent port
      strictPort: true,
    },
  },
});

----------------------------------------

TITLE: Importing Tauri Log Functions in JavaScript
DESCRIPTION: Importing the log plugin's JavaScript APIs for client-side logging.

LANGUAGE: javascript
CODE:
import {
  warn,
  debug,
  trace,
  info,
  error,
  attachConsole,
  attachLogger,
} from '@tauri-apps/plugin-log';
// when using `"withGlobalTauri": true`, you may use
// const { warn, debug, trace, info, error, attachConsole, attachLogger } = window.__TAURI__.log;

----------------------------------------

TITLE: Customizing Log Format
DESCRIPTION: Defining a custom format function to change how log entries are formatted.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .format(|out, message, record| {
    out.finish(format_args!(
      "[{} {}] {}",
      record.level(),
      record.target(),
      message
    ))
  })
  .build()

----------------------------------------

TITLE: Using Log Functions in JavaScript
DESCRIPTION: Example of using the log plugin's JavaScript APIs to create log entries at different severity levels.

LANGUAGE: javascript
CODE:
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

trace('Trace');
info('Info');
error('Error');

----------------------------------------

TITLE: Implementing Load Lifecycle Event in iOS
DESCRIPTION: Shows how to implement the load lifecycle event in an iOS Tauri plugin to perform initialization when the plugin is loaded into the web view.

LANGUAGE: swift
CODE:
class ExamplePlugin: Plugin {
  @objc public override func load(webview: WKWebView) {
    let timeout = self.config["timeout"] as? Int ?? 30
  }
}

----------------------------------------

TITLE: Configuring Deny Scopes for Webview Data Protection
DESCRIPTION: Example showing how to deny access to sensitive webview data on different platforms. It defines platform-specific deny scopes to prevent exposure of sensitive webview configuration data.

LANGUAGE: toml
CODE:
[[permission]]
identifier = "deny-webview-data-linux"
description = '''
This denies read access to the
`$APPLOCALDATA` folder on linux as the webview data and
configuration values are stored here.
Allowing access can lead to sensitive information disclosure and
should be well considered.
'''
platforms = ["linux"]

[[scope.deny]]
path = "$APPLOCALDATA/**"

[[permission]]
identifier = "deny-webview-data-windows"
description = '''
This denies read access to the
`$APPLOCALDATA/EBWebView` folder on windows as the webview data and
configuration values are stored here.
Allowing access can lead to sensitive information disclosure and
should be well considered.
'''
platforms = ["windows"]

[[scope.deny]]
path = "$APPLOCALDATA/EBWebView/**"

----------------------------------------

TITLE: Manual Setup for Tauri Project with Vite
DESCRIPTION: Commands to manually create a new directory for a Tauri project, initialize a Vite frontend, install Tauri CLI, and initialize the Tauri backend configuration.

LANGUAGE: sh
CODE:
✔ What is your app name? tauri-app
✔ What should the window title be? tauri-app
✔ Where are your web assets located? ..
✔ What is the url of your dev server? http://localhost:5173
✔ What is your frontend dev command? pnpm run dev
✔ What is your frontend build command? pnpm run build

----------------------------------------

TITLE: Initializing Notification Plugin in Rust
DESCRIPTION: Code snippet to initialize the notification plugin in the Tauri application's Rust file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Setting Per-Machine Installation Mode in Tauri
DESCRIPTION: Configuration to set the NSIS installer mode to 'perMachine', which installs the application system-wide rather than for the current user only. This requires administrator privileges and installs to Program Files instead of the user's local app data.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "nsis": {
        "installMode": "perMachine"
      }
    }
  }
}

----------------------------------------

TITLE: Configuring Tauri for Next.js Integration
DESCRIPTION: JSON configuration for src-tauri/tauri.conf.json that sets up the build commands and paths for a Next.js frontend with Tauri. Shows configuration for different package managers.

LANGUAGE: json
CODE:
// src-tauri/tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../out"
  }
}

----------------------------------------

TITLE: Configuring Tauri for Next.js Integration
DESCRIPTION: JSON configuration for src-tauri/tauri.conf.json that sets up the build commands and paths for a Next.js frontend with Tauri. Shows configuration for different package managers.

LANGUAGE: json
CODE:
// src-tauri/tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../out"
  }
}

----------------------------------------

TITLE: Setting Maximum Log Level
DESCRIPTION: Configuring the maximum log level to filter out less important logs globally.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .level(log::LevelFilter::Info)
  .build()

----------------------------------------

TITLE: Configuring Tauri with Qwik using Deno
DESCRIPTION: Tauri configuration for a Qwik project using Deno as the runtime. It defines the development URL, frontend distribution directory, and Deno-specific task commands to run before development and build processes.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "devUrl": "http://localhost:5173"
    "frontendDist": "../dist",
    "beforeDevCommand": "deno task dev",
    "beforeBuildCommand": "deno task build"
  }
}

----------------------------------------

TITLE: Configuring Tauri FS Plugin to Handle Dotfiles
DESCRIPTION: Configuration for the Tauri FS plugin to handle dotfiles on Unix systems by disabling the requirement for literal leading dots in paths.

LANGUAGE: json
CODE:
"plugins": {
    "fs": {
      "requireLiteralLeadingDot": false
    }
  }

----------------------------------------

TITLE: Making HTTP Requests with Rust in Tauri
DESCRIPTION: Example of using the reqwest crate re-exported by the HTTP plugin to make requests directly from Rust code.

LANGUAGE: rust
CODE:
use tauri_plugin_http::reqwest;

let res = reqwest::get("http://my.api.host/data.json").await;
println!("{:?}", res.status()); // e.g. 200
println!("{:?}", res.text().await); // e.g Ok("{ Content }")

----------------------------------------

TITLE: Positioning App and Applications Folder Icons in DMG Window
DESCRIPTION: Customizes the position of both the application icon and the Applications folder icon within the DMG window by specifying their x and y coordinates in tauri.conf.json.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "dmg": {
        "appPosition": {
          "x": 180,
          "y": 220
        },
        "applicationFolderPosition": {
          "x": 480,
          "y": 220
        }
      }
    }
  }
}

----------------------------------------

TITLE: Implementing Plugin Configuration in iOS
DESCRIPTION: Shows how to retrieve and use plugin configuration in an iOS Tauri plugin by defining a Config struct and parsing it during the plugin load lifecycle.

LANGUAGE: swift
CODE:
struct Config: Decodable {
  let timeout: Int?
}

class ExamplePlugin: Plugin {
  var timeout: Int? = 3000

  @objc public override func load(webview: WKWebView) {
    do {
      let config = try parseConfig(Config.self)
      self.timeout = config.timeout
    } catch {}
  }
}

----------------------------------------

TITLE: Configuring External Binaries in Tauri JSON Configuration
DESCRIPTION: A configuration snippet for the tauri.conf.json file that demonstrates how to specify external binaries (sidecars) to be bundled with your Tauri application.

LANGUAGE: json
CODE:
{
  "bundle": {
    "externalBin": [
      "/absolute/path/to/sidecar",
      "../relative/path/to/binary",
      "binaries/my-sidecar"
    ]
  }
}

----------------------------------------

TITLE: Triggering Deep Links on Windows via Terminal
DESCRIPTION: Shell command to trigger a deep link on Windows using the start command. This opens the application using the specified scheme and URL.

LANGUAGE: sh
CODE:
start <scheme>://url

----------------------------------------

TITLE: Configuring Tauri with yarn for SvelteKit Integration
DESCRIPTION: JSON configuration for Tauri when using yarn as the package manager. Defines the build commands and frontend distribution path for a SvelteKit project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}

----------------------------------------

TITLE: Triggering Deep Links on Linux via Terminal
DESCRIPTION: Shell command to trigger a deep link on Linux using the xdg-open command. This opens the application using the specified scheme and URL.

LANGUAGE: sh
CODE:
xdg-open <scheme>://url

----------------------------------------

TITLE: Adding log Crate Dependency in Cargo.toml
DESCRIPTION: Configuration to add the log crate as a dependency in Cargo.toml, which is required for Rust-side logging.

LANGUAGE: toml
CODE:
[dependencies]
log = "0.4"

----------------------------------------

TITLE: Handling Tray Icon Events in JavaScript
DESCRIPTION: Shows how to listen for various tray icon interaction events in JavaScript, including clicks, double clicks, mouse enter/leave, and movement. Events are identified using a switch statement.

LANGUAGE: javascript
CODE:
import { TrayIcon } from '@tauri-apps/api/tray';

const options = {
  action: (event) => {
    switch (event.type) {
      case 'Click':
        console.log(
          `mouse ${event.button} button pressed, state: ${event.buttonState}`
        );
        break;
      case 'DoubleClick':
        console.log(`mouse ${event.button} button pressed`);
        break;
      case 'Enter':
        console.log(
          `mouse hovered tray at ${event.rect.position.x}, ${event.rect.position.y}`
        );
        break;
      case 'Move':
        console.log(
          `mouse moved on tray at ${event.rect.position.x}, ${event.rect.position.y}`
        );
        break;
      case 'Leave':
        console.log(
          `mouse left tray at ${event.rect.position.x}, ${event.rect.position.y}`
        );
        break;
    }
  },
};

const tray = await TrayIcon.new(options);

----------------------------------------

TITLE: Configuring Android Permissions for File System Access
DESCRIPTION: XML configuration for Android permissions required to access external storage when using the file system plugin.

LANGUAGE: xml
CODE:
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE"/>
<uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />

----------------------------------------

TITLE: Implementing WebView Ready Lifecycle Hook in Tauri Plugin
DESCRIPTION: Shows how to implement the on_webview_ready lifecycle hook to execute initialization code when a new window has been created.

LANGUAGE: rust
CODE:
use tauri::plugin::Builder;

Builder::new("<plugin-name>")
  .on_webview_ready(|window| {
    window.listen("content-loaded", |event| {
      println!("webview content has been loaded");
    });
  })

----------------------------------------

TITLE: Visualizing Tauri's Process Model with D2 Diagram
DESCRIPTION: A diagram illustrating Tauri's process model architecture. It shows the Core process (represented as a diamond) connecting to multiple WebView processes through Events & Commands channels. The animated connections highlight the communication flow between processes.

LANGUAGE: d2
CODE:
direction: right

Core: {
  shape: diamond
}

"Events & Commands 1": {
  WebView1: WebView
}

"Events & Commands 2": {
  WebView2: WebView
}

"Events & Commands 3": {
  WebView3: WebView
}

Core -> "Events & Commands 1"{style.animated: true}
Core -> "Events & Commands 2"{style.animated: true}
Core -> "Events & Commands 3"{style.animated: true}

"Events & Commands 1" -> WebView1{style.animated: true}
"Events & Commands 2" -> WebView2{style.animated: true}
"Events & Commands 3" -> WebView3{style.animated: true}

----------------------------------------

TITLE: Configuring Tauri with pnpm for SvelteKit Integration
DESCRIPTION: JSON configuration for Tauri when using pnpm as the package manager. Sets up the necessary build commands and frontend distribution directory for a SvelteKit project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}

----------------------------------------

TITLE: Configuring Tauri with pnpm for SvelteKit Integration
DESCRIPTION: JSON configuration for Tauri when using pnpm as the package manager. Sets up the necessary build commands and frontend distribution directory for a SvelteKit project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}

----------------------------------------

TITLE: Running Sidecar from JavaScript with Tauri Shell Plugin
DESCRIPTION: JavaScript code example showing how to import and use the Command class from @tauri-apps/plugin-shell to execute a sidecar binary.

LANGUAGE: javascript
CODE:
import { Command } from '@tauri-apps/plugin-shell';
const command = Command.sidecar('binaries/my-sidecar');
const output = await command.execute();

----------------------------------------

TITLE: Getting OS Platform in Rust
DESCRIPTION: Example of using the platform function from the OS Information plugin in Rust to get the current operating system platform.

LANGUAGE: rust
CODE:
let platform = tauri_plugin_os::platform();
println!("Platform: {}", platform);
// Prints "windows" to the terminal

----------------------------------------

TITLE: Triggering App Links on iOS Simulator
DESCRIPTION: Shell command to trigger an app link on iOS simulator using the simctl CLI. This allows direct opening of a link from the terminal during development and testing.

LANGUAGE: sh
CODE:
xcrun simctl openurl booted https://<host>/path

----------------------------------------

TITLE: Tauri Configuration for Yarn with Vite
DESCRIPTION: Configuration in tauri.conf.json for a Tauri project using yarn with Vite, specifying build commands, development URL, and frontend distribution folder.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Tauri Configuration for Yarn with Vite
DESCRIPTION: Configuration in tauri.conf.json for a Tauri project using yarn with Vite, specifying build commands, development URL, and frontend distribution folder.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Configuring LogDir Target for Platform-Specific Log Storage
DESCRIPTION: Configuration to store logs in the platform's recommended log directory with a custom filename.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .target(tauri_plugin_log::Target::new(
    tauri_plugin_log::TargetKind::LogDir {
      file_name: Some("logs".to_string()),
    },
  ))
  .build()

----------------------------------------

TITLE: Creating a Tray Icon in Rust
DESCRIPTION: Shows how to create a new system tray icon using TrayIconBuilder in Rust within a Tauri application setup function.

LANGUAGE: rust
CODE:
use tauri::tray::TrayIconBuilder;

tauri::Builder::default()
.setup(|app| {
let tray = TrayIconBuilder::new().build(app)?;
Ok(())
})

----------------------------------------

TITLE: Emitting Webview-Specific Events in Tauri
DESCRIPTION: This code shows how to emit events to specific webviews using the emit_to function. It demonstrates a login command that sends authentication results to a particular webview.

LANGUAGE: rust
CODE:
use tauri::{AppHandle, Emitter};

#[tauri::command]
fn login(app: AppHandle, user: String, password: String) {
  let authenticated = user == "tauri-apps" && password == "tauri";
  let result = if authenticated { "loggedIn" } else { "invalidCredentials" };
  app.emit_to("login", "login-result", result).unwrap();
}

----------------------------------------

TITLE: Emitting Webview-Specific Events in Tauri
DESCRIPTION: This code shows how to emit events to specific webviews using the emit_to function. It demonstrates a login command that sends authentication results to a particular webview.

LANGUAGE: rust
CODE:
use tauri::{AppHandle, Emitter};

#[tauri::command]
fn login(app: AppHandle, user: String, password: String) {
  let authenticated = user == "tauri-apps" && password == "tauri";
  let result = if authenticated { "loggedIn" } else { "invalidCredentials" };
  app.emit_to("login", "login-result", result).unwrap();
}

----------------------------------------

TITLE: Configuring Autostart Plugin Permissions in Tauri
DESCRIPTION: JSON configuration for the capabilities file to allow the autostart plugin's functionality, including enabling, disabling, and checking autostart status.

LANGUAGE: json
CODE:
{
  "permissions": [
    ...,
    "autostart:allow-enable",
    "autostart:allow-disable",
    "autostart:allow-is-enabled"
  ]
}

----------------------------------------

TITLE: Configuring Upload Plugin Permissions
DESCRIPTION: JSON configuration to set the necessary permissions for the Upload plugin in the capabilities file.

LANGUAGE: json
CODE:
{
  "permissions": [
    ...,
    "upload:default",
  ]
}

----------------------------------------

TITLE: Implementing onNewIntent Lifecycle Event in Android
DESCRIPTION: Shows how to implement the onNewIntent lifecycle event in an Android Tauri plugin to handle the activity being re-launched, such as when a notification is clicked.

LANGUAGE: kotlin
CODE:
import android.app.Activity
import android.content.Intent
import app.tauri.annotation.TauriPlugin

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
  override fun onNewIntent(intent: Intent) {
    // handle new intent event
  }
}

----------------------------------------

TITLE: Using Process Plugin in JavaScript
DESCRIPTION: JavaScript example showing how to use the process plugin to exit the app with a status code and relaunch the application.

LANGUAGE: javascript
CODE:
import { exit, relaunch } from '@tauri-apps/plugin-process';
// when using `"withGlobalTauri": true`, you may use
// const { exit, relaunch } = window.__TAURI__.process;

// exits the app with the given status code
await exit(0);

// restarts the app
await relaunch();

----------------------------------------

TITLE: Checking NFC Availability in Rust
DESCRIPTION: Rust code to check if the device supports NFC functionality using the NfcExt trait from the Tauri NFC plugin.

LANGUAGE: rust
CODE:
tauri::Builder::default()
  .setup(|app| {
    #[cfg(mobile)]
    {
      use tauri_plugin_nfc::NfcExt;

      app.handle().plugin(tauri_plugin_nfc::init());

      let can_scan_nfc = app.nfc().is_available()?;
    }
    Ok(())
  })

----------------------------------------

TITLE: Tauri Configuration for PNPM with Vite
DESCRIPTION: Configuration in tauri.conf.json for a Tauri project using pnpm with Vite, specifying build commands, development URL, and frontend distribution folder.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Tauri Configuration for PNPM with Vite
DESCRIPTION: Configuration in tauri.conf.json for a Tauri project using pnpm with Vite, specifying build commands, development URL, and frontend distribution folder.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Configuring Permissions for Barcode Scanner Plugin
DESCRIPTION: JSON configuration for mobile capabilities that defines the necessary permissions to allow scanning and canceling operations for the barcode scanner plugin.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/mobile-schema.json",
  "identifier": "mobile-capability",
  "windows": ["main"],
  "platforms": ["iOS", "android"],
  "permissions": ["barcode-scanner:allow-scan", "barcode-scanner:allow-cancel"]
}

----------------------------------------

TITLE: Prompting Biometric Authentication in Rust
DESCRIPTION: Rust code to prompt the user for biometric authentication with customizable options, handling both success and failure cases.

LANGUAGE: rust
CODE:
use tauri_plugin_biometric::{BiometricExt, AuthOptions};

fn bio_auth(app_handle: tauri::AppHandle) {

    let options = AuthOptions {
        // Set True if you want the user to be able to authenticate using phone password
        allow_device_credential:false,
        cancel_title: Some("Feature won't work if Canceled".to_string()),

        // iOS only feature
        fallback_title: Some("Sorry, authentication failed".to_string()),

        // Android only features
        title: Some("Tauri feature".to_string()),
        subtitle: Some("Authenticate to access the locked Tauri function".to_string()),
        confirmation_required: Some(true),
    };

    // if the authentication was successful, the function returns Result::Ok()
    // otherwise returns Result::Error()
    match app_handle.biometric().authenticate("This feature is locked".to_string(), options) {
        Ok(_) => {
            println!("Hooray! Successfully Authenticated! We can now perform the locked Tauri function!");
        }
        Err(e) => {
            println!("Oh no! Authentication failed because : {e}");
        }
    }
}

----------------------------------------

TITLE: Rust Code for Transparent Titlebar with Custom Background on macOS
DESCRIPTION: Rust implementation to create a window with a transparent titlebar and custom background color on macOS, using both Tauri and Cocoa APIs.

LANGUAGE: rust
CODE:
use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

pub fn run() {
	tauri::Builder::default()
		.setup(|app| {
			let win_builder =
				WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
					.title("Transparent Titlebar Window")
					.inner_size(800.0, 600.0);

			// set transparent title bar only when building for macOS
			#[cfg(target_os = "macos")]
			let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

			let window = win_builder.build().unwrap();

			// set background color only when building for macOS
			#[cfg(target_os = "macos")]
			{
				use cocoa::appkit::{NSColor, NSWindow};
				use cocoa::base::{id, nil};

				let ns_window = window.ns_window().unwrap() as id;
				unsafe {
					let bg_color = NSColor::colorWithRed_green_blue_alpha_(
							nil,
							50.0 / 255.0,
							158.0 / 255.0,
							163.5 / 255.0,
							1.0,
					);
					ns_window.setBackgroundColor_(bg_color);
				}
			}

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

----------------------------------------

TITLE: Emitting Events from iOS Tauri Plugin
DESCRIPTION: Demonstrates how to emit events from an iOS Tauri plugin. This example shows triggering events during the plugin lifecycle and in response to command execution.

LANGUAGE: swift
CODE:
class ExamplePlugin: Plugin {
  @objc public override func load(webview: WKWebView) {
    trigger("load", data: [:])
  }

  @objc public func openCamera(_ invoke: Invoke) {
    trigger("camera", data: ["open": true])
  }
}

----------------------------------------

TITLE: Getting OS Platform in JavaScript
DESCRIPTION: Example of using the platform function from the OS Information plugin in JavaScript to get the current operating system platform.

LANGUAGE: javascript
CODE:
import { platform } from '@tauri-apps/plugin-os';
// when using `"withGlobalTauri": true`, you may use
// const { platform } = window.__TAURI__.os;

const currentPlatform = platform();
console.log(currentPlatform);
// Prints "windows" to the console

----------------------------------------

TITLE: Configuring CLI Plugin Permissions in Tauri
DESCRIPTION: JSON configuration for granting CLI plugin permissions in the capabilities file. It adds the 'cli:default' permission to the main capability to allow the application to use the CLI plugin.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["cli:default"]
}

----------------------------------------

TITLE: Defining Permissions for Android Tauri Plugin
DESCRIPTION: Shows how to define required permissions in an Android Tauri plugin using the TauriPlugin annotation. This example demonstrates requesting notification permissions by defining them in the plugin class.

LANGUAGE: kotlin
CODE:
@TauriPlugin(
  permissions = [
    Permission(strings = [Manifest.permission.POST_NOTIFICATIONS], alias = "postNotification")
  ]
)
class ExamplePlugin(private val activity: Activity): Plugin(activity) { }

----------------------------------------

TITLE: Configuring Cargo manifest for Mobile Support in Tauri 2.0
DESCRIPTION: Adds library configuration to the Cargo.toml file to produce the required shared library artifacts for mobile support in Tauri 2.0, including static libraries, C dynamic libraries, and Rust libraries.

LANGUAGE: toml
CODE:
// src-tauri/Cargo.toml
[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

----------------------------------------

TITLE: Initializing SQL Plugin in Rust
DESCRIPTION: Code snippet showing how to initialize the SQL plugin in the Tauri application's lib.rs file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Defining Scope Data Structure in Rust
DESCRIPTION: This code defines a Rust struct that will hold scope data for plugin permissions, in this example for a list of binaries a shell plugin is allowed to spawn.

LANGUAGE: rust
CODE:
#[derive(Debug, schemars::JsonSchema)]
pub struct Entry {
    pub binary: String,
}

----------------------------------------

TITLE: Configuring Process Plugin Permissions
DESCRIPTION: JSON configuration for enabling the process plugin permissions in the Tauri capabilities system.

LANGUAGE: json
CODE:
{
  "permissions": [
    ...,
    "process:default",
  ]
}

----------------------------------------

TITLE: Vite Configuration for Development Server in Tauri 2.0 Beta
DESCRIPTION: JavaScript configuration for Vite development server in Tauri 2.0 beta, using platform detection for mobile environments and the internal-ip package for network address resolution.

LANGUAGE: js
CODE:
import { defineConfig } from 'vite';\nimport { svelte } from '@sveltejs/vite-plugin-svelte';\nimport { internalIpV4Sync } from 'internal-ip';\n\nconst mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);\n\nexport default defineConfig({\n  plugins: [svelte()],\n  clearScreen: false,\n  server: {\n    host: mobile ? '0.0.0.0' : false,\n    port: 1420,\n    strictPort: true,\n    hmr: mobile\n      ? {\n          protocol: 'ws',\n          host: internalIpV4Sync(),\n          port: 1421,\n        }\n      : undefined,\n  },\n});

----------------------------------------

TITLE: Vite Configuration for Development Server in Tauri 2.0 Beta
DESCRIPTION: JavaScript configuration for Vite development server in Tauri 2.0 beta, using platform detection for mobile environments and the internal-ip package for network address resolution.

LANGUAGE: js
CODE:
import { defineConfig } from 'vite';\nimport { svelte } from '@sveltejs/vite-plugin-svelte';\nimport { internalIpV4Sync } from 'internal-ip';\n\nconst mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);\n\nexport default defineConfig({\n  plugins: [svelte()],\n  clearScreen: false,\n  server: {\n    host: mobile ? '0.0.0.0' : false,\n    port: 1420,\n    strictPort: true,\n    hmr: mobile\n      ? {\n          protocol: 'ws',\n          host: internalIpV4Sync(),\n          port: 1421,\n        }\n      : undefined,\n  },\n});

----------------------------------------

TITLE: Configuring NFC Permissions in Tauri Capabilities File
DESCRIPTION: This snippet demonstrates how to add NFC permissions to the Tauri capabilities configuration file. The highlighted line shows the specific permission entry needed to enable NFC functionality in a Tauri application.

LANGUAGE: json
CODE:
{
  "permissions": [
    ...,
    "nfc:default",
  ]
}

----------------------------------------

TITLE: Creating a Node.js Script to Rename Sidecar Binary Files
DESCRIPTION: A Node.js script that renames the compiled binary to follow Tauri's naming convention for sidecars. It detects the platform's target triple using Rust and places the renamed binary in the correct Tauri directory.

LANGUAGE: javascript
CODE:
import { execSync } from 'child_process';
import fs from 'fs';

const ext = process.platform === 'win32' ? '.exe' : '';

const rustInfo = execSync('rustc -vV');
const targetTriple = /host: (\S+)/g.exec(rustInfo)[1];
if (!targetTriple) {
  console.error('Failed to determine platform target triple');
}
fs.renameSync(
  `app${ext}`,
  `../src-tauri/binaries/app-${targetTriple}${ext}`
);

----------------------------------------

TITLE: Adding File System Plugin to Cargo Dependencies in Tauri
DESCRIPTION: Adds the tauri-plugin-fs dependency to a Cargo.toml file for enabling file system operations in a Tauri application.

LANGUAGE: toml
CODE:
[dependencies]
tauri-plugin-fs = "2"

----------------------------------------

TITLE: Checking NFC Availability in JavaScript
DESCRIPTION: JavaScript code to check if the device supports NFC functionality using the isAvailable function from the NFC plugin.

LANGUAGE: javascript
CODE:
import { isAvailable } from '@tauri-apps/plugin-nfc';

const canScanNfc = await isAvailable();

----------------------------------------

TITLE: Installing Tauri DevTools Plugin via Cargo
DESCRIPTION: Command to add the tauri-plugin-devtools crate to your Rust project using cargo. This installs version 2.0.0 of the plugin which is required for the CrabNebula DevTools functionality.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-devtools@2.0.0

----------------------------------------

TITLE: Configuring HTTP Permissions in Tauri
DESCRIPTION: JSON configuration for setting up allowed and denied URLs for HTTP requests in the capabilities file.

LANGUAGE: json
CODE:
//src-tauri/capabilities/default.json
{
  "permissions": [
    {
      "identifier": "http:default",
      "allow": [{ "url": "https://*.tauri.app" }],
      "deny": [{ "url": "https://private.tauri.app" }]
    }
  ]
}

----------------------------------------

TITLE: Rust Migration from v1/v2 to v3
DESCRIPTION: Code diff showing how to migrate Rust code from Store plugin v1 and v2 beta/rc to v3.

LANGUAGE: diff
CODE:
- with_store(app.handle().clone(), stores, path, |store| {
-     store.insert("some-key".to_string(), json!({ "value": 5 }))?;
-     Ok(())
- });
+ let store = app.store(path)?;
+ store.set("some-key".to_string(), json!({ "value": 5 }));

----------------------------------------

TITLE: Configuring SQL Plugin Permissions in Tauri
DESCRIPTION: JSON configuration that shows how to enable SQL plugin permissions in a Tauri application's capabilities.

LANGUAGE: json
CODE:
{
  "permissions": [
    ...,
    "sql:default",
    "sql:allow-execute",
  ]
}

----------------------------------------

TITLE: Setting JAVA_HOME Environment Variable for Android Development on Linux
DESCRIPTION: Command to set the JAVA_HOME environment variable on Linux for Android development with Tauri, pointing to the Java installation from Android Studio.

LANGUAGE: sh
CODE:
export JAVA_HOME=/opt/android-studio/jbr

----------------------------------------

TITLE: Creating Base-Level Window Menu in JavaScript
DESCRIPTION: Creates a basic window menu with a quit item using the Tauri Menu API in JavaScript. The menu is then set as the application's main menu.

LANGUAGE: javascript
CODE:
import { Menu } from '@tauri-apps/api/menu';

const menu = await Menu.new({
  items: [
    {
      id: 'quit',
      text: 'Quit',
      action: () => {
        console.log('quit pressed');
      },
    },
  ],
});

// If a window was not created with an explicit menu or had one set explicitly,
// this menu will be assigned to it.
menu.setAsAppMenu().then((res) => {
  console.log('menu set success', res);
});

----------------------------------------

TITLE: Installing NFC Plugin with Cargo
DESCRIPTION: Command to add the NFC plugin to the project's dependencies in Cargo.toml, specifically targeting Android and iOS platforms.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-nfc --target 'cfg(any(target_os = "android", target_os = "ios"))'

----------------------------------------

TITLE: Using Global Shortcut Plugin in Rust with Tauri
DESCRIPTION: Example of implementing global shortcuts in Rust, showing how to register shortcuts and handle events.

LANGUAGE: rust
CODE:
use tauri_plugin_global_shortcut::GlobalShortcutExt;

tauri::Builder::default()
    .plugin(
        tauri_plugin_global_shortcut::Builder::new().with_handler(|app, shortcut| {
            println!("Shortcut triggered: {:?}", shortcut);
        })
        .build(),
    )
    .setup(|app| {
        // registrar un atajo global
        // en macOS, se usa la tecla Cmd
        // en Windows y Linux, se usa la tecla Ctrl
        app.global_shortcut().register("CmdOrCtrl+Y")?;
        Ok(())
    })

----------------------------------------

TITLE: Using File System Plugin in JavaScript with Tauri
DESCRIPTION: Example of using the file system plugin in JavaScript to create a directory in the application's local data directory.

LANGUAGE: javascript
CODE:
import { mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';
await mkdir('db', { baseDir: BaseDirectory.AppLocalData });

----------------------------------------

TITLE: Using Clipboard Plugin in Rust
DESCRIPTION: Demonstrates how to use the Clipboard Manager plugin in Rust to write clipboard content.

LANGUAGE: rust
CODE:
use tauri_plugin_clipboard::{ClipboardExt, ClipKind};
tauri::Builder::default()
    .plugin(tauri_plugin_clipboard::init())
    .setup(|app| {
        app.clipboard().write(ClipKind::PlainText {
            label: None,
            text: "Tauri is awesome!".into(),
        })?;
        Ok(())
    })

----------------------------------------

TITLE: Reading Text Files with Tauri FS Plugin
DESCRIPTION: Demonstrates how to read a text file directly using the readTextFile function. This provides a simpler API for reading text content compared to the open/read/close pattern.

LANGUAGE: javascript
CODE:
import { readTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
const configToml = await readTextFile('config.toml', {
  baseDir: BaseDirectory.AppConfig,
});

----------------------------------------

TITLE: Configuring Flag Arguments in Tauri CLI
DESCRIPTION: JSON configuration for flag arguments in tauri.conf.json. It defines a 'verbose' flag with a short form 'v' that can be used multiple times to increase verbosity levels.

LANGUAGE: json
CODE:
{
  "args": [
    {
      "name": "verbose",
      "short": "v"
    }
  ]
}

----------------------------------------

TITLE: Setting Android Environment Variables on Linux
DESCRIPTION: Commands to set ANDROID_HOME and NDK_HOME environment variables on Linux, which are required for Android development with Tauri.

LANGUAGE: sh
CODE:
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

----------------------------------------

TITLE: Saving window state in Rust
DESCRIPTION: Rust code example showing how to manually save the state of all open windows to disk using the save_window_state method exposed by the AppHandleExt trait.

LANGUAGE: rust
CODE:
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

// `tauri::AppHandle` now has the following additional method
app.save_window_state(StateFlags::all()); // will save the state of all open windows to disk

----------------------------------------

TITLE: Configuring Tauri for Leptos Integration
DESCRIPTION: JSON configuration for src-tauri/tauri.conf.json that sets up the Tauri environment to work with Leptos. It specifies build commands, development URL, and enables the global Tauri object.

LANGUAGE: json
CODE:
// src-tauri/tauri.conf.json
{
  "build": {
    "beforeDevCommand": "trunk serve",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "trunk build",
    "frontendDist": "../dist"
  },
  "app": {
    "withGlobalTauri": true
  }
}

----------------------------------------

TITLE: Initializing Autostart Plugin in Rust
DESCRIPTION: Code snippet showing how to initialize the autostart plugin in a Tauri application's lib.rs file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"]) /* arbitrary number of args to pass to your app */));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Using Positioner Plugin in JavaScript
DESCRIPTION: JavaScript example showing how to use the positioner plugin to move a window to a predefined position.

LANGUAGE: javascript
CODE:
import { moveWindow, Position } from '@tauri-apps/plugin-positioner';
// when using `"withGlobalTauri": true`, you may use
// const { moveWindow, Position } = window.__TAURI__.positioner;

moveWindow(Position.TopRight);

----------------------------------------

TITLE: Using HTTP Plugin in Rust with Tauri
DESCRIPTION: Example of making HTTP requests in Rust using the HTTP plugin which re-exports reqwest.

LANGUAGE: rust
CODE:
use tauri_plugin_http::reqwest;

tauri::Builder::default()
    .plugin(tauri_plugin_http::init())
    .setup(|app| {
        let response_data = tauri::async_runtime::block_on(async {
            let response = reqwest::get(
                "https://raw.githubusercontent.com/tauri-apps/tauri/dev/package.json",
            )
            .await
            .unwrap();
            response.text().await
        })?;
        Ok(())
    })

----------------------------------------

TITLE: Configuring Positioner Plugin Permissions
DESCRIPTION: JSON configuration for adding the required permissions for the positioner plugin in the capabilities file.

LANGUAGE: json
CODE:
{
  "permissions": [
    ...,
    "positioner:default",
  ]
}

----------------------------------------

TITLE: Handling Tray Icon Events in Rust
DESCRIPTION: Demonstrates how to handle tray icon events in Rust using pattern matching on TrayIconEvent. This example shows the main window when the tray icon is clicked.

LANGUAGE: rust
CODE:
use tauri:{
    Manager,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}
};

TrayIconBuilder::new()
  .on_tray_icon_event(|tray, event| match event {
    TrayIconEvent::Click {
      button: MouseButton::Left,
      button_state: MouseButtonState::Up,
      ..
    } => {
      println!("left click pressed and released");
      // in this example, let's show and focus the main window when the tray is clicked
      let app = tray.app_handle();
      if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
      }
    }
    _ => {
      println!("unhandled event {event:?}");
    }
  })

----------------------------------------

TITLE: Emitting Global Events in Tauri
DESCRIPTION: Example of emitting global events in a Tauri application using the event.emit and WebviewWindow#emit functions. Global events are delivered to all listeners.

LANGUAGE: javascript
CODE:
import { emit } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

// emit(eventName, payload)
emit('file-selected', '/path/to/file');

const appWebview = getCurrentWebviewWindow();
appWebview.emit('route-changed', { url: window.location.href });

----------------------------------------

TITLE: Initializing CLI Plugin in Rust
DESCRIPTION: Modification to the lib.rs file to initialize the CLI plugin in a Tauri application. It adds the plugin initialization to the app setup function with a desktop-only configuration.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_cli::init());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Initializing Upload Plugin in Rust
DESCRIPTION: Code snippet showing how to initialize the Upload plugin in the Rust part of a Tauri application by modifying the lib.rs file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_upload::init())
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

----------------------------------------

TITLE: Initializing Barcode Scanner Plugin in Rust
DESCRIPTION: Code snippet showing how to modify the lib.rs file to initialize the barcode scanner plugin in a Tauri application. The plugin is conditionally initialized only for mobile targets.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(mobile)]
            app.handle().plugin(tauri_plugin_barcode_scanner::init());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Setting App Category in Tauri Configuration
DESCRIPTION: JSON configuration snippet that sets the application category for the App Store. This is a required field for apps to be properly categorized in the Apple App Store.

LANGUAGE: json
CODE:
{
  "bundle": {
    "category": "Utility"
  }
}

----------------------------------------

TITLE: Initializing Biometric Plugin in Tauri
DESCRIPTION: Rust code for initializing the biometric plugin in the Tauri application setup process. The plugin is only initialized when building for mobile platforms.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(mobile)]
            app.handle().plugin(tauri_plugin_biometric::Builder::new().build());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Installing Stronghold plugin with package manager
DESCRIPTION: Command-line instructions for adding the Stronghold plugin to a Tauri project using various package managers.

LANGUAGE: bash
CODE:
cargo add tauri-plugin-stronghold

----------------------------------------

TITLE: Initializing HTTP Plugin in Rust for Tauri
DESCRIPTION: Initializes the HTTP plugin in a Tauri Rust application builder.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
}

----------------------------------------

TITLE: Configuring Biometric Permissions in Capabilities JSON
DESCRIPTION: JSON configuration for enabling biometric plugin permissions in a Tauri application's capabilities configuration.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["biometric:default"]
}

----------------------------------------

TITLE: Creating a Notification Channel in JavaScript
DESCRIPTION: Code to create a notification channel with specific behaviors and properties for organizing notifications.

LANGUAGE: javascript
CODE:
import {
  createChannel,
  Importance,
  Visibility,
} from '@tauri-apps/plugin-notification';

await createChannel({
  id: 'messages',
  name: 'Messages',
  description: 'Notifications for new messages',
  importance: Importance.High,
  visibility: Visibility.Private,
  lights: true,
  lightColor: '#ff0000',
  vibration: true,
  sound: 'notification_sound',
});

----------------------------------------

TITLE: Custom Error Type with Serialization for Tauri Commands
DESCRIPTION: Creates a custom error type with thiserror and implements serde::Serialize to handle errors more idiomatically. This approach makes possible errors explicit and provides better control over error serialization.

LANGUAGE: rust
CODE:
// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		serializer.serialize_str(self.to_string().as_ref())
	}
}

#[tauri::command]
fn my_custom_command() -> Result<(), Error> {
	// This will return an error
	std::fs::File::open("path/that/does/not/exist")?;
	// Return `null` on success
	Ok(())
}

----------------------------------------

TITLE: Configuring Windows Install Mode in Tauri Updater Plugin
DESCRIPTION: Configuration for setting the Windows installation mode in the Tauri updater plugin. Supports 'passive', 'basicUi', or 'quiet' modes to control the update installation experience.

LANGUAGE: json
CODE:
{
  "plugins": {
    "updater": {
      "windows": {
        "installMode": "passive"
      }
    }
  }
}

----------------------------------------

TITLE: Building iOS App with Tauri CLI
DESCRIPTION: Command to build an iOS app using the Tauri CLI with the app-store-connect export method. This generates an IPA file that can be uploaded to the App Store.

LANGUAGE: shell
CODE:
npm run tauri ios build -- --export-method app-store-connect

----------------------------------------

TITLE: Checking Biometric Authentication Status in Rust
DESCRIPTION: Rust code to check the availability of biometric authentication on the device using the Tauri application handle.

LANGUAGE: rust
CODE:
use tauri_plugin_biometric::BiometricExt;

fn check_biometric(app_handle: tauri::AppHandle) {
    let status = app_handle.biometric().status().unwrap();
    if status.is_available {
        println!("Yes! Biometric Authentication is available");
    } else {
        println!("No! Biometric Authentication is not available due to: {}", status.error.unwrap());
    }
}

----------------------------------------

TITLE: Configuring Fixed WebView2 Runtime in Tauri
DESCRIPTION: Configuration for using a fixed WebView2 runtime version in a Tauri Windows application. This approach allows control over the WebView2 distribution but increases installer size by about 180MB.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "webviewInstallMode": {
        "type": "fixedRuntime",
        "path": "./Microsoft.WebView2.FixedVersionRuntime.98.0.1108.50.x64/"
      }
    }
  }
}

----------------------------------------

TITLE: Initializing Clipboard Plugin in Rust
DESCRIPTION: Code snippet showing how to initialize the clipboard plugin in the Tauri application's lib.rs file. It adds the plugin to the Tauri builder chain.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: PKGBUILD for Building Tauri App from Source
DESCRIPTION: A PKGBUILD file for building a Tauri application from source code. Includes git version handling, build dependencies, build process, and package creation steps for Arch Linux.

LANGUAGE: ini
CODE:
# Maintainer:
pkgname=<pkgname>-git
pkgver=<pkgver>
pkgrel=1
pkgdesc="Description of your app"
arch=('x86_64' 'aarch64')
url="https://github.com/<user>/<project>"
license=('MIT')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')
makedepends=('git' 'openssl' 'appmenu-gtk-module' 'libappindicator-gtk3' 'librsvg' 'cargo' 'pnpm' 'nodejs')
provides=('<pkgname>')
conflicts=('<binname>' '<pkgname>')
source=("git+${url}.git")
sha256sums=('SKIP')

pkgver() {
	cd <project>
	( set -o pipefail
	  git describe --long --abbrev=7 2>/dev/null | sed 's/\([^-]*-g\)/r\1/;s/-/./g' ||
	  printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
	)
}

prepare() {
	cd <project>
	pnpm install
}

build() {
	cd <project>
	pnpm tauri build -b deb
}

package() {
	cp -a <project>/src-tauri/target/release/bundle/deb/<project>_${pkgver}_*/data/* "${pkgdir}"
}

----------------------------------------

TITLE: Installing System Dependencies for Debian Linux
DESCRIPTION: Command to install required system packages for Tauri development on Debian-based Linux distributions using apt package manager.

LANGUAGE: sh
CODE:
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

----------------------------------------

TITLE: Managing Plugin Permissions in Rust
DESCRIPTION: Demonstrates how to check and request permissions using Rust in a Tauri plugin. Includes structures for permission requests and responses, as well as helper methods for permission management.

LANGUAGE: rust
CODE:
use serde::{Serialize, Deserialize};
use tauri::{plugin::PermissionState, Runtime};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PermissionResponse {
  pub post_notification: PermissionState,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestPermission {
  post_notification: bool,
}

impl<R: Runtime> Notification<R> {
  pub fn request_post_notification_permission(&self) -> crate::Result<PermissionState> {
    self.0
      .run_mobile_plugin::<PermissionResponse>("requestPermissions", RequestPermission { post_notification: true })
      .map(|r| r.post_notification)
      .map_err(Into::into)
  }

  pub fn check_permissions(&self) -> crate::Result<PermissionResponse> {
    self.0
      .run_mobile_plugin::<PermissionResponse>("checkPermissions", ())
      .map_err(Into::into)
  }
}

----------------------------------------

TITLE: Configuring Maximum Log File Size
DESCRIPTION: Setting the maximum size for log files before they are discarded or rotated.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .max_file_size(50_000 /* bytes */)
  .build()

----------------------------------------

TITLE: Creating a Tray Icon in JavaScript
DESCRIPTION: Demonstrates how to create a new system tray icon using the TrayIcon.new function in JavaScript. The options parameter allows for customization of the tray icon.

LANGUAGE: javascript
CODE:
import { TrayIcon } from '@tauri-apps/api/tray';

const options = {
  // here you can add a tray menu, title, tooltip, event handler, etc
};

const tray = await TrayIcon.new(options);

----------------------------------------

TITLE: Configuring Provisioning Profile in Tauri
DESCRIPTION: JSON configuration that specifies the location of the provisioning profile for macOS App Store distribution. This profile is required by Apple to authenticate and authorize app distribution.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "files": {
        "embedded.provisionprofile": "path/to/profile-name.provisionprofile"
      }
    }
  }
}

----------------------------------------

TITLE: Configuring Global Shortcut Plugin Permissions
DESCRIPTION: JSON configuration example for enabling required permissions for the global-shortcut plugin in a Tauri application. This allows checking registered shortcuts, registering new shortcuts, and unregistering existing ones.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "global-shortcut:allow-is-registered",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister"
  ]
}

----------------------------------------

TITLE: CSS for Custom Titlebar in Tauri
DESCRIPTION: CSS styles for implementing a custom titlebar, including positioning, dimensions, and button styling.

LANGUAGE: css
CODE:
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}
.titlebar-button:hover {
  background: #5bbec3;
}

----------------------------------------

TITLE: Initializing Deep Link Plugin in Rust
DESCRIPTION: Code to initialize the deep-link plugin in the Tauri application's Rust code.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Building Tauri Applications with Various Package Managers
DESCRIPTION: Command examples for building Tauri applications using different package managers like npm, yarn, pnpm, deno, bun, and cargo.

LANGUAGE: shell
CODE:
npm run tauri build
yarn tauri build
pnpm tauri build
deno task tauri build
bun tauri build
cargo tauri build

----------------------------------------

TITLE: Initializing OS Plugin in Rust
DESCRIPTION: Code to initialize the OS Information plugin in the Tauri application's lib.rs file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Installing System Dependencies for Alpine Linux
DESCRIPTION: Command to install required system packages for Tauri development on Alpine Linux using apk package manager.

LANGUAGE: sh
CODE:
sudo apk add \
  build-base \
  webkit2gtk \
  curl \
  wget \
  file \
  openssl \
  libayatana-appindicator-dev \
  librsvg

----------------------------------------

TITLE: Implementing Load Lifecycle Event in Android
DESCRIPTION: Shows how to implement the load lifecycle event in an Android Tauri plugin to perform initialization when the plugin is loaded into the web view.

LANGUAGE: kotlin
CODE:
import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.TauriPlugin

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
  override fun load(webView: WebView) {
    // perform plugin setup here
  }
}

----------------------------------------

TITLE: Updating Tauri Dependencies for Mobile Alpha Release
DESCRIPTION: Commands to update both NPM and Cargo dependencies to the Tauri 2.0.0-alpha.0 release. This includes updating the CLI, API packages, and installing the required Cargo dependencies.

LANGUAGE: bash
CODE:
npm install @tauri-apps/cli@next @tauri-apps/api@next

LANGUAGE: bash
CODE:
yarn upgrade @tauri-apps/cli@next @tauri-apps/api@next

LANGUAGE: bash
CODE:
pnpm update @tauri-apps/cli@next @tauri-apps/api@next

LANGUAGE: bash
CODE:
cargo add tauri@2.0.0-alpha.0
cargo add tauri-build@2.0.0-alpha.0 --build
cargo install tauri-cli --version "^2.0.0-alpha" --locked

----------------------------------------

TITLE: Listening to Webview-Specific Events in TypeScript with Tauri
DESCRIPTION: Shows how to listen to events specific to a webview in a Tauri application. This example gets the current webview window and registers a listener for a 'logged-in' event, storing the received session token in localStorage.

LANGUAGE: typescript
CODE:
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

const appWebview = getCurrentWebviewWindow();
appWebview.listen<string>('logged-in', (event) => {
  localStorage.setItem('session-token', event.payload);
});

----------------------------------------

TITLE: Configuring Sidecar Arguments in Capabilities JSON
DESCRIPTION: JSON configuration for defining both static and dynamic arguments that can be passed to a sidecar command, including validators for dynamic arguments.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    {
      "identifier": "shell:allow-execute",
      "allow": [
        {
          "args": [
            "arg1",
            "-a",
            "--arg2",
            {
              "validator": "\\S+"
            }
          ],
          "name": "binaries/my-sidecar",
          "sidecar": true
        }
      ]
    },
    "shell:allow-open"
  ]
}

----------------------------------------

TITLE: Creating Message Dialog in JavaScript
DESCRIPTION: Example of displaying a simple message dialog with an error styling using the message function.

LANGUAGE: javascript
CODE:
import { message } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { message } = window.__TAURI__.dialog;

// Shows message
await message('File not found', { title: 'Tauri', kind: 'error' });

----------------------------------------

TITLE: Adding Custom Files to the Application Bundle
DESCRIPTION: JSON configuration that specifies custom files to be included in the application bundle, mapping destination paths to source files. Files are added to the Contents folder.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "files": {
        "embedded.provisionprofile": "./profile-name.provisionprofile",
        "SharedSupport/docs.md": "./docs/index.md"
      }
    }
  }
}

----------------------------------------

TITLE: Adding Attachments to Notifications in JavaScript
DESCRIPTION: Code to send a notification with a media attachment using asset URLs.

LANGUAGE: javascript
CODE:
import { sendNotification } from '@tauri-apps/plugin-notification';

sendNotification({
  title: 'New Image',
  body: 'Check out this picture',
  attachments: [
    {
      id: 'image-1',
      url: 'asset:///notification-image.jpg',
    },
  ],
});

----------------------------------------

TITLE: Creating WiX Fragment for Registry Entries in Tauri
DESCRIPTION: XML definition of a WiX fragment that writes registry entries for a Tauri Windows application. This example sets up registry entries under HKEY_CURRENT_USER\Software\MyCompany\MyApplicationName.

LANGUAGE: xml
CODE:
<?xml version="1.0" encoding="utf-8"?>
<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
  <Fragment>
    <!-- these registry entries should be installed
		 to the target user's machine -->
    <DirectoryRef Id="TARGETDIR">
      <!-- groups together the registry entries to be installed -->
      <!-- Note the unique `Id` we provide here -->
      <Component Id="MyFragmentRegistryEntries" Guid="*">
        <!-- the registry key will be under
			 HKEY_CURRENT_USER\Software\MyCompany\MyApplicationName -->
        <!-- Tauri uses the second portion of the
			 bundle identifier as the `MyCompany` name
			 (e.g. `tauri-apps` in `com.tauri-apps.test`)  -->
        <RegistryKey
          Root="HKCU"
          Key="Software\MyCompany\MyApplicationName"
          Action="createAndRemoveOnUninstall"
        >
          <!-- values to persist on the registry -->
          <RegistryValue
            Type="integer"
            Name="SomeIntegerValue"
            Value="1"
            KeyPath="yes"
          />
          <RegistryValue Type="string" Value="Default Value" />
        </RegistryKey>
      </Component>
    </DirectoryRef>
  </Fragment>
</Wix>

----------------------------------------

TITLE: Removing a Notification Channel in JavaScript
DESCRIPTION: Code to remove a specific notification channel by its ID.

LANGUAGE: javascript
CODE:
import { removeChannel } from '@tauri-apps/plugin-notification';

await removeChannel('messages');

----------------------------------------

TITLE: Invalid Publisher Configuration Example
DESCRIPTION: Example of an invalid Tauri configuration where the publisher name (derived from identifier) conflicts with the product name, which is not allowed in Microsoft Store.

LANGUAGE: json
CODE:
{
  "productName": "Example",
  "identifier": "com.example.app"
}

----------------------------------------

TITLE: Defining Command Permissions in TOML
DESCRIPTION: This TOML configuration defines permissions that control access to plugin commands, specifying allow and deny rules for the start_server command in this example.

LANGUAGE: toml
CODE:
"$schema" = "schemas/schema.json"

[[permission]]
identifier = "allow-start-server"
description = "Enables the start_server command."
commands.allow = ["start_server"]

[[permission]]
identifier = "deny-start-server"
description = "Denies the start_server command."
commands.deny = ["start_server"]

----------------------------------------

TITLE: Configuring Core Plugin Permissions in Tauri 2.0 Beta
DESCRIPTION: JSON configuration for defining permissions in Tauri 2.0 beta version, listing individual permissions for various core functionalities.

LANGUAGE: json
CODE:
...\n"permissions": [\n    "path:default",\n    "event:default",\n    "window:default",\n    "app:default",\n    "image:default",\n    "resources:default",\n    "menu:default",\n    "tray:default",\n]\n...

----------------------------------------

TITLE: Configuring Trunk.toml for Tauri Integration
DESCRIPTION: This TOML configuration for Trunk ignores the Tauri source directory during file watching and sets the WebSocket protocol to "ws" for proper hot-reload functionality, especially important for mobile development.

LANGUAGE: toml
CODE:
# Trunk.toml
[watch]
ignore = ["./src-tauri"]

[serve]
ws_protocol = "ws"

----------------------------------------

TITLE: Displaying a Book Item with Purchase Options
DESCRIPTION: This code presents information about a Tauri-related book, including its cover image, title, author, and links to purchase or download it, using a custom BookItem component.

LANGUAGE: markdown
CODE:
<BookItem
  image={RoseRustBook}
  title="HTML, CSS, JavaScript, and Rust for Beginners: A Guide to Application Development with Tauri"
  alt="HTML, CSS, JavaScript, and Rust for Beginners Book Cover"
  author="James Alexander Rose"
  links={[
    {
      preText: 'Paperback on Amazon:',
      text: 'Buy Here',
      url: 'https://www.amazon.com/dp/B0DR6KZVVW',
    },
    {
      preText: 'Free PDF version:',
      text: 'Download (PDF 4MB)',
      url: '/assets/learn/community/HTML_CSS_JavaScript_and_Rust_for_Beginners_A_Guide_to_Application_Development_with_Tauri.pdf',
    },
  ]}
/>

----------------------------------------

TITLE: Saving window state in JavaScript
DESCRIPTION: JavaScript code example showing how to manually save the window state using the saveWindowState function from the window-state plugin, with both import and global Tauri object approaches.

LANGUAGE: javascript
CODE:
import { saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state';
// when using `"withGlobalTauri": true`, you may use
// const { saveWindowState, StateFlags } = window.__TAURI__.windowState;

saveWindowState(StateFlags.ALL);

----------------------------------------

TITLE: Configuring Update Requests with Rust in Tauri
DESCRIPTION: This Rust implementation demonstrates how to configure custom request parameters for the updater, including setting a timeout duration, proxy URL, and authorization headers when checking for updates.

LANGUAGE: rust
CODE:
use tauri_plugin_updater::UpdaterExt;
let update = app
  .updater_builder()
  .timeout(std::time::Duration::from_secs(30))
  .proxy("<proxy-url>".parse().expect("invalid URL"))
  .header("Authorization", "Bearer <token>")
  .build()?
  .check()
  .await?;

----------------------------------------

TITLE: Installing Biometric Plugin with Cargo
DESCRIPTION: Command to add the Tauri biometric plugin as a dependency in a Rust project, targeting Android and iOS platforms.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-biometric --target 'cfg(any(target_os = "android", target_os = "ios"))'

----------------------------------------

TITLE: Corrected Publisher Configuration
DESCRIPTION: Corrected Tauri configuration that explicitly sets the publisher name to avoid conflicts with the product name, which is required for Microsoft Store distribution.

LANGUAGE: json
CODE:
{
  "productName": "Example",
  "identifier": "com.example.app",
  "bundle": {
    "publisher": "Example Inc."
  }
}

----------------------------------------

TITLE: Creating Custom Log Filter Function
DESCRIPTION: Implementing a custom filter function to exclude logs based on their metadata.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  // exclude logs with target `"hyper"`
  .filter(|metadata| metadata.target() != "hyper")
  .build()

----------------------------------------

TITLE: Implementing Dialog Plugin in Rust
DESCRIPTION: Sets up the Dialog plugin in a Rust Tauri application.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
}

----------------------------------------

TITLE: Defining Plugin Configuration in Rust
DESCRIPTION: Demonstrates how to define and use a plugin configuration struct in Rust, which parses settings from the Tauri configuration file.

LANGUAGE: rust
CODE:
use tauri::plugin::{Builder, Runtime, TauriPlugin};
use serde::Deserialize;

// Define the plugin config
#[derive(Deserialize)]
struct Config {
  timeout: usize,
}

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
  // Make the plugin config optional
  // by using `Builder::<R, Option<Config>>` instead
  Builder::<R, Config>::new("<plugin-name>")
    .setup(|app, api| {
      let timeout = api.config().timeout;
      Ok(())
    })
    .build()
}

----------------------------------------

TITLE: Triggering App Links on Android Emulator
DESCRIPTION: Shell command to trigger an app link on Android emulator using the adb CLI. This command opens the specified URL in the application with the given bundle identifier.

LANGUAGE: sh
CODE:
adb shell am start -a android.intent.action.VIEW -d https://<host>/path <bundle-identifier>

----------------------------------------

TITLE: Creating Tauri Build Script
DESCRIPTION: A Rust build script (build.rs) that watches the dist/ directory for changes and runs the Tauri build-time helpers. This ensures the application recompiles when frontend files change.

LANGUAGE: rust
CODE:
fn main() {
    // Only watch the `dist/` directory for recompiling, preventing unnecessary
    // changes when we change files in other project subdirectories.
    println!("cargo:rerun-if-changed=dist");

    // Run the Tauri build-time helpers
    tauri_build::build()
}

----------------------------------------

TITLE: Checking RPM Package Dependencies
DESCRIPTION: Command to list all dependencies required by an RPM package.

LANGUAGE: bash
CODE:
rpm -qp --requires package_name.rpm

----------------------------------------

TITLE: Creating a Permission Set in an Application
DESCRIPTION: Example of extending plugin permissions in an application by creating a permission set. This combines multiple permissions from the fs plugin to allow home directory read access and directory creation.

LANGUAGE: toml
CODE:
[[set]]
identifier = "allow-home-read-extended"
description = """ This allows non-recursive read access to files and to create directories
in the `$HOME` folder.
"""
permissions = [
    "fs:read-files",
    "fs:scope-home",
    "fs:allow-mkdir"
]

----------------------------------------

TITLE: Configuring WiX Fragment References in Tauri
DESCRIPTION: Configuration to reference WiX fragments in tauri.conf.json. This example shows how to include custom fragment paths and component references for WiX installer customization.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "wix": {
        "fragmentPaths": ["./windows/fragments/registry.wxs"],
        "componentRefs": ["MyFragmentRegistryEntries"]
      }
    }
  }
}

----------------------------------------

TITLE: Configuring Timezone Strategy for Logs
DESCRIPTION: Setting the timezone strategy to use the local timezone instead of UTC for log timestamps.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
  .build()

----------------------------------------

TITLE: Installing snap on Arch Linux
DESCRIPTION: Installs the snapd package manager on Arch Linux by cloning from AUR, building the package, and enabling necessary systemd services.

LANGUAGE: shell
CODE:
sudo pacman -S --needed git base-devel
git clone https://aur.archlinux.org/snapd.git
cd snapd
makepkg -si
sudo systemctl enable --now snapd.socket
sudo systemctl start snapd.socket
sudo systemctl enable --now snapd.apparmor.service

----------------------------------------

TITLE: Using PredefinedMenuItem in Tauri 2.0
DESCRIPTION: Demonstrates how to use the new PredefinedMenuItem API in Tauri 2.0, which replaces the previous MenuItem API.

LANGUAGE: rust
CODE:
use tauri::menu::{MenuBuilder, PredefinedMenuItem};

tauri::Builder::default()
    .setup(|app| {
        let menu = MenuBuilder::new(app).item(&PredefinedMenuItem::copy(app)?).build()?;
        Ok(())
    })

----------------------------------------

TITLE: Creating Development Environment for NixOS
DESCRIPTION: Nix shell configuration for setting up a Tauri development environment on NixOS, which installs Rust, Node.js, and other required dependencies.

LANGUAGE: nix
CODE:
let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
  ];

  buildInputs = with pkgs;[
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
  ];
}

----------------------------------------

TITLE: Configuring Tauri Hooks in tauri.conf.json
DESCRIPTION: Configuration for integrating Node.js scripts with Tauri CLI through the beforeDevCommand and beforeBuildCommand hooks. This allows Tauri to automatically run frontend build scripts during development and production builds.

LANGUAGE: json
CODE:
{
  "build": {
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn build"
  }
}

----------------------------------------

TITLE: JavaScript/TypeScript Migration from v1/v2 to v3
DESCRIPTION: Code diff showing how to migrate JavaScript or TypeScript code from Store plugin v1 and v2 beta/rc to v3.

LANGUAGE: diff
CODE:
- import { Store } from '@tauri-apps/plugin-store';
+ import { LazyStore } from '@tauri-apps/plugin-store';

----------------------------------------

TITLE: Configuring Tauri with Yarn for Nuxt Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using Yarn as the package manager. Sets up build commands, development URL, and frontend distribution path for a Nuxt project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Copying Files with Tauri FS Plugin
DESCRIPTION: Copies a file from a source path to a destination path, allowing different base directories for each. This example shows copying a database file to a temporary location for backup.

LANGUAGE: javascript
CODE:
import { copyFile, BaseDirectory } from '@tauri-apps/plugin-fs';
await copyFile('user.db', 'user.db.bk', {
  fromPathBaseDir: BaseDirectory.AppLocalData,
  toPathBaseDir: BaseDirectory.Temp,
});

----------------------------------------

TITLE: Configuring Tauri with Yarn for Nuxt Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using Yarn as the package manager. Sets up build commands, development URL, and frontend distribution path for a Nuxt project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Configuring Log Plugin Permissions in Capabilities
DESCRIPTION: JSON configuration to grant permission for the log plugin to operate within a specified capability scope.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["log:default"]
}

----------------------------------------

TITLE: Initializing WebdriverIO Package Configuration in JSON
DESCRIPTION: Package.json file for WebdriverIO test setup that includes the required dependencies and test script. This configuration uses WebdriverIO with Mocha framework and Spec Reporter.

LANGUAGE: json
CODE:
{
  "name": "webdriverio",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "test": "wdio run wdio.conf.js"
  },
  "dependencies": {
    "@wdio/cli": "^7.9.1"
  },
  "devDependencies": {
    "@wdio/local-runner": "^7.9.1",
    "@wdio/mocha-framework": "^7.9.1",
    "@wdio/spec-reporter": "^7.9.0"
  }
}

----------------------------------------

TITLE: Manually Releasing a Snap Package
DESCRIPTION: Commands to authenticate with snapcraft and upload a snap package to the Snap Store, releasing it to the stable channel.

LANGUAGE: shell
CODE:
snapcraft login # Login with your UbuntuOne credentials
snapcraft upload --release=stable mysnap_latest_amd64.snap

----------------------------------------

TITLE: Structuring Search Component with Features and Community Resources
DESCRIPTION: This JSX code structures the search component with sections for Features and Community Resources. It includes a call-to-action for community contributions and displays plugins and integrations from the AwesomeTauri component.

LANGUAGE: jsx
CODE:
<Search>
  ## Features
  <FeaturesList />
  ## Community Resources
  <LinkCard
    title="Have something to share?"
    description="Open a pull request to show us your amazing resource."
    href="https://github.com/tauri-apps/awesome-tauri/pulls"
  />
  ### Plugins
  <AwesomeTauri section="plugins-no-official" />
  ### Integrations
  <AwesomeTauri section="integrations" />
</Search>

----------------------------------------

TITLE: Using MenuItemBuilder in Tauri 2.0
DESCRIPTION: Shows how to create custom menu items with the new MenuItemBuilder API in Tauri 2.0, which replaces the previous CustomMenuItem API.

LANGUAGE: rust
CODE:
use tauri::menu::MenuItemBuilder;

tauri::Builder::default()
    .setup(|app| {
        let toggle = MenuItemBuilder::new("Toggle").accelerator("Ctrl+Shift+T").build(app)?;
        Ok(())
    })

----------------------------------------

TITLE: Installing System Dependencies for Fedora Linux
DESCRIPTION: Command to install required system packages for Tauri development on Fedora Linux using dnf package manager.

LANGUAGE: sh
CODE:
sudo dnf check-update
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel
sudo dnf group install "c-development"

----------------------------------------

TITLE: Handling Command Errors in JavaScript
DESCRIPTION: Shows how to handle both successful and error results from Tauri commands in JavaScript using promise chaining with then() and catch() methods.

LANGUAGE: javascript
CODE:
invoke('login', { user: 'tauri', password: '0j4rijw8=' })
  .then((message) => console.log(message))
  .catch((error) => console.error(error));

----------------------------------------

TITLE: Handling Command Errors in JavaScript
DESCRIPTION: Shows how to handle both successful and error results from Tauri commands in JavaScript using promise chaining with then() and catch() methods.

LANGUAGE: javascript
CODE:
invoke('login', { user: 'tauri', password: '0j4rijw8=' })
  .then((message) => console.log(message))
  .catch((error) => console.error(error));

----------------------------------------

TITLE: RPM Macros Configuration for GPG Signing
DESCRIPTION: Configuration for the ~/.rpmmacros file to enable GPG signing verification of RPM packages.

LANGUAGE: bash
CODE:
%_signature gpg
%_gpg_path /home/johndoe/.gnupg
%_gpg_name Tauri-App
%_gpgbin /usr/bin/gpg2
%__gpg_sign_cmd %{__gpg} \
    gpg --force-v3-sigs --digest-algo=sha1 --batch --no-verbose --no-armor \
    --passphrase-fd 3 --no-secmem-warning -u "%{_gpg_name}" \
    -sbo %{__signature_filename} %{__plaintext_filename}

----------------------------------------

TITLE: Setting DMG Background Image in Tauri Configuration
DESCRIPTION: Configures a custom background image for the DMG installation window by specifying the image path in the tauri.conf.json file.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "dmg": {
        "background": "./images/"
      }
    }
  }
}

----------------------------------------

TITLE: Defining Home Directory Scope Permission
DESCRIPTION: Example of a permission configuration that grants access to all files and lists content of top-level directories in the user's home folder. This defines a scope-based permission.

LANGUAGE: toml
CODE:
[[permission]]
identifier = "scope-home"
description = """This scope permits access to all files and
list content of top level directories in the `$HOME`folder."""

[[scope.allow]]
path = "$HOME/*"

----------------------------------------

TITLE: Example Output from create-tauri-app Command
DESCRIPTION: Example output from running the create-tauri-app command, showing the interactive prompts and their selected values for configuring a new Tauri project.

LANGUAGE: shell
CODE:
✔ Project name · plugin-permission-demo
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, bun)
✔ Choose your package manager · pnpm
✔ Choose your UI template · Vanilla
✔ Choose your UI flavor · TypeScript

Template created! To get started run:
cd plugin-permission-demo
pnpm install
pnpm tauri dev

----------------------------------------

TITLE: Creating a Separate Configuration for App Store in Tauri
DESCRIPTION: A JSON configuration file specifically for App Store submissions that includes entitlements and provisioning profile settings. This allows developers to apply these configurations only when building for the App Store.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "entitlements": "./Entitlements.plist",
      "files": {
        "embedded.provisionprofile": "path/to/profile-name.provisionprofile"
      }
    }
  }
}

----------------------------------------

TITLE: Creating a Flatpak Manifest for Tauri Applications
DESCRIPTION: YAML manifest configuration for packaging a Tauri application as a Flatpak. Defines runtime requirements, permissions, and build steps to extract and install files from a Debian package.

LANGUAGE: yaml
CODE:
id: org.your.id

runtime: org.gnome.Platform
runtime-version: '46'
sdk: org.gnome.Sdk

command: tauri-app
finish-args:
  - --socket=wayland # Permission needed to show the window
  - --socket=fallback-x11 # Permission needed to show the window
  - --device=dri # OpenGL, not necessary for all projects
  - --share=ipc

modules:
  - name: binary
    buildsystem: simple
    sources:
      - type: file
        url: https://github.com/your_username/your_repository/releases/download/v1.0.1/yourapp_1.0.1_amd64.deb
        sha256: 08305b5521e2cf0622e084f2b8f7f31f8a989fc7f407a7050fa3649facd61469 # This is required if you are using a remote source
        only-arches: [x86_64] #This source is only used on x86_64 Computers
        # This path points to the binary file which was created in the .deb bundle.
        # Tauri also creates a folder which corresponds to the content of the unpacked .deb.
    build-commands:
      - ar -x *.deb
      - tar -xf data.tar.gz
      - 'install -Dm755 usr/bin/tauri-app /app/bin/tauri-app'
      - install -Dm644 usr/share/applications/yourapp.desktop /app/share/applications/org.your.id.desktop
      - install -Dm644 usr/share/icons/hicolor/128x128/apps/yourapp.png /app/share/icons/hicolor/128x128/apps/org.your.id.png
      - install -Dm644 usr/share/icons/hicolor/32x32/apps/yourapp.png /app/share/icons/hicolor/32x32/apps/org.your.id.png
      - install -Dm644 usr/share/icons/hicolor/256x256@2/apps/yourapp.png /app/share/icons/hicolor/256x256@2/apps/org.your.id.png
      - install -Dm644 org.your.id.metainfo.xml /app/share/metainfo/org.your.id.rosary.metainfo.xml

----------------------------------------

TITLE: Rendering Tauri Plugin Compatibility Table
DESCRIPTION: A simple JSX code block that renders the TableCompatibility component, which displays a support table for Tauri plugins with hover functionality for viewing additional notes.

LANGUAGE: jsx
CODE:
<TableCompatibility />

----------------------------------------

TITLE: Verifying File Content in Terminal
DESCRIPTION: Command to check the content of the text file written by the application. This displays the content of the test.txt file in the user's home directory.

LANGUAGE: shell
CODE:
cat $HOME/test.txt

----------------------------------------

TITLE: Adding OS Plugin to Cargo Dependencies
DESCRIPTION: Shows how to add the OS plugin to your Cargo.toml dependencies for a Tauri project.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-os = "2"

----------------------------------------

TITLE: Installing System Dependencies for Arch Linux
DESCRIPTION: Command to install required system packages for Tauri development on Arch Linux using pacman package manager.

LANGUAGE: sh
CODE:
sudo pacman -Syu
sudo pacman -S --needed \
  webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  libappindicator-gtk3 \
  librsvg

----------------------------------------

TITLE: Identifying Development Server Configuration in Tauri
DESCRIPTION: A JSON snippet from the tauri.conf.json file showing the beforeDevCommand configuration. This defines the command that should be executed before starting the development server, which in this example is 'pnpm dev'.

LANGUAGE: json
CODE:
    "beforeDevCommand": "pnpm dev"

----------------------------------------

TITLE: Creating a Tauri App with Bash
DESCRIPTION: Command to create a new Tauri application using Bash shell. This uses curl to fetch and execute the Tauri app creation script.

LANGUAGE: sh
CODE:
sh <(curl https://create.tauri.app/sh)

----------------------------------------

TITLE: Using Notification Plugin in JavaScript with Tauri
DESCRIPTION: Example of sending a notification using the Tauri notification plugin in JavaScript.

LANGUAGE: javascript
CODE:
import { sendNotification } from '@tauri-apps/plugin-notification';
sendNotification('Tauri is awesome!');

----------------------------------------

TITLE: Initializing Store Plugin in Rust Application
DESCRIPTION: Code snippet showing how to modify the Rust application entry point to initialize the Store plugin in a Tauri application.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Adding the OS Plugin Manually with Cargo
DESCRIPTION: Command to manually add the OS Information plugin as a dependency to the project's Cargo.toml file.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-os

----------------------------------------

TITLE: Implementing Tauri Plugin for iOS Using Swift
DESCRIPTION: Example Swift code for creating a Tauri plugin on iOS. The plugin implements a 'ping' function that takes a string value and resolves it as a dictionary, with the necessary initialization function.

LANGUAGE: swift
CODE:
import UIKit
import WebKit
import Tauri

class ExamplePlugin: Plugin {
	@objc public func ping(_ invoke: Invoke) throws {
		let value = invoke.getString("value")
		invoke.resolve(["value": value as Any])
	}
}

@_cdecl("init_plugin_example")
func initPlugin(name: SRString, webview: WKWebView?) {
	Tauri.registerPlugin(webview: webview, name: name.toString(), plugin: ExamplePlugin())
}


----------------------------------------

TITLE: Creating a Tauri Plugin Using CLI Commands
DESCRIPTION: Shell commands for bootstrapping a new Tauri plugin using the Tauri CLI. The commands initialize a new plugin called 'test', install dependencies, and build the plugin.

LANGUAGE: shell
CODE:
mkdir -p tauri-learning
cd tauri-learning
cargo tauri plugin new test
cd tauri-plugin-test
pnpm install
pnpm build
cargo build

----------------------------------------

TITLE: Initializing Tauri Plugin in Rust
DESCRIPTION: Rust code that initializes a Tauri plugin and registers the platform-specific implementations for both Android and iOS platforms. Uses conditional compilation with target_os attributes.

LANGUAGE: rust
CODE:
use tauri:{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_example);

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("example")
    .setup(|app, api| {
      #[cfg(target_os = "android")]
      api.register_android_plugin("com.plugin.example", "ExamplePlugin")?;
      #[cfg(target_os = "ios")]
      api.register_ios_plugin(init_plugin_example)?;
      Ok(())
    })
    .build()
}

----------------------------------------

TITLE: Using OS Plugin in JavaScript
DESCRIPTION: Demonstrates how to get system architecture information using the OS plugin in a JavaScript-based Tauri application.

LANGUAGE: javascript
CODE:
import { arch } from '@tauri-apps/plugin-os';
const architecture = await arch();

----------------------------------------

TITLE: Enabling DevTools in Production for Tauri Applications
DESCRIPTION: Configuration in Cargo.toml to enable the developer tools in production builds of Tauri applications by adding the 'devtools' feature. Note that this uses private APIs on macOS which can prevent App Store approval.

LANGUAGE: toml
CODE:
[dependencies]
tauri = { version = "...", features = ["...", "devtools"] }

----------------------------------------

TITLE: VS Code Style Task Configuration for Trunk Dev Server
DESCRIPTION: JSON configuration for VS Code style tasks to control a Trunk development server from Neovim using the overseer plugin.

LANGUAGE: json
CODE:
{
  "version": "2.0.0",
  "tasks": [
    {
      "type": "process",
      "label": "dev server",
      "command": "trunk",
      "args": ["serve"],
      "isBackground": true,
      "presentation": {
        "revealProblems": "onProblem"
      },
      "problemMatcher": {
        "pattern": {
          "regexp": "^error:.*",
          "file": 1,
          "line": 2
        },
        "background": {
          "activeOnStart": false,
          "beginsPattern": ".*Rebuilding.*",
          "endsPattern": ".*server listening at:.*"
        }
      }
    }
  ]
}

----------------------------------------

TITLE: Installing CLI Plugin via Cargo in Tauri
DESCRIPTION: Command to add the CLI plugin to the project dependencies in Cargo.toml file. It adds the plugin with a target configuration for supported platforms (macOS, Windows, Linux).

LANGUAGE: sh
CODE:
cargo add tauri-plugin-cli --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

----------------------------------------

TITLE: Scanning NFC Tags in JavaScript
DESCRIPTION: JavaScript code to scan NFC tags using the scan function from the NFC plugin. It shows how to configure scan type and options including custom messages to display during scanning.

LANGUAGE: javascript
CODE:
import { scan } from '@tauri-apps/plugin-nfc';

const scanType = {
  type: 'ndef', // or 'tag',
};

const options = {
  keepSessionAlive: false,
  // configure the messages displayed in the "Scan NFC" dialog on iOS
  message: 'Scan a NFC tag',
  successMessage: 'NFC tag successfully scanned',
};

const tag = await scan(scanType, options);

----------------------------------------

TITLE: Scanning NFC Tags in JavaScript
DESCRIPTION: JavaScript code to scan NFC tags using the scan function from the NFC plugin. It shows how to configure scan type and options including custom messages to display during scanning.

LANGUAGE: javascript
CODE:
import { scan } from '@tauri-apps/plugin-nfc';

const scanType = {
  type: 'ndef', // or 'tag',
};

const options = {
  keepSessionAlive: false,
  // configure the messages displayed in the "Scan NFC" dialog on iOS
  message: 'Scan a NFC tag',
  successMessage: 'NFC tag successfully scanned',
};

const tag = await scan(scanType, options);

----------------------------------------

TITLE: Permission Identifier Constants in Rust
DESCRIPTION: Shows the constants that define the length limitations for permission identifiers in Rust. These constants determine the maximum length of the identifier parts.

LANGUAGE: rust
CODE:
const IDENTIFIER_SEPARATOR: u8 = b':';
const PLUGIN_PREFIX: &str = "tauri-plugin-";

// https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field
const MAX_LEN_PREFIX: usize = 64 - PLUGIN_PREFIX.len();
const MAX_LEN_BASE: usize = 64;
const MAX_LEN_IDENTIFIER: usize = MAX_LEN_PREFIX + 1 + MAX_LEN_BASE;

----------------------------------------

TITLE: HTML for Manual Drag Region Implementation in Tauri
DESCRIPTION: Modified HTML structure for implementing a custom drag region without using the data-tauri-drag-region attribute, allowing for more customized drag behavior.

LANGUAGE: html
CODE:
<div id="titlebar" class="titlebar">
    <!-- ... -->
  </div>

----------------------------------------

TITLE: Checking Node.js Installation
DESCRIPTION: Commands to verify successful installation of Node.js and npm by checking their versions. Node.js is required when using JavaScript frontend frameworks with Tauri.

LANGUAGE: sh
CODE:
node -v
# v20.10.0
npm -v
# 10.2.3

----------------------------------------

TITLE: Creating a Signed PKG File for App Store Submission
DESCRIPTION: Command that uses xcrun productbuild to create a signed .pkg installer from the app bundle. The PKG file is required for App Store submissions and must be signed with a Mac Installer Distribution certificate.

LANGUAGE: bash
CODE:
xcrun productbuild --sign "<certificate signing identity>" --component "target/universal-apple-darwin/release/bundle/macos/$APPNAME.app" /Applications "$APPNAME.pkg"

----------------------------------------

TITLE: Rendering Link Cards in JSX for Tauri Documentation Navigation
DESCRIPTION: This code snippet renders a grid of link cards using Astrojs Starlight components. It creates a navigation interface with links to Tauri Philosophy, Governance, and Trademark pages, each with descriptive text.

LANGUAGE: jsx
CODE:
<CardGrid>
  <LinkCard
    title="Tauri Philosophy"
    href="/about/philosophy/"
    description="Learn more about the approach behind Tauri"
  />
  <LinkCard
    title="Governance"
    href="/about/governance/"
    description="Understand how the Tauri governance structure is setup"
  />
  <LinkCard
    title="Trademark"
    href="/about/trademark/"
    description="Guidelines for using the Tauri trademark"
  />
</CardGrid>

----------------------------------------

TITLE: Adding Cocoa Dependency for macOS in Tauri
DESCRIPTION: Cargo.toml configuration to add the cocoa crate as a dependency when targeting macOS, needed for native API calls to customize the titlebar.

LANGUAGE: toml
CODE:
[target."cfg(target_os = \"macos\")".dependencies]
cocoa = "0.26"

----------------------------------------

TITLE: Using OS Plugin in Rust
DESCRIPTION: Shows how to get system architecture information using the OS plugin in a Rust-based Tauri application.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            let os_arch = tauri_plugin_os::arch();
            Ok(())
        })
}

----------------------------------------

TITLE: Installing NSIS on macOS via Homebrew
DESCRIPTION: Command to install NSIS on macOS using the Homebrew package manager.

LANGUAGE: sh
CODE:
brew install nsis

----------------------------------------

TITLE: Beta Configuration Extension for Tauri
DESCRIPTION: Example of a configuration file used to extend the base configuration for creating a beta version of an application with a different product name and identifier.

LANGUAGE: json
CODE:
{
  "productName": "My App Beta",
  "identifier": "com.myorg.myappbeta"
}

----------------------------------------

TITLE: Creating Predefined Menu Items in JavaScript
DESCRIPTION: Demonstrates how to use built-in predefined menu items in JavaScript that have system-defined behaviors. Creates common menu actions like copy, paste, undo, redo, and select all.

LANGUAGE: javascript
CODE:
import { Menu, PredefinedMenuItem } from '@tauri-apps/api/menu';

const copy = await PredefinedMenuItem.new({
  text: 'copy-text',
  item: 'Copy',
});

const separator = await PredefinedMenuItem.new({
  text: 'separator-text',
  item: 'Separator',
});

const undo = await PredefinedMenuItem.new({
  text: 'undo-text',
  item: 'Undo',
});

const redo = await PredefinedMenuItem.new({
  text: 'redo-text',
  item: 'Redo',
});

const cut = await PredefinedMenuItem.new({
  text: 'cut-text',
  item: 'Cut',
});

const paste = await PredefinedMenuItem.new({
  text: 'paste-text',
  item: 'Paste',
});

const select_all = await PredefinedMenuItem.new({
  text: 'select_all-text',
  item: 'SelectAll',
});

const menu = await Menu.new({
  items: [copy, separator, undo, redo, cut, paste, select_all],
});

await menu.setAsAppMenu();

----------------------------------------

TITLE: Creating HTML Content for Splashscreen Page
DESCRIPTION: HTML markup for the splashscreen page that will be displayed during application initialization, linked to the application's styling.

LANGUAGE: html
CODE:
// /splashscreen.html
<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <link rel="stylesheet" href="/src/styles.css" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Tauri App</title>
</head>
<body>
    <div class="container">
        <h1>Tauri used Splash!</h1>
        <div class="row">
            <h5>It was super effective!</h5>
        </div>
    </div>
</body>
</html>

----------------------------------------

TITLE: RPM Post-Removal Script Template
DESCRIPTION: Sample shell script for the post-removal hook that shows how to access uninstallation parameters.

LANGUAGE: bash
CODE:
echo "-------------"
echo "This is postun"
echo "Install Value: $1"
echo "Upgrade Value: $1"
echo "Uninstall Value: $1"
echo "-------------"

----------------------------------------

TITLE: Using Updater Plugin in JavaScript
DESCRIPTION: This code demonstrates how to check for updates, download and install them, and then relaunch the application using the Tauri updater plugin in JavaScript. It also shows how to access update information.

LANGUAGE: javascript
CODE:
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

const update = await check();
if (update.response.available) {
  console.log(
    `Update to ${update.response.latestVersion} available! Date: ${update.response.date}`
  );
  console.log(`Release notes: ${update.response.body}`);
  await update.downloadAndInstall();
  // nécéssite le plugin `process`
  await relaunch();
}

----------------------------------------

TITLE: Creating a Branch for a New Flathub Application
DESCRIPTION: Git command to create and switch to a new branch named after your application for the Flathub submission process.

LANGUAGE: shell
CODE:
git checkout -b your_app_name

----------------------------------------

TITLE: Creating Base-Level Window Menu in Rust
DESCRIPTION: Implements a basic window menu in Rust using Tauri's Menu and MenuItem structures. The menu is created during application setup and contains basic open and close options.

LANGUAGE: rust
CODE:
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::menu::{Menu, MenuItem};

fn main() {
  tauri::Builder::default()
        .setup(|app| {
            let menu = MenuBuilder::new(app)
                .text("open", "Open")
                .text("close", "Close")
                .build()?;

            app.set_menu(menu)?;

            Ok(())
        })
}

----------------------------------------

TITLE: Adding a Shared Menu Click Handler in JavaScript
DESCRIPTION: Demonstrates how to create a shared click handler function for menu items in JavaScript. The handler receives the item ID as a parameter.

LANGUAGE: javascript
CODE:
import { Menu } from '@tauri-apps/api/menu';

function onTrayMenuClick(itemId) {
  // itemId === 'quit'
}

const menu = await Menu.new({
  items: [
    {
      id: 'quit',
      text: 'Quit',
      action: onTrayMenuClick,
    },
  ],
});

----------------------------------------

TITLE: Initializing Process Plugin in Rust
DESCRIPTION: Demonstrates how to initialize the Process plugin in a Rust-based Tauri application.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
}

----------------------------------------

TITLE: Installing NSIS on Fedora with Manual Plugin Setup
DESCRIPTION: Commands to install NSIS on Fedora Linux, which requires downloading and installing Stubs and Plugins manually as they're not included in the distribution package.

LANGUAGE: sh
CODE:
sudo dnf in mingw64-nsis
wget https://github.com/tauri-apps/binary-releases/releases/download/nsis-3/nsis-3.zip
unzip nsis-3.zip
sudo cp nsis-3.08/Stubs/* /usr/share/nsis/Stubs/
sudo cp -r nsis-3.08/Plugins/** /usr/share/nsis/Plugins/

----------------------------------------

TITLE: Unlistening from Events in Rust with Tauri
DESCRIPTION: Demonstrates how to stop listening to events in the Rust backend of a Tauri application. The example shows two approaches: unlistening outside the event handler scope, and unlistening when a specific event criteria is matched.

LANGUAGE: rust
CODE:
// unlisten outside of the event handler scope:
let event_id = app.listen("download-started", |event| {});
app.unlisten(event_id);

// unlisten when some event criteria is matched
let handle = app.handle().clone();
app.listen("status-changed", |event| {
  if event.data == "ready" {
    handle.unlisten(event.id);
  }
});

----------------------------------------

TITLE: Simplified Core Permissions Configuration in Tauri 2.0 RC
DESCRIPTION: Streamlined JSON configuration using the new 'core:default' permission set which includes all default permissions of core plugins in Tauri 2.0 RC.

LANGUAGE: json
CODE:
...\n"permissions": [\n    "core:default"\n]\n...

----------------------------------------

TITLE: Implementing WebDriver Testing with GitHub Actions for Tauri Applications
DESCRIPTION: A complete GitHub Actions workflow that sets up a Linux environment for running WebDriver tests on a Tauri application. The workflow installs necessary dependencies including webkit2gtk-driver and xvfb for headless testing, builds the Tauri application, and runs WebdriverIO tests through a fake display server.

LANGUAGE: yaml
CODE:
# run this action when the repository is pushed to
on: [push]

# the name of our workflow
name: WebDriver

jobs:
  # a single job named test
  test:
    # the display name of the test job
    name: WebDriverIO Test Runner

    # we want to run on the latest linux environment
    runs-on: ubuntu-22.04

    # the steps our job runs **in order**
    steps:
      # checkout the code on the workflow runner
      - uses: actions/checkout@v4

      # install system dependencies that Tauri needs to compile on Linux.
      # note the extra dependencies for `tauri-driver` to run which are: `webkit2gtk-driver` and `xvfb`
      - name: Tauri dependencies
        run: |
          sudo apt update && sudo apt install -y \
            libwebkit2gtk-4.1-dev \
            build-essential \
            curl \
            wget \
            file \
            libxdo-dev \
            libssl-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev \
            webkit2gtk-driver \
            xvfb

      - name: Setup rust-toolchain stable
        id: rust-toolchain
        uses: dtolnay/rust-toolchain@stable

      # we run our rust tests before the webdriver tests to avoid testing a broken application
      - name: Cargo test
        run: cargo test

      # build a release build of our application to be used during our WebdriverIO tests
      - name: Cargo build
        run: cargo build --release

      # install the latest stable node version at the time of writing
      - name: Node 20
        uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: 'yarn'

      # install our Node.js dependencies with Yarn
      - name: Yarn install
        run: yarn install --frozen-lockfile
        working-directory: webdriver/webdriverio

      # install the latest version of `tauri-driver`.
      # note: the tauri-driver version is independent of any other Tauri versions
      - name: Install tauri-driver
        run: cargo install tauri-driver --locked

      # run the WebdriverIO test suite.
      # we run it through `xvfb-run` (the dependency we installed earlier) to have a fake
      # display server which allows our application to run headless without any changes to the code
      - name: WebdriverIO
        run: xvfb-run yarn test
        working-directory: webdriver/webdriverio

----------------------------------------

TITLE: Converting iOS Signing Certificate to Base64 in macOS Terminal
DESCRIPTION: Command to convert an exported iOS signing certificate (.p12 file) to base64 format and copy it to the clipboard. This base64 string is used as the IOS_CERTIFICATE environment variable for manual code signing.

LANGUAGE: bash
CODE:
base64 -i <path-to-certificate.p12> | pbcopy

----------------------------------------

TITLE: Enabling Image Features in Cargo.toml
DESCRIPTION: Shows how to enable the image-png feature in the Cargo.toml configuration file, which is required for using icon images in Tauri menus.

LANGUAGE: toml
CODE:
[dependencies]
tauri = { version = "...", features = ["...", "image-png"] }

----------------------------------------

TITLE: Using the File System Plugin in JavaScript
DESCRIPTION: JavaScript code demonstrating how to check if a file exists using the Tauri file system plugin with the AppData base directory.

LANGUAGE: javascript
CODE:
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';
// when using `"withGlobalTauri": true`, you may use
// const { exists, BaseDirectory } = window.__TAURI__.fs;

// Check if the `$APPDATA/avatar.png` file exists
await exists('avatar.png', { baseDir: BaseDirectory.AppData });

----------------------------------------

TITLE: Configuring Relic for Azure Key Vault Certificate Signing in YAML
DESCRIPTION: YAML configuration for the relic tool to use Azure Key Vault for Windows code signing. This configuration specifies the Azure token type and the certificate URL with placeholders for the key vault name and certificate name.

LANGUAGE: yml
CODE:
tokens:
  azure:
    type: azure

keys:
  azure:
    token: azure
    id: https://\<KEY_VAULT_NAME\>.vault.azure.net/certificates/\<CERTIFICATE_NAME\>

----------------------------------------

TITLE: Configuring Android Version Code in Tauri Configuration File
DESCRIPTION: JSON configuration for setting a custom version code in the Tauri configuration file, which overrides the default version code calculation used for Android app publishing.

LANGUAGE: json
CODE:
{
  "bundle": {
    "android": {
      "versionCode": 100
    }
  }
}

----------------------------------------

TITLE: Registering All Deep Links in Development Mode
DESCRIPTION: Rust code to force register all statically configured deep links at runtime for development.

LANGUAGE: rust
CODE:
#[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
{
  use tauri_plugin_deep_link::DeepLinkExt;
  app.deep_link().register_all()?;
}

----------------------------------------

TITLE: Adding Process Plugin to JavaScript Dependencies
DESCRIPTION: Shows how to add the Process plugin to package.json dependencies for a JavaScript-based Tauri project.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-process": "^2.0.0"
  }
}

----------------------------------------

TITLE: Verifying RPM Package Signature
DESCRIPTION: Command to verify the signature of an RPM package using the rpm utility.

LANGUAGE: bash
CODE:
rpm  -v --checksig tauri-app-0.0.0-1.x86_64.rpm

----------------------------------------

TITLE: Configuring Static Frontend Source in Tauri
DESCRIPTION: JSON configuration for pointing Tauri to frontend source code. This setup is used when not using a UI framework or module bundler, allowing Tauri to start a development server for your frontend code.

LANGUAGE: json
CODE:
{
  "build": {
    "frontendDist": "./src"
  }
}

----------------------------------------

TITLE: Installing create-tauri-app with Alpha Support Using Various Package Managers
DESCRIPTION: Commands to install create-tauri-app version 3 with the alpha flag using various package managers and installation methods including pnpm, yarn, npm, Cargo, Bash, and PowerShell.

LANGUAGE: bash
CODE:
# pnpm
pnpm create tauri-app --alpha

# yarn
yarn create tauri-app --alpha

# npm
npm create tauri-app -- --alpha

# Cargo
cargo install create-tauri-app --locked
cargo create-tauri-app --alpha

# Bash
sh <(curl https://create.tauri.app/sh) --alpha

# Powershell
$env:CTA_ARGS="--alpha";iwr -useb https://create.tauri.app/ps | iex

----------------------------------------

TITLE: Configuring Global Shortcut Plugin in package.json
DESCRIPTION: Adds the @tauri-apps/plugin-global-shortcut dependency to package.json for JavaScript usage.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-global-shortcut": "^2.0.0"
  }
}

----------------------------------------

TITLE: Registering Log Plugin with Target Configuration
DESCRIPTION: Configuring and initializing the log plugin with custom targets for directing log output.

LANGUAGE: rust
CODE:
use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Declaring Updater Plugin in package.json
DESCRIPTION: This shows how to add the Tauri updater plugin as a dependency in your package.json file for JavaScript projects. This enables frontend code to interact with the updater functionality.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-updater": "^2.0.0"
  }
}

----------------------------------------

TITLE: Setting Minimum Android SDK Version in Tauri Configuration File
DESCRIPTION: JSON configuration for specifying the minimum supported Android SDK version for your Tauri app. This allows targeting newer Android APIs when needed.

LANGUAGE: json
CODE:
{
  "bundle": {
    "android": {
      "minSdkVersion": 28
    }
  }
}

----------------------------------------

TITLE: Appending to Files with Tauri FS Plugin
DESCRIPTION: Opens a file in append mode, adds content to the end of the file, and then closes it. This demonstrates how to add content to an existing file without overwriting its contents.

LANGUAGE: javascript
CODE:
import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
const file = await open('foo/bar.txt', {
  append: true,
  baseDir: BaseDirectory.AppData,
});
await file.write(new TextEncoder().encode('world'));
await file.close();

----------------------------------------

TITLE: Configuring Windows Code Signing in tauri.conf.json
DESCRIPTION: Configuration settings in tauri.conf.json that specify certificate thumbprint, digest algorithm, and timestamp URL for Windows code signing.

LANGUAGE: json
CODE:
"windows": {
        "certificateThumbprint": "A1B1A2B2A3B3A4B4A5B5A6B6A7B7A8B8A9B9A0B0",
        "digestAlgorithm": "sha256",
        "timestampUrl": "http://timestamp.comodoca.com"
}

----------------------------------------

TITLE: Migrating JavaScript Imports from tauri to core Module
DESCRIPTION: Updates import statements from the deprecated '@tauri-apps/api/tauri' module to the new '@tauri-apps/api/core' module for Tauri v2.

LANGUAGE: diff
CODE:
- import { invoke } from "@tauri-apps/api/tauri"
+ import { invoke } from "@tauri-apps/api/core"

----------------------------------------

TITLE: Getting RPM Package Information
DESCRIPTION: Command to retrieve detailed information about an RPM package, including version, release, and architecture.

LANGUAGE: bash
CODE:
rpm -qip package_name.rpm

----------------------------------------

TITLE: Initializing Window Plugin in Rust
DESCRIPTION: This code demonstrates how to initialize the window plugin in a Rust-based Tauri application. It shows the basic setup required to register the plugin with Tauri's builder.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
}

----------------------------------------

TITLE: Implementing Tauri Plugin for Android Using Kotlin
DESCRIPTION: Example Kotlin code for creating a Tauri plugin on Android. The plugin takes a string value and resolves it as a JSON object using the @TauriPlugin and @Command annotations.

LANGUAGE: kotlin
CODE:
package com.plugin.example

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    @Command
    fun ping(invoke: Invoke) {
        val value = invoke.getString("value") ?: ""
        val ret = JSObject()
        ret.put("value", value)
        invoke.resolve(ret)
    }
}

----------------------------------------

TITLE: Creating Tauri Plugin for Android with Kotlin Annotations
DESCRIPTION: Example showing how to use Kotlin annotations to define commands in an Android Tauri plugin. The @Command annotation marks methods that should be exposed to the Tauri frontend.

LANGUAGE: kotlin
CODE:
@Command

----------------------------------------

TITLE: Creating Directories with Tauri FS Plugin
DESCRIPTION: Creates a directory at the specified path. If the parent directories don't exist, this operation will fail unless recursive creation is specified.

LANGUAGE: javascript
CODE:
import { mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';
await mkdir('images', {
  baseDir: BaseDirectory.AppLocalData,
});

----------------------------------------

TITLE: Configuring Offline Installer in Tauri
DESCRIPTION: Configuration for using an offline installer with embedded WebView2 components in a Tauri Windows application. This increases installer size by about 127MB but allows installation without an internet connection.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "webviewInstallMode": {
        "type": "offlineInstaller"
      }
    }
  }
}

----------------------------------------

TITLE: Installing the window-state plugin with Cargo
DESCRIPTION: Command to add the window-state plugin to a Tauri project's dependencies in Cargo.toml, specifically targeting macOS, Windows, and Linux platforms.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-window-state --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

----------------------------------------

TITLE: Configuring Custom Sign Command in Tauri Configuration JSON
DESCRIPTION: JSON configuration for Tauri to use a custom sign command for Windows executables. This example configures Tauri to use the relic tool with the Azure Key Vault certificate to sign Windows installers.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "signCommand": "relic sign --file %1 --key azure --config relic.conf"
    }
  }
}

----------------------------------------

TITLE: Watching Files for Changes with Debounce in Tauri FS Plugin
DESCRIPTION: Watches a file for changes with debouncing, which means events are only emitted after a specified delay to prevent rapid successive notifications.

LANGUAGE: javascript
CODE:
import { watch, BaseDirectory } from '@tauri-apps/plugin-fs';
await watch(
  'app.log',
  (event) => {
    console.log('app.log event', event);
  },
  {
    baseDir: BaseDirectory.AppLog,
    delayMs: 500,
  }
);

----------------------------------------

TITLE: Implementing Clipboard Plugin in Rust
DESCRIPTION: Sets up the Clipboard Manager plugin in a Rust Tauri application.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
}

----------------------------------------

TITLE: Verbose RPM Package Installation
DESCRIPTION: Command to install an RPM package with detailed verbose output for debugging installation issues.

LANGUAGE: bash
CODE:
rpm -ivvh package_name.rpm

----------------------------------------

TITLE: Setting Custom Update Target in Rust
DESCRIPTION: This code shows how to set a custom update target in Rust, specifically for macOS using the darwin-universal target. This is useful for targeting specific platforms with different update packages.

LANGUAGE: rust
CODE:
fn main() {
    let mut updater = tauri_plugin_updater::Builder::new();
    #[cfg(target_os = "macos")]
    {
        updater = updater.target("darwin-universal");
    }
    tauri::Builder::default()
        .plugin(updater.build())
}

----------------------------------------

TITLE: Setting Application Icon as Tray Icon in Rust
DESCRIPTION: Shows how to use the application's default window icon as the tray icon in Rust. The icon is cloned from the app's default window icon.

LANGUAGE: rust
CODE:
let tray = TrayIconBuilder::new()
  .icon(app.default_window_icon().unwrap().clone())
  .build(app)?;

----------------------------------------

TITLE: Adding Tray-Icon Feature to Cargo.toml
DESCRIPTION: Configuration to add the tray-icon feature to the positioner plugin in Cargo.toml.

LANGUAGE: toml
CODE:
[dependencies]
tauri-plugin-positioner = { version = "2.0.0", features = ["tray-icon"] }

----------------------------------------

TITLE: Updating SvelteKit Configuration for Tauri Integration
DESCRIPTION: JavaScript configuration for svelte.config.js that imports and uses the static adapter required for Tauri compatibility.

LANGUAGE: js
CODE:
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter(),
  },
};

export default config;

----------------------------------------

TITLE: Configuring Azure Code Signing in Tauri Configuration JSON
DESCRIPTION: JSON configuration for Tauri to use Azure Code Signing service. This example sets up a custom sign command using the trusted-signing-cli tool with parameters for the signing endpoint, account name, certificate profile, and description for the signed content.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "signCommand": "trusted-signing-cli -e https://wus2.codesigning.azure.net -a MyAccount -c MyProfile -d MyApp %1"
    }
  }
}

----------------------------------------

TITLE: Watching Directories for Changes Immediately in Tauri FS Plugin
DESCRIPTION: Watches a directory for changes without debouncing, which means events are emitted as soon as they occur. The recursive option watches all subdirectories as well.

LANGUAGE: javascript
CODE:
import { watchImmediate, BaseDirectory } from '@tauri-apps/plugin-fs';
await watchImmediate(
  'logs',
  (event) => {
    console.log('logs directory event', event);
  },
  {
    baseDir: BaseDirectory.AppLog,
    recursive: true,
  }
);

----------------------------------------

TITLE: Using Dialog Plugin in JavaScript
DESCRIPTION: Shows how to use the Dialog plugin in JavaScript to create a file save dialog with filters.

LANGUAGE: javascript
CODE:
import { save } from '@tauri-apps/plugin-dialog';
const filePath = await save({
  filters: [
    {
      name: 'Image',
      extensions: ['png', 'jpeg'],
    },
  ],
});

----------------------------------------

TITLE: Custom Bundling for Tauri Applications
DESCRIPTION: Commands for splitting build and bundle steps to customize how platform bundles are generated, including options for different distribution channels like App Store vs direct distribution.

LANGUAGE: bash
CODE:
npm run tauri build -- --no-bundle
# bundle for distribution outside the macOS App Store
npm run tauri bundle -- --bundles app,dmg
# bundle for App Store distribution
npm run tauri bundle -- --bundles app --config src-tauri/tauri.appstore.conf.json

----------------------------------------

TITLE: Initializing Updater Plugin in Rust
DESCRIPTION: This code demonstrates how to initialize the updater plugin in a Rust-based Tauri application. It shows the basic setup required to register the plugin with Tauri's builder.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
}

----------------------------------------

TITLE: Integrating the LoginLayout with a Tauri application
DESCRIPTION: Set up a complete Tauri application with the egui login window, handling user authentication. Creates the egui window, defines a password validation function, and processes the result on a separate thread.

LANGUAGE: rust
CODE:
use tauri::Manager;
fn main() {
  tauri::Builder::default()
    .setup(|app| {
      app.wry_plugin(tauri_egui::EguiPluginBuilder::new(app.handle()));

      // the closure that is called when the submit button is clicked - validate the password
      let password_checker: Box<dyn Fn(&str) -> bool + Send> = Box::new(|s| s == "tauri-egui-released");

      let (egui_app, rx) = LoginLayout::new(
        password_checker,
        vec!["John".into(), "Jane".into(), "Joe".into()],
      );
      let native_options = tauri_egui::eframe::NativeOptions {
        resizable: false,
        ..Default::default()
      };

      app
        .state::<tauri_egui::EguiPluginHandle>()
        .create_window(
          "login".to_string(),
          Box::new(|_cc| Box::new(egui_app)),
          "Sign in".into(),
          native_options,
        )
        .unwrap();

      // wait for the window to be closed with the user data on another thread
      // you don't need to spawn a thread when using e.g. an async command
      std::thread::spawn(move || {
        if let Ok(signal) = rx.recv() {
          dbg!(signal);
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application")
}

----------------------------------------

TITLE: Creating Multi-Level Menu in JavaScript
DESCRIPTION: Implements a complex multi-level menu structure with various menu item types including text items, checkboxes, and icon items. This JavaScript code is incorrectly labeled and is actually Rust code for creating nested menus.

LANGUAGE: rust
CODE:
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{image::Image, menu::{CheckMenuItemBuilder, IconMenuItemBuilder, MenuBuilder, SubmenuBuilder}};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let text_menu = SubmenuBuilder::new(app, "File")
                .text("open", "Open")
                .text("quit", "Quit")
                .build()?;

            let lang_str = "en";
            let check_sub_item_1 = CheckMenuItemBuilder::new("English")
                .id("en")
                .checked(lang_str == "en")
                .build(app)?;

            let check_sub_item_2 = CheckMenuItemBuilder::new("Chinese")
                .id("en")
                .checked(lang_str == "en")
                .enabled(false)
                .build(app)?;

            let icon_image = Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap();

            let icon_item = IconMenuItemBuilder::new("icon")
                .icon(icon_image)
                .build(app)?;

            let check_menus = SubmenuBuilder::new(app, "language")
                .item(&check_sub_item_1)
                .item(&check_sub_item_2)
                .build()?;


            let menu = MenuBuilder::new(app)
                .items(&[&text_menu, &check_menus, &icon_item])
                .build()?;

            app.set_menu(menu)?;

            print!("Hello from setup");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Initializing Localhost Plugin in Tauri Application
DESCRIPTION: Code snippet showing how to initialize the localhost plugin in a Tauri application's lib.rs file. This adds the plugin to the Tauri Builder.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_localhost::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Installing SvelteKit Static Adapter for Tauri Integration
DESCRIPTION: Command for installing the @sveltejs/adapter-static package which is required for Tauri integration with SvelteKit, as Tauri doesn't support server-based solutions.

LANGUAGE: shell
CODE:
npm install --save-dev @sveltejs/adapter-static

----------------------------------------

TITLE: Configuring Localized WiX Installer Strings in Tauri
DESCRIPTION: Configuration for setting up localization strings for WiX installers in different languages. This example shows how to reference a custom locale file for Portuguese (Brazil).

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "wix": {
        "language": {
          "en-US": null,
          "pt-BR": {
            "localePath": "./wix/locales/pt-BR.wxl"
          }
        }
      }
    }
  }
}

----------------------------------------

TITLE: Validating AppImage Signature
DESCRIPTION: Commands to validate an AppImage signature using the AppImage validate tool. This involves making the validator executable and running it against your Tauri AppImage to check if the signature is valid.

LANGUAGE: shell
CODE:
chmod +x validate-$PLATFORM.AppImage
./validate-$PLATFORM.AppImage $TAURI_OUTPUT.AppImage

----------------------------------------

TITLE: Setting Module-Specific Log Levels
DESCRIPTION: Configuring different maximum log levels for specific modules to allow more detailed logging in targeted areas.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .level(log::LevelFilter::Info)
  // verbose logs only for the commands module
  .level_for("my_crate_name::commands", log::LevelFilter::Trace)
  .build()

----------------------------------------

TITLE: Initializing HTTP Plugin in Rust
DESCRIPTION: Demonstrates how to initialize the HTTP plugin in a Rust-based Tauri application.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
}

----------------------------------------

TITLE: Navigating to the Flathub Repository Directory
DESCRIPTION: Command to change the current directory to the cloned Flathub repository.

LANGUAGE: shell
CODE:
cd flathub

----------------------------------------

TITLE: Handling Menu Events in Tauri 2.0
DESCRIPTION: Example of setting up a menu and handling menu events using the new on_menu_event API in Tauri 2.0.

LANGUAGE: rust
CODE:
use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder};

tauri::Builder::default()
    .setup(|app| {
        let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;
        let check = CheckMenuItemBuilder::new("Mark").build(app)?;
        let menu = MenuBuilder::new(app).items(&[&toggle, &check]).build()?;

        app.set_menu(menu)?;

        app.on_menu_event(move |app, event| {
            if event.id() == check.id() {
                println!("`check` triggered, do something! is checked? {}", check.is_checked().unwrap());
            } else if event.id() == "toggle" {
                println!("toggle triggered!");
            }
        });
        Ok(())
    })

----------------------------------------

TITLE: Static JSON File Structure for Tauri Updater
DESCRIPTION: Example of a static JSON file structure for the Tauri updater plugin. Contains version information, release notes, publication date, and platform-specific download URLs with signatures for various operating systems.

LANGUAGE: json
CODE:
{
  "version": "",
  "notes": "",
  "pub_date": "",
  "platforms": {
    "linux-x86_64": {
      "signature": "",
      "url": ""
    },
    "windows-x86_64": {
      "signature": "",
      "url": ""
    },
    "darwin-x86_64": {
      "signature": "",
      "url": ""
    }
  }
}

----------------------------------------

TITLE: Configuring Updater Permissions in Tauri Capabilities
DESCRIPTION: JSON configuration snippet showing how to grant the necessary permissions for the updater plugin in the Tauri capabilities configuration file. This is required as all potentially dangerous plugin commands are blocked by default.

LANGUAGE: json
CODE:
{
  "permissions": [
    ...,
    "updater:default",
  ]
}

----------------------------------------

TITLE: Creating WiX Localization File for Tauri Installer
DESCRIPTION: XML definition of a WiX localization file for customizing installer strings. This example defines localized strings for application name, downgrade error message, and feature descriptions.

LANGUAGE: xml
CODE:
<WixLocalization
  Culture="en-US"
  xmlns="http://schemas.microsoft.com/wix/2006/localization"
>
  <String Id="LaunchApp"> Launch MyApplicationName </String>
  <String Id="DowngradeErrorMessage">
    A newer version of MyApplicationName is already installed.
  </String>
  <String Id="PathEnvVarFeature">
    Add the install location of the MyApplicationName executable to
    the PATH system environment variable. This allows the
    MyApplicationName executable to be called from any location.
  </String>
  <String Id="InstallAppFeature">
    Installs MyApplicationName.
  </String>
</WixLocalization>

----------------------------------------

TITLE: Configuring Custom Files in Debian Packages for Tauri Applications
DESCRIPTION: This snippet demonstrates how to include custom files and folders in the Debian package using the tauri.conf.json configuration file. It shows mapping paths in the Debian package to files on your filesystem, relative to the configuration file.

LANGUAGE: json
CODE:
{
  "bundle": {
    "linux": {
      "deb": {
        "files": {
          "/usr/share/README.md": "../README.md", // copies the README.md file to /usr/share/README.md
          "/usr/share/assets": "../assets/" // copies the entire assets directory to /usr/share/assets
        }
      }
    }
  }
}

----------------------------------------

TITLE: Configuring OS Plugin Permissions
DESCRIPTION: JSON configuration to add the necessary permissions for the OS Information plugin in the Tauri capabilities configuration file.

LANGUAGE: json
CODE:
{
  "permissions": [
    ...,
    "os:default"
  ]
}

----------------------------------------

TITLE: Configuring Custom Files in AppImage Bundle with Tauri
DESCRIPTION: This snippet demonstrates how to include custom files in an AppImage bundle by configuring the tauri.conf.json file. It shows how to map paths in the AppImage to files or directories on the local filesystem, with paths relative to the tauri.conf.json file.

LANGUAGE: json
CODE:
{
  "bundle": {
    "linux": {
      "appimage": {
        "files": {
          "/usr/share/README.md": "../README.md", // copies the ../README.md file to <appimage>/usr/share/README.md
          "/usr/assets": "../assets/" // copies the entire ../assets directory to <appimage>/usr/assets
        }
      }
    }
  }
}

----------------------------------------

TITLE: Using HTTP Plugin in JavaScript
DESCRIPTION: Demonstrates how to use the HTTP plugin to make fetch requests in a JavaScript-based Tauri application.

LANGUAGE: javascript
CODE:
import { fetch } from '@tauri-apps/plugin-http';
const response = await fetch(
  'https://raw.githubusercontent.com/tauri-apps/tauri/dev/package.json'
);

----------------------------------------

TITLE: Configuring DMG Window Size in Tauri Configuration
DESCRIPTION: Sets custom width and height for the DMG installation window to accommodate custom background images by modifying the windowSize property in tauri.conf.json.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "dmg": {
        "windowSize": {
          "width": 800,
          "height": 600
        }
      }
    }
  }
}

----------------------------------------

TITLE: Creating Submenus with SubmenuBuilder in Tauri 2.0
DESCRIPTION: Example of creating a submenu with various items using the new tauri::menu::SubmenuBuilder API.

LANGUAGE: rust
CODE:
use tauri::menu::{MenuBuilder, SubmenuBuilder};

tauri::Builder::default()
    .setup(|app| {
        let submenu = SubmenuBuilder::new(app, "Sub")
            .text("Tauri")
            .separator()
            .check("Is Awesome")
            .build()?;
        let menu = MenuBuilder::new(app).item(&submenu).build()?;
        Ok(())
    })

----------------------------------------

TITLE: Configuring Tauri with Qwik using pnpm
DESCRIPTION: Tauri configuration for a Qwik project using pnpm as the package manager. It specifies the development URL, frontend distribution directory, and commands to run before development and build processes.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "devUrl": "http://localhost:5173"
    "frontendDist": "../dist",
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build"
  }
}

----------------------------------------

TITLE: Setting up Tray Event Handling for Positioner
DESCRIPTION: Rust code for setting up tray event handling to enable tray-relative positioning functionality.

LANGUAGE: rust
CODE:
pub fn run() {
	tauri::Builder::default()
		// This is required to get tray-relative positions to work
		.setup(|app| {
				#[cfg(desktop)]
				{
					app.handle().plugin(tauri_plugin_positioner::init());
						tauri::tray::TrayIconBuilder::new()
							.on_tray_icon_event(|tray_handle, event| {
								tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);
							})
							.build(app)?
				}
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring NSIS Installer Hooks in Tauri Configuration
DESCRIPTION: JSON configuration for Tauri that specifies the path to the NSIS installer hooks file. This entry goes in the tauri.conf.json file to tell the bundler where to find the custom hooks.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "nsis": {
        "installerHooks": "./windows/hooks.nsi"
      }
    }
  }
}

----------------------------------------

TITLE: Initializing Persisted Scope Plugin in Tauri Application
DESCRIPTION: Code snippet showing how to modify the Rust lib.rs file to initialize the persisted-scope plugin in a Tauri application. This adds the plugin to the Tauri builder chain.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_persisted_scope::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Appending Target Triple to Binary with Node.js
DESCRIPTION: Node.js script that automatically appends the target triple to a binary file name, making it compatible with Tauri's sidecar naming requirements.

LANGUAGE: javascript
CODE:
import { execSync } from 'child_process';
import fs from 'fs';

const extension = process.platform === 'win32' ? '.exe' : '';

const rustInfo = execSync('rustc -vV');
const targetTriple = /host: (\S+)/g.exec(rustInfo)[1];
if (!targetTriple) {
  console.error('Failed to determine platform target triple');
}
fs.renameSync(
  `src-tauri/binaries/sidecar${extension}`,
  `src-tauri/binaries/sidecar-${targetTriple}${extension}`
);

----------------------------------------

TITLE: Setting DMG Window Position in Tauri Configuration
DESCRIPTION: Configures the initial position of the DMG window on screen by specifying x and y coordinates in the tauri.conf.json file.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "dmg": {
        "windowPosition": {
          "x": 400,
          "y": 400
        }
      }
    }
  }
}

----------------------------------------

TITLE: Creating Info.plist for Encryption Export Compliance
DESCRIPTION: XML file that declares whether the app uses non-exempt encryption, which is required for compliance with encryption export regulations. This must be included in App Store submissions.

LANGUAGE: xml
CODE:
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>ITSAppUsesNonExemptEncryption</key>
	<false/> # or `true` if your app uses encryption
</dict>
</plist>

----------------------------------------

TITLE: Installing Tauri Positioner Plugin with CLI
DESCRIPTION: Command examples to install the positioner plugin using different package managers.

LANGUAGE: sh
CODE:
cargo tauri add positioner

LANGUAGE: sh
CODE:
cargo add tauri-plugin-positioner --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

----------------------------------------

TITLE: Using HTTP Plugin in Rust
DESCRIPTION: Shows how to make HTTP requests using the reqwest library re-exported by the HTTP plugin in a Rust-based Tauri application.

LANGUAGE: rust
CODE:
use tauri_plugin_http::reqwest;

tauri::Builder::default()
    .plugin(tauri_plugin_http::init())
    .setup(|app| {
        let response_data = tauri::async_runtime::block_on(async {
            let response = reqwest::get(
                "https://raw.githubusercontent.com/tauri-apps/tauri/dev/package.json",
            )
            .await
            .unwrap();
            response.text().await
        })?;
        Ok(())
    })

----------------------------------------

TITLE: PKGBUILD for Extracting Tauri App from Debian Package
DESCRIPTION: A complete PKGBUILD file for packaging a Tauri application by extracting it from a Debian package. Includes package metadata, checksums, and extraction logic to create a proper Arch package.

LANGUAGE: ini
CODE:
# Maintainer:
# Contributor:
pkgname=<pkgname>
pkgver=1.0.0
pkgrel=1
pkgdesc="Description of your app"
arch=('x86_64' 'aarch64')
url="https://github.com/<user>/<project>"
license=('MIT')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')
options=('!strip' '!debug')
install=${pkgname}.install
source_x86_64=("${url}/releases/download/v${pkgver}/appname_${pkgver}_amd64.deb")
source_aarch64=("${url}/releases/download/v${pkgver}/appname_${pkgver}_arm64.deb")
sha256sums_x86_64=('ca85f11732765bed78f93f55397b4b4cbb76685088553dad612c5062e3ec651f')
sha256sums_aarch64=('ed2dc3169d34d91188fb55d39867713856dd02a2360ffe0661cb2e19bd701c3c')
package() {
	# Extract package data
	tar -xvf data.tar.gz -C "${pkgdir}"

}

----------------------------------------

TITLE: Creating Menu Items with MenuItemBuilder in Tauri 2.0
DESCRIPTION: Example of creating a custom menu item with accelerator using the new tauri::menu::MenuItemBuilder API.

LANGUAGE: rust
CODE:
use tauri::menu::MenuItemBuilder;

tauri::Builder::default()
    .setup(|app| {
        let toggle = MenuItemBuilder::new("Toggle").accelerator("Ctrl+Shift+T").build(app)?;
        Ok(())
    })

----------------------------------------

TITLE: Installing Tauri Autostart Plugin Manually with Cargo
DESCRIPTION: Command to add the Tauri autostart plugin as a dependency in Cargo.toml, targeting desktop platforms.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-autostart --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

----------------------------------------

TITLE: HTML Navigation Help Section for 404 Page
DESCRIPTION: A paragraph containing links to help users report navigation issues they encounter. It provides links to create a GitHub issue or report the problem on Discord.

LANGUAGE: html
CODE:
  <p>
    If you're having trouble navigating, please <a href="https://github.com/tauri-apps/tauri-docs/issues/new/choose">create an issue on GitHub</a> or <a href="https://discord.com/invite/tauri"
      >report on Discord</a
    >.
  </p>

----------------------------------------

TITLE: Configuring launch.json for Windows-specific Tauri Debugging
DESCRIPTION: This configuration sets up VS Code to debug Tauri applications on Windows using the Visual Studio Windows Debugger (cppvsdbg). It directly targets the compiled executable instead of using cargo commands.

LANGUAGE: json
CODE:
{
  // Use IntelliSense to learn about possible attributes.
  // Hover to view descriptions of existing attributes.
  // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Launch App Debug",
      "type": "cppvsdbg",
      "request": "launch",
      // change the exe name to your actual exe name
      // (to debug release builds, change `target/debug` to `release/debug`)
      "program": "${workspaceRoot}/src-tauri/target/debug/your-app-name-here.exe",
      "cwd": "${workspaceRoot}",
      "preLaunchTask": "ui:dev"
    }
  ]
}

----------------------------------------

TITLE: Configuring Sidecar Permission in Tauri Capabilities JSON
DESCRIPTION: JSON configuration for the capabilities file to grant permission for running a sidecar binary using the shell:allow-execute identifier.

LANGUAGE: json
CODE:
{
  "permissions": [
    "core:default",
    {
      "identifier": "shell:allow-execute",
      "allow": [
        {
          "name": "binaries/app",
          "sidecar": true
        }
      ]
    },
    "shell:allow-open"
  ]
}

----------------------------------------

TITLE: Uploading a macOS App to App Store with altool
DESCRIPTION: Command that uses xcrun altool to upload the signed PKG file to the App Store for review and distribution. This requires App Store Connect API keys for authentication.

LANGUAGE: bash
CODE:
xcrun altool --upload-app --type macos --file "$APPNAME.pkg" --apiKey $APPLE_API_KEY_ID --apiIssuer $APPLE_API_ISSUER

----------------------------------------

TITLE: Using Positioner Plugin in Rust
DESCRIPTION: Rust example showing how to use the positioner plugin directly from Rust code to move a window to a predefined position.

LANGUAGE: rust
CODE:
use tauri_plugin_positioner::{WindowExt, Position};

let mut win = app.get_webview_window("main").unwrap();
let _ = win.as_ref().window().move_window(Position.TopRight);

----------------------------------------

TITLE: Installing Flatpak Tools on Arch Linux
DESCRIPTION: Command to install the required flatpak and flatpak-builder tools on Arch Linux.

LANGUAGE: shell
CODE:
sudo pacman -S --needed flatpak flatpak-builder

----------------------------------------

TITLE: Adding Notification Plugin to JavaScript Dependencies
DESCRIPTION: Shows how to add the Notification plugin to package.json dependencies for a JavaScript-based Tauri project.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-notification": "^2.0.0"
  }
}

----------------------------------------

TITLE: Generating .SRCINFO file for AUR packages
DESCRIPTION: Command to generate the required .SRCINFO file for AUR packages. This file contains metadata about the package and is required when publishing to the AUR.

LANGUAGE: sh
CODE:
makepkg --printsrcinfo > .SRCINFO

----------------------------------------

TITLE: Using PredefinedMenuItem in Tauri 2.0
DESCRIPTION: Example of creating a menu with a predefined copy menu item using the new tauri::menu::PredefinedMenuItem API.

LANGUAGE: rust
CODE:
use tauri::menu::{MenuBuilder, PredefinedMenuItem};

tauri::Builder::default()
    .setup(|app| {
        let menu = MenuBuilder::new(app).item(&PredefinedMenuItem::copy(app)?).build()?;
        Ok(())
    })

----------------------------------------

TITLE: Using Global Shortcut Plugin in JavaScript with Tauri
DESCRIPTION: Example of registering a global keyboard shortcut in JavaScript that logs a message when triggered.

LANGUAGE: javascript
CODE:
import { register } from '@tauri-apps/plugin-global-shortcut';
await register('CommandOrControl+Shift+C', () => {
  console.log('Shortcut triggered');
});

----------------------------------------

TITLE: Initializing the File System Plugin in Rust
DESCRIPTION: Code to initialize the file system plugin in a Tauri Rust application by adding it to the Builder configuration.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring Tauri with Verso Runtime in Rust
DESCRIPTION: Example of setting up a Tauri application with the Verso runtime. It demonstrates how to configure the Verso path and resource directory before initializing the Tauri builder with VersoRuntime. The INVOKE_SYSTEM_SCRIPTS are added to ensure command compatibility.

LANGUAGE: rust
CODE:
use tauri_runtime_verso::
    {INVOKE_SYSTEM_SCRIPTS, VersoRuntime, set_verso_path, set_verso_resource_directory};

fn main() {
    // You need to set this to the path of the versoview executable
    // before creating any of the webview windows
    set_verso_path("../verso/target/debug/versoview");
    // Set this to verso/servo's resources directory before creating any of the webview windows
    // this is optional but recommended, this directory will include very important things
    // like user agent stylesheet
    set_verso_resource_directory("../verso/resources");
    tauri::Builder::<VersoRuntime>::new()
        // Make sure to do this or some of the commands will not work
        .invoke_system(INVOKE_SYSTEM_SCRIPTS.to_owned())
        .run(tauri::generate_context!())
        .unwrap();
}

----------------------------------------

TITLE: Implementing Window Focus on New Instance Attempt
DESCRIPTION: Complete implementation that adds window focusing functionality when a user attempts to open a new instance of the application.

LANGUAGE: rust
CODE:
use tauri::{AppHandle, Manager};

pub fn run() {
    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = app.get_webview_window("main")
                       .expect("no main window")
                       .set_focus();
        }));
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Refreshing Environment Variables in Current PowerShell Session
DESCRIPTION: PowerShell command to refresh environment variables in the current session after setting them, allowing immediate use without rebooting or logging out.

LANGUAGE: powershell
CODE:
[System.Environment]::GetEnvironmentVariables("User").GetEnumerator() | % { Set-Item -Path "Env:\$($_.key)" -Value $_.value }

----------------------------------------

TITLE: Configuring RPM Scripts in Tauri Configuration
DESCRIPTION: JSON configuration for adding RPM scripts to the tauri.conf.json file with paths to the script files.

LANGUAGE: json
CODE:
{
  "bundle": {
    "linux": {
      "rpm": {
        "epoch": 0,
        "files": {},
        "release": "1",
        // add the script here
        "preInstallScript": "/path/to/your/project/src-tauri/scripts/prescript.sh",
        "postInstallScript": "/path/to/your/project/src-tauri/scripts/postscript.sh",
        "preRemoveScript": "/path/to/your/project/src-tauri/scripts/prescript.sh",
        "postRemoveScript": "/path/to/your/project/src-tauri/scripts/postscript.sh"
      }
    }
  }
}

----------------------------------------

TITLE: Creating Entitlements.plist for App Sandbox
DESCRIPTION: Example of an Entitlements.plist file that enables the App Sandbox feature, which is Apple's security mechanism to limit an application's access to system resources.

LANGUAGE: xml
CODE:
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <true/>
</dict>
</plist>

----------------------------------------

TITLE: Initializing the window-state plugin in Rust
DESCRIPTION: Code modification for lib.rs to initialize the window-state plugin in a Tauri application, adding it to the application setup process.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_window_state::Builder::default().build());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Cloning an AUR repository in Shell
DESCRIPTION: Command to clone an empty git repository from the Arch User Repository for publishing your application. This is the first step in the AUR package publication process.

LANGUAGE: sh
CODE:
git clone https://aur.archlinux.org/your-repo-name

----------------------------------------

TITLE: Configuring HTTP Plugin in package.json
DESCRIPTION: Adds the @tauri-apps/plugin-http dependency to package.json for JavaScript HTTP requests.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-http": "^2.0.0"
  }
}

----------------------------------------

TITLE: Adding Dialog Plugin Dependency with Cargo
DESCRIPTION: Command to add the Tauri Dialog plugin as a dependency in a Rust project using Cargo.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-dialog

----------------------------------------

TITLE: Implementing a Tauri Command to Execute Node.js Sidecar in Rust
DESCRIPTION: Rust implementation of a Tauri command that executes the Node.js sidecar. It passes a 'ping' command with the provided message to the sidecar and returns the response.

LANGUAGE: rust
CODE:
#[tauri::command]
async fn ping(app: tauri::AppHandle, message: String) -> String {
  let sidecar_command = app
    .shell()
    .sidecar("app")
    .unwrap()
    .arg("ping")
    .arg(message);
  let output = sidecar_command.output().unwrap();
  let response = String::from_utf8(output.stdout).unwrap();
  response
}

----------------------------------------

TITLE: Adding iOS Rust Targets with rustup
DESCRIPTION: This command adds the necessary Rust targets for iOS development including ARM64, x86_64, and ARM64 simulator architecture support.

LANGUAGE: sh
CODE:
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim

----------------------------------------

TITLE: Including macOS Frameworks in the Application Bundle
DESCRIPTION: JSON configuration that specifies additional macOS frameworks needed by the application, including both system frameworks and custom libraries (.dylib files).

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "frameworks": [
        "CoreAudio",
        "./libs/libmsodbcsql.18.dylib",
        "./frameworks/MyApp.framework"
      ]
    }
  }
}

----------------------------------------

TITLE: Example of Rust Compiler Error Output
DESCRIPTION: An example of the detailed error messages provided by the Rust compiler, showing line numbers and suggestions for fixing common issues.

LANGUAGE: bash
CODE:
error[E0425]: cannot find value `sun` in this scope
  --> src/main.rs:11:5
   |
11 |     sun += i.to_string().parse::<u64>().unwrap();
   |     ^^^ help: a local variable with a similar name exists: `sum`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.

----------------------------------------

TITLE: Connecting to MySQL Database in JavaScript
DESCRIPTION: JavaScript code that loads and interacts with a MySQL database using the Tauri SQL plugin.

LANGUAGE: javascript
CODE:
import Database from '@tauri-apps/plugin-sql';
// when using `"withGlobalTauri": true`, you may use
// const Database = window.__TAURI__.sql;

const db = await Database.load('mysql://user:password@host/test');
await db.execute('INSERT INTO ...');

----------------------------------------

TITLE: Configuring Entitlements File Path in Tauri
DESCRIPTION: JSON configuration that specifies the path to the entitlements file for macOS bundle. This links the entitlements definition to the app during the build process.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "entitlements": "./Entitlements.plist"
    }
  }
}

----------------------------------------

TITLE: Adding HTTP Plugin to Cargo Dependencies
DESCRIPTION: Adds the tauri-plugin-http dependency to Cargo.toml for HTTP request functionality.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-http = "2"

----------------------------------------

TITLE: Installing Upload Plugin with Package Manager
DESCRIPTION: Command to add the Tauri Upload plugin to your project using various package managers.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-upload

----------------------------------------

TITLE: Handling Menu Events in Tauri 2.0
DESCRIPTION: Shows how to handle menu events with the new API in Tauri 2.0, using App::on_menu_event instead of the removed Builder::on_menu_event.

LANGUAGE: rust
CODE:
use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder};

tauri::Builder::default()
    .setup(|app| {
        let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;
        let check = CheckMenuItemBuilder::new("Mark").build(app)?;
        let menu = MenuBuilder::new(app).items(&[&toggle, &check]).build()?;

        app.set_menu(menu)?;

        app.on_menu_event(move |app, event| {
            if event.id() == check.id() {
                println!("`check` triggered, do something! is checked? {}", check.is_checked().unwrap());
            } else if event.id() == "toggle" {
                println!("toggle triggered!");
            }
        });
        Ok(())
    })

----------------------------------------

TITLE: Configuring External Binary in Tauri Configuration
DESCRIPTION: JSON configuration for the Tauri application that specifies the sidecar binary in the externalBin array. This tells Tauri to include the binary when bundling the application.

LANGUAGE: json
CODE:
{
  "bundle": {
    "externalBin": ["binaries/app"]
  }
}

----------------------------------------

TITLE: Programmatically Controlling WebView DevTools in Tauri
DESCRIPTION: Example of how to programmatically open and close the WebView developer tools in a Tauri application using Rust code. This example includes a conditional compilation flag to only include this code in debug builds.

LANGUAGE: rust
CODE:
tauri::Builder::default()
  .setup(|app| {
    #[cfg(debug_assertions)] // only include this code on debug builds
    {
      let window = app.get_webview_window("main").unwrap();
      window.open_devtools();
      window.close_devtools();
    }
    Ok(())
  });

----------------------------------------

TITLE: Setting Android Environment Variables on Windows
DESCRIPTION: PowerShell commands to set ANDROID_HOME and NDK_HOME environment variables on Windows, which are required for Android development with Tauri.

LANGUAGE: powershell
CODE:
[System.Environment]::SetEnvironmentVariable("ANDROID_HOME", "$env:LocalAppData\Android\Sdk", "User")
$VERSION = Get-ChildItem -Name "$env:LocalAppData\Android\Sdk\ndk"
[System.Environment]::SetEnvironmentVariable("NDK_HOME", "$env:LocalAppData\Android\Sdk\ndk\$VERSION", "User")

----------------------------------------

TITLE: Adding License in Tauri Configuration
DESCRIPTION: JSON configuration for specifying the license and license file in the tauri.conf.json file.

LANGUAGE: json
CODE:
{
  "bundle": {
    "licenseFile": "../LICENSE", // put the path to the license file here
    "license": "MIT" // add the license here
  }
}

----------------------------------------

TITLE: Removing Files with Tauri FS Plugin
DESCRIPTION: Deletes a file from the filesystem. If the file doesn't exist, this operation will return an error. The example shows how to remove a database file from the local data directory.

LANGUAGE: javascript
CODE:
import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';
await remove('user.db', { baseDir: BaseDirectory.AppLocalData });

----------------------------------------

TITLE: Setting up nvim-dap-ui for Automatic Debugger UI Management
DESCRIPTION: Configuration for the nvim-dap-ui plugin to automatically open and close the debugging UI when debugging sessions start and end.

LANGUAGE: lua
CODE:
local dapui = require("dapui")
dapui.setup()

dap.listeners.before.attach.dapui_config = function()
  dapui.open()
end
dap.listeners.before.launch.dapui_config = function()
  dapui.open()
end
dap.listeners.before.event_terminated.dapui_config = function()
  dapui.close()
end
dap.listeners.before.event_exited.dapui_config = function()
  dapui.close()
end


----------------------------------------

TITLE: Creating Entitlements.plist for App Sandbox and App Identification
DESCRIPTION: XML file that defines app entitlements for the App Store, including App Sandbox capability, application identifier, and team identifier. This is required for all apps distributed through the App Store.

LANGUAGE: xml
CODE:
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <true/>
    <key>com.apple.application-identifier</key>
    <string>$TEAM_ID.$IDENTIFIER</string>
    <key>com.apple.developer.team-identifier</key>
    <string>$TEAM_ID</string>
</dict>
</plist>

----------------------------------------

TITLE: Creating NSIS Installer Hooks in Tauri
DESCRIPTION: Example of defining custom hooks for NSIS installers that execute at different stages of the installation process. The hooks demonstrate displaying message boxes at pre-installation, post-installation, pre-uninstallation, and post-uninstallation phases.

LANGUAGE: nsh
CODE:
!macro NSIS_HOOK_PREINSTALL
  MessageBox MB_OK "PreInstall"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  MessageBox MB_OK "PostInstall"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  MessageBox MB_OK "PreUnInstall"
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  MessageBox MB_OK "PostUninstall"
!macroend

----------------------------------------

TITLE: Saving Files with File Extensions Filter in Rust
DESCRIPTION: Rust example showing how to create a blocking file save dialog with custom file extension filters.

LANGUAGE: rust
CODE:
use tauri_plugin_dialog::DialogExt;

let file_path = app
    .dialog()
    .file()
    .add_filter("My Filter", &["png", "jpeg"])
    .blocking_save_file();
    // do something with the optional file path here
    // the file path is `None` if the user closed the dialog

----------------------------------------

TITLE: Modifying Rust Code Entry Point for Mobile Support
DESCRIPTION: Transforms the main function in the library file to be compatible with mobile platforms using the mobile_entry_point macro, which prepares the function to be executed on mobile devices.

LANGUAGE: rust
CODE:
// src-tauri/src/lib.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // your code here
}

----------------------------------------

TITLE: Writing to NFC Tags in JavaScript
DESCRIPTION: JavaScript code to write NDEF records to NFC tags using the write function. It demonstrates how to create text and URI records and configure the write operation.

LANGUAGE: javascript
CODE:
import { write, textRecord, uriRecord } from '@tauri-apps/plugin-nfc';

const payload = [uriRecord('https://tauri.app'), textRecord('some payload')];

const options = {
  // the kind is only required if you do not have a scanned tag session alive
  // its format is the same as the argument provided to scan()
  kind: {
    type: 'ndef',
  },
  // configure the messages displayed in the "Scan NFC" dialog on iOS
  message: 'Scan a NFC tag',
  successfulReadMessage: 'NFC tag successfully scanned',
  successMessage: 'NFC tag successfully written',
};

await write(payload, options);

----------------------------------------

TITLE: Installing Homebrew Package Manager
DESCRIPTION: This command downloads and executes the Homebrew installation script, which is a prerequisite for installing Cocoapods via Homebrew.

LANGUAGE: sh
CODE:
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

----------------------------------------

TITLE: Getting Directory Metadata with Tauri FS Plugin
DESCRIPTION: Retrieves metadata about a directory using the stat function. This provides information such as creation time and modification time. The stat function follows symlinks.

LANGUAGE: javascript
CODE:
import { stat, BaseDirectory } from '@tauri-apps/plugin-fs';
const metadata = await stat('databases', {
  baseDir: BaseDirectory.AppLocalData,
});

----------------------------------------

TITLE: Implementing Plugin Configuration in Android
DESCRIPTION: Demonstrates how to retrieve and use plugin configuration in an Android Tauri plugin using a Config class and the getConfig method.

LANGUAGE: kotlin
CODE:
import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.TauriPlugin
import app.tauri.annotation.InvokeArg

@InvokeArg
class Config {
    var timeout: Int? = 3000
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
  private var timeout: Int? = 3000

  override fun load(webView: WebView) {
    getConfig(Config::class.java).let {
       this.timeout = it.timeout
    }
  }
}

----------------------------------------

TITLE: Creating a Tauri App with Bun
DESCRIPTION: Command to create a new Tauri application using Bun JavaScript runtime. This uses Bun's create command to initialize a new Tauri project.

LANGUAGE: sh
CODE:
bun create tauri-app

----------------------------------------

TITLE: Allowing Downgrades in Tauri Updater
DESCRIPTION: Code snippet demonstrating how to configure the Tauri updater to allow version downgrades by using a custom version comparator function. The comparator function checks if the update version is different from the current version rather than just greater.

LANGUAGE: rust
CODE:
use tauri_plugin_updater::UpdaterExt;

let update = app
  .updater_builder()
  .version_comparator(|current, update| {
    // default comparison: `update.version > current`
    update.version != current
  })
  .build()?
  .check()
  .await?;

----------------------------------------

TITLE: Configuring Embedded Bootstrapper in Tauri
DESCRIPTION: Configuration for embedding the WebView2 bootstrapper in a Tauri Windows application. This increases installer size by about 1.8MB but improves compatibility with Windows 7 systems.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "webviewInstallMode": {
        "type": "embedBootstrapper"
      }
    }
  }
}

----------------------------------------

TITLE: Configuring File System Plugin in package.json
DESCRIPTION: Adds the @tauri-apps/plugin-fs dependency to package.json for using the file system plugin in JavaScript.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-fs": "^2.0.0"
  }
}

----------------------------------------

TITLE: Adding Dialog Plugin Dependencies in Cargo.toml
DESCRIPTION: Adds the Dialog plugin as a dependency in Cargo.toml for a Rust Tauri application.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-dialog = "2"

----------------------------------------

TITLE: Retrieving WebKit Version on macOS using Terminal
DESCRIPTION: This command extracts the WebKit version from the Info.plist file on macOS. It allows developers to determine which WebKit version is being used by WKWebView on their specific macOS installation by parsing the CFBundleVersion from the WebKit framework's Info.plist file.

LANGUAGE: shell
CODE:
awk '/CFBundleVersion/{getline;gsub(/<[^>]*>/,"");print}' /System/Library/Frameworks/WebKit.framework/Resources/Info.plist

----------------------------------------

TITLE: Adding Dialog Plugin Dependencies in Cargo.toml
DESCRIPTION: Adds the Dialog plugin as a dependency in Cargo.toml for a Rust Tauri application.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-dialog = "2"

----------------------------------------

TITLE: Calling Mobile Commands from Rust in Tauri Plugin
DESCRIPTION: Demonstrates how to call a mobile command from Rust code in a Tauri plugin using the PluginHandle, including serialization/deserialization of request and response types.

LANGUAGE: rust
CODE:
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri::Runtime;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraRequest {
  quality: usize,
  allow_edit: bool,
}

#[derive(Deserialize)]
pub struct Photo {
  path: PathBuf,
}


impl<R: Runtime> <plugin-name;pascal-case><R> {
  pub fn open_camera(&self, payload: CameraRequest) -> crate::Result<Photo> {
    self
      .0
      .run_mobile_plugin("openCamera", payload)
      .map_err(Into::into)
  }
}

----------------------------------------

TITLE: Setting RPM Signing Key Environment Variable
DESCRIPTION: Command to export the GPG signing key as an environment variable for use with Tauri's RPM packaging.

LANGUAGE: bash
CODE:
export TAURI_SIGNING_RPM_KEY=$(cat /home/johndoe/my_super_private.key)

----------------------------------------

TITLE: Exporting Public GPG Key for RPM Verification
DESCRIPTION: Command to export the public GPG key for verifying signed RPM packages.

LANGUAGE: bash
CODE:
gpg --export -a 'Tauri-App' > RPM-GPG-KEY-Tauri-App

----------------------------------------

TITLE: Configuring Multiple Windows in Tauri Configuration File
DESCRIPTION: JSON snippet showing how to define multiple windows in the tauri.conf.json file with different labels, titles, and dimensions.

LANGUAGE: javascript
CODE:
  "productName": "multiwindow",
  ...
  "app": {
    "windows": [
      {
        "label": "first",
        "title": "First",
        "width": 800,
        "height": 600
      },
      {
        "label": "second",
        "title": "Second",
        "width": 800,
        "height": 600
      }
    ],
  },
  ...
}

----------------------------------------

TITLE: Creating TypeScript Binding for Rust Command
DESCRIPTION: This TypeScript code defines a binding function in webview-src/index.ts that allows plugin users to easily call the Rust command from JavaScript, handling progress updates through a Channel.

LANGUAGE: javascript
CODE:
import { invoke, Channel } from '@tauri-apps/api/core'

export async function upload(url: string, onProgressHandler: (progress: number) => void): Promise<void> {
  const onProgress = new Channel<number>()
  onProgress.onmessage = onProgressHandler
  await invoke('plugin:<plugin-name>|upload', { url, onProgress })
}

----------------------------------------

TITLE: Disabling SSR in SvelteKit for Tauri Compatibility
DESCRIPTION: TypeScript configuration that disables server-side rendering and enables prerendering for a SvelteKit application, which is necessary for using Tauri's client-side APIs.

LANGUAGE: ts
CODE:
// src/routes/+layout.ts
export const prerender = true;
export const ssr = false;

----------------------------------------

TITLE: Complete Updater Configuration in tauri.conf.json
DESCRIPTION: Full JSON configuration for the Tauri updater plugin, including public key, update endpoints, and artifact creation settings. The endpoints can use dynamic variables to customize update URLs.

LANGUAGE: json
CODE:
{
  "bundle": {
    "createUpdaterArtifacts": true
  },
  "plugins": {
    "updater": {
      "pubkey": "CONTENT FROM PUBLICKEY.PEM",
      "endpoints": [
        "https://releases.myapp.com/{{target}}/{{arch}}/{{current_version}}",
        // or a static github json file
        "https://github.com/user/repo/releases/latest/download/latest.json"
      ]
    }
  }
}

----------------------------------------

TITLE: Adding Windows MSVC Target to Rust Toolchain
DESCRIPTION: Command to add the 64-bit Windows MSVC target to Rust, enabling compilation for Windows systems.

LANGUAGE: sh
CODE:
rustup target add x86_64-pc-windows-msvc

----------------------------------------

TITLE: Restoring window state in JavaScript
DESCRIPTION: JavaScript code example demonstrating how to manually restore a window's state from disk using the restoreStateCurrent function from the window-state plugin.

LANGUAGE: javascript
CODE:
import {
  restoreStateCurrent,
  StateFlags,
} from '@tauri-apps/plugin-window-state';
// when using `"withGlobalTauri": true`, you may use
// const { restoreStateCurrent, StateFlags } = window.__TAURI__.windowState;

restoreStateCurrent(StateFlags.ALL);

----------------------------------------

TITLE: Using Plugin Permissions in JavaScript
DESCRIPTION: Shows how to check and request permissions from a Tauri plugin using JavaScript. This example handles different permission states and demonstrates the pattern for requesting permissions when needed.

LANGUAGE: javascript
CODE:
import { invoke, PermissionState } from '@tauri-apps/api/core'

interface Permissions {
  postNotification: PermissionState
}

// check permission state
const permission = await invoke<Permissions>('plugin:<plugin-name>|checkPermissions')

if (permission.postNotification === 'prompt-with-rationale') {
  // show information to the user about why permission is needed
}

// request permission
if (permission.postNotification.startsWith('prompt')) {
  const state = await invoke<Permissions>('plugin:<plugin-name>|requestPermissions', { permissions: ['postNotification'] })
}

----------------------------------------

TITLE: Visualizing Event Communication Flow in Tauri Using D2
DESCRIPTION: A sequence diagram illustrating the bidirectional event communication between the Webview Frontend and Core Backend. Shows how events can be sent in both directions as fire-and-forget messages.

LANGUAGE: d2
CODE:
shape: sequence_diagram

Frontend: {
  shape: rectangle
  label: "Webview\nFrontend"
}
Core: {
  shape: rectangle
  label: "Core\nBackend"
}

Frontend -> Core: "Event"{style.animated: true}
Core -> Frontend: "Event"{style.animated: true}

----------------------------------------

TITLE: Setting RPM Signing Key Passphrase
DESCRIPTION: Command to export the GPG signing key passphrase as an environment variable for use with Tauri's RPM packaging.

LANGUAGE: bash
CODE:
export TAURI_SIGNING_RPM_KEY_PASSPHRASE=password

----------------------------------------

TITLE: Exposing Command to Frontend in Plugin Initialization
DESCRIPTION: Updates to the plugin initialization code to register the new 'write_custom_file' command with Tauri's invoke handler so it can be called from the frontend.

LANGUAGE: rust
CODE:
pub fn init<R: Runtime>() -> TauriPlugin<R> {
Builder::new("test")
    .invoke_handler(tauri::generate_handler![
        commands::ping,
        commands::write_custom_file,
    ])
    .setup(|app, api| {
        #[cfg(mobile)]
        let test = mobile::init(app, api)?;
        #[cfg(desktop)]
        let test = desktop::init(app, api)?;
        app.manage(test);

        // manage state so it is accessible by the commands
        app.manage(MyState::default());
        Ok(())
    })
    .build()
}

----------------------------------------

TITLE: Defining Global Scope Permissions in TOML
DESCRIPTION: This TOML configuration defines a global scope permission that permits spawning the node binary, illustrating how to create permissions that apply globally rather than to specific commands.

LANGUAGE: toml
CODE:
[[permission]]
identifier = "allow-spawn-node"
description = "This scope permits spawning the `node` binary."

[[permission.scope.allow]]
binary = "node"

----------------------------------------

TITLE: Creating TypeScript Bindings for the Plugin Command
DESCRIPTION: TypeScript function that exports the 'writeCustomFile' function to the frontend, allowing JavaScript/TypeScript code to invoke the Rust command through Tauri's IPC bridge.

LANGUAGE: typescript
CODE:
import { invoke } from '@tauri-apps/api/core'

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:test|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function writeCustomFile(user_input: string): Promise<string> {
 return await invoke('plugin:test|write_custom_file',{userInput: user_input});
}

----------------------------------------

TITLE: Adding a Dedicated Menu Click Handler in JavaScript
DESCRIPTION: Shows how to define inline action handlers for menu items in JavaScript using arrow functions for direct event handling.

LANGUAGE: javascript
CODE:
import { Menu } from '@tauri-apps/api/menu';

const menu = await Menu.new({
  items: [
    {
      id: 'quit',
      text: 'Quit',
      action: () => {
        console.log('quit pressed');
      },
    },
  ],
});

----------------------------------------

TITLE: Configuring Stronghold Plugin Permissions
DESCRIPTION: Sample JSON configuration for enabling the Stronghold plugin permissions in the capabilities file.

LANGUAGE: json
CODE:
{
	...,
	"permissions": [
		"stronghold:default",
	]
}

----------------------------------------

TITLE: Installing snapcraft tool
DESCRIPTION: Installs the snapcraft tool with classic confinement, which is required to build snap packages.

LANGUAGE: shell
CODE:
sudo snap install snapcraft --classic

----------------------------------------

TITLE: Configuring Tauri with pnpm for Nuxt Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using pnpm as package manager. Defines build commands, development URL, and frontend distribution path for Nuxt integration.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Listening to Plugin Events in JavaScript
DESCRIPTION: Shows how to listen for events emitted by a Tauri plugin using JavaScript. This helper function uses the addPluginListener API to subscribe to plugin events.

LANGUAGE: javascript
CODE:
import { addPluginListener, PluginListener } from '@tauri-apps/api/core';

export async function onRequest(
	handler: (url: string) => void
): Promise<PluginListener> {
	return await addPluginListener(
		'<plugin-name>',
		'event-name',
		handler
	);
}

----------------------------------------

TITLE: Adding a Menu to Tray Icon in JavaScript
DESCRIPTION: Demonstrates creating a menu with a quit option and attaching it to a tray icon in JavaScript. The menu is configured to appear on left clicks.

LANGUAGE: javascript
CODE:
import { TrayIcon } from '@tauri-apps/api/tray';
import { Menu } from '@tauri-apps/api/menu';

const menu = await Menu.new({
  items: [
    {
      id: 'quit',
      text: 'Quit',
    },
  ],
});

const options = {
  menu,
  menuOnLeftClick: true,
};

const tray = await TrayIcon.new(options);

----------------------------------------

TITLE: Generating Schema in build.rs
DESCRIPTION: This build script configures the plugin builder to generate a JSON schema for the global scope, which helps plugin consumers understand the format of the scope and provides autocomplete in IDEs.

LANGUAGE: rust
CODE:
#[path = "src/scope.rs"]
mod scope;

const COMMANDS: &[&str] = &[];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .global_scope_schema(schemars::schema_for!(scope::Entry))
        .build();
}

----------------------------------------

TITLE: RPM Pre-Removal Script Template
DESCRIPTION: Sample shell script for the pre-removal hook that shows how to access uninstallation parameters.

LANGUAGE: bash
CODE:
echo "-------------"
echo "This is preun"
echo "Install Value: $1"
echo "Upgrade Value: $1"
echo "Uninstall Value: $1"
echo "-------------"

----------------------------------------

TITLE: Importing Public GPG Key to RPM Database
DESCRIPTION: Command to import the public GPG key into the RPM database for package verification.

LANGUAGE: bash
CODE:
sudo rpm --import RPM-GPG-KEY-Tauri-App

----------------------------------------

TITLE: Implementing Frontend Setup Task Simulation
DESCRIPTION: TypeScript code that simulates a heavy frontend setup task using a setTimeout-based sleep function and communicates completion to the backend via Tauri's invoke API.

LANGUAGE: javascript
CODE:
// src/main.ts
// These contents can be copy-pasted below the existing code, don't replace the entire file!!

// Utility function to implement a sleep function in TypeScript
function sleep(seconds: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, seconds * 1000));
}

// Setup function
async function setup() {
    // Fake perform some really heavy setup task
    console.log('Performing really heavy frontend setup task...')
    await sleep(3);
    console.log('Frontend setup task complete!')
    // Set the frontend task as being completed
    invoke('set_complete', {task: 'frontend'})
}

// Effectively a JavaScript main function
window.addEventListener("DOMContentLoaded", () => {
    setup()
});

----------------------------------------

TITLE: Accessing Plugin APIs in Tauri Application
DESCRIPTION: Demonstrates how to access a plugin's exported APIs using the extension trait pattern, showing an example with the global-shortcut plugin.

LANGUAGE: rust
CODE:
use tauri_plugin_global_shortcut::GlobalShortcutExt;

tauri::Builder::default()
  .plugin(tauri_plugin_global_shortcut::init())
  .setup(|app| {
    app.global_shortcut().register(...);
    Ok(())
  })

----------------------------------------

TITLE: Handling Menu Events in Rust
DESCRIPTION: Demonstrates how to listen for and handle menu click events in Rust using the on_menu_event method. The handler uses pattern matching to respond to specific menu IDs.

LANGUAGE: rust
CODE:
use tauri::tray::TrayIconBuilder;

TrayIconBuilder::new()
  .on_menu_event(|app, event| match event.id.as_ref() {
    "quit" => {
      println!("quit menu item was clicked");
      app.exit(0);
    }
    _ => {
      println!("menu item {:?} not handled", event.id);
    }
  })

----------------------------------------

TITLE: Initializing Stronghold Plugin in Rust
DESCRIPTION: Basic setup for adding the Stronghold plugin to a Tauri application in the lib.rs file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
		tauri::Builder::default()
				.plugin(tauri_plugin_stronghold::Builder::new(|password| {}).build())
				.run(tauri::generate_context!())
				.expect("error while running tauri application");
}

----------------------------------------

TITLE: Installing core22 snap base
DESCRIPTION: Installs the core22 base snap, which provides the runtime environment for snaps based on Ubuntu 22.04.

LANGUAGE: shell
CODE:
sudo snap install core22

----------------------------------------

TITLE: TOML Configuration Format Example in Tauri
DESCRIPTION: Example of the new TOML configuration format supported in Tauri 1.1.0 with the config-toml Cargo feature. Shows how to configure build settings in TOML syntax.

LANGUAGE: toml
CODE:
[build]
dev-path = "http://localhost:8000"
dist-dir = "../dist"

----------------------------------------

TITLE: Implementing Global Scope Handling
DESCRIPTION: This Rust function shows how to access and use global scope information in a Tauri command, providing access to entries that are globally allowed or denied across the plugin.

LANGUAGE: rust
CODE:
use tauri::ipc::GlobalScope;
use crate::scope::Entry;

async fn spawn<R: tauri::Runtime>(app: tauri::AppHandle<R>, scope: GlobalScope<'_, Entry>) -> Result<()> {
  let allowed = scope.allows();
  let denied = scope.denies();
  todo!()
}

----------------------------------------

TITLE: Using Type Aliases to Prevent State Mismatches in Tauri
DESCRIPTION: Shows how to create a type alias for state to prevent runtime panics caused by using the wrong type when accessing state. This ensures consistent state type usage throughout the application.

LANGUAGE: rust
CODE:
use std::sync::Mutex;

#[derive(Default)]
struct AppStateInner {
  counter: u32,
}

type AppState = Mutex<AppStateInner>;

----------------------------------------

TITLE: Listing Dependent RPM Packages
DESCRIPTION: Command to list all packages that depend on a specific RPM package.

LANGUAGE: bash
CODE:
rpm -q --whatrequires package_name.rpm

----------------------------------------

TITLE: Adding Window Permissions in Capabilities JSON
DESCRIPTION: JSON configuration for window permissions in Tauri's capabilities system, enabling window control operations needed for custom titlebars.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["core:window:default", "core:window:allow-start-dragging"]
}

----------------------------------------

TITLE: Listening to Global Events in TypeScript with Tauri
DESCRIPTION: Demonstrates how to listen to global events in a Tauri application using TypeScript. The example shows listening to a 'download-started' event with typed payload data containing URL, download ID, and content length information.

LANGUAGE: typescript
CODE:
import { listen } from '@tauri-apps/api/event';

type DownloadStarted = {
  url: string;
  downloadId: number;
  contentLength: number;
};

listen<DownloadStarted>('download-started', (event) => {
  console.log(
    `downloading ${event.payload.contentLength} bytes from ${event.payload.url}`
  );
});

----------------------------------------

TITLE: Configuring iOS Camera Usage Description for Barcode Scanner
DESCRIPTION: XML configuration for Info.ios.plist file that adds the required NSCameraUsageDescription property, explaining why the app needs camera access for reading QR codes.

LANGUAGE: xml
CODE:
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
	<dict>
		<key>NSCameraUsageDescription</key>
		<string>Read QR codes</string>
	</dict>
</plist>

----------------------------------------

TITLE: Configuring Windows in Tauri Configuration File
DESCRIPTION: JSON configuration for registering multiple windows in the Tauri application, including a hidden main window and a visible splashscreen window.

LANGUAGE: json
CODE:
// src-tauri/tauri.conf.json
{
    "windows": [
        {
            "label": "main",
            "visible": false
        },
        {
            "label": "splashscreen",
            "url": "/splashscreen"
        }
    ]
}

----------------------------------------

TITLE: Configuring Shell Plugin Permissions in Capabilities
DESCRIPTION: JSON configuration for setting up security permissions to allow shell command execution in a Tauri application.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "shell:allow-execute",
      "allow": [
        {
          "name": "exec-sh",
          "cmd": "sh",
          "args": [
            "-c",
            {
              "validator": "\\S+"
            }
          ],
          "sidecar": false
        }
      ]
    }
  ]
}

----------------------------------------

TITLE: Listing RPM Package Contents
DESCRIPTION: Command to list all files included in an RPM package, useful for verification and debugging.

LANGUAGE: bash
CODE:
rpm -qlp package_name.rpm

----------------------------------------

TITLE: Installing Notification Plugin Dependencies in Package.json
DESCRIPTION: Command to add the notification plugin dependency to your project using your package manager of choice.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-notification

----------------------------------------

TITLE: Implementing Command-Specific Scope Handling
DESCRIPTION: This Rust function demonstrates how to access and use command-specific scope information in a Tauri command, allowing access to allowed and denied entries for that command.

LANGUAGE: rust
CODE:
use tauri::ipc::CommandScope;
use crate::scope::Entry;

async fn spawn<R: tauri::Runtime>(app: tauri::AppHandle<R>, command_scope: CommandScope<'_, Entry>) -> Result<()> {
  let allowed = command_scope.allows();
  let denied = command_scope.denies();
  todo!()
}

----------------------------------------

TITLE: Configuring Tauri with Deno for SvelteKit Integration
DESCRIPTION: JSON configuration for Tauri when using Deno as the runtime. Specifies the build tasks and frontend distribution path for a SvelteKit project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "deno task dev",
    "beforeBuildCommand": "deno task build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}

----------------------------------------

TITLE: Defining Basic Permission Structure in TOML
DESCRIPTION: Shows the basic structure of a permission configuration in TOML format. It includes defining an identifier, description, allowed commands, and scope settings with allow and deny rules.

LANGUAGE: toml
CODE:
[[permission]]
identifier = "my-identifier"
description = "This describes the impact and more."
commands.allow = [
    "read_file"
]

[[scope.allow]]
my-scope = "$HOME/*"

[[scope.deny]]
my-scope = "$HOME/secret"

----------------------------------------

TITLE: Listening to Global Events in Rust with Tauri
DESCRIPTION: Demonstrates how to listen to global events in the Rust backend of a Tauri application. The example shows listening to a 'download-started' event and deserializing the JSON payload using serde_json.

LANGUAGE: rust
CODE:
use tauri::Listener;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      app.listen("download-started", |event| {
        if let Ok(payload) = serde_json::from_str::<DownloadStarted>(&event.payload()) {
          println!("downloading {}", payload.url);
        }
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

----------------------------------------

TITLE: Creating Script Directory in Tauri Project
DESCRIPTION: Command to create a scripts directory in the src-tauri folder for storing RPM package scripts.

LANGUAGE: bash
CODE:
mkdir src-tauri/scripts

----------------------------------------

TITLE: Configuring Tauri with Deno for SvelteKit Integration
DESCRIPTION: JSON configuration for Tauri when using Deno as the runtime. Specifies the build tasks and frontend distribution path for a SvelteKit project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "deno task dev",
    "beforeBuildCommand": "deno task build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}

----------------------------------------

TITLE: Configuring Single Instance Plugin with Deep Link
DESCRIPTION: Cargo.toml configuration to add deep-link feature to the single instance plugin for desktop platforms.

LANGUAGE: toml
CODE:
[target."cfg(any(target_os = \"macos\", windows, target_os = \"linux\"))".dependencies]
tauri-plugin-single-instance = { version = "2.0.0", features = ["deep-link"] }

----------------------------------------

TITLE: Listening to Events Once in Rust with Tauri
DESCRIPTION: Shows how to listen to an event exactly once in the Rust backend of a Tauri application. The 'once' method automatically unregisters the event listener after it's triggered the first time.

LANGUAGE: rust
CODE:
app.once("ready", |event| {
  println!("app is ready");
});

----------------------------------------

TITLE: Executing a Node.js Sidecar from JavaScript in Tauri
DESCRIPTION: JavaScript code using Tauri's shell plugin to execute the Node.js sidecar binary. It sends a 'ping' command with a message and captures the sidecar's response.

LANGUAGE: javascript
CODE:
import { Command } from '@tauri-apps/plugin-shell';

const message = 'Tauri';

const command = Command.sidecar('binaries/app', ['ping', message]);
const output = await command.execute();
const response = output.stdout;

----------------------------------------

TITLE: Installing OS Plugin NPM Package
DESCRIPTION: Commands to install the OS Information plugin NPM package for JavaScript usage.

LANGUAGE: sh
CODE:
npm install @tauri-apps/plugin-os

LANGUAGE: sh
CODE:
yarn add @tauri-apps/plugin-os

LANGUAGE: sh
CODE:
pnpm add @tauri-apps/plugin-os

LANGUAGE: sh
CODE:
deno add npm:@tauri-apps/plugin-os

LANGUAGE: sh
CODE:
bun add @tauri-apps/plugin-os

----------------------------------------

TITLE: Listing Script Files
DESCRIPTION: Command to list the created script files in the src-tauri/scripts directory.

LANGUAGE: bash
CODE:
ls src-tauri/scripts/
postinstall.sh  postremove.sh  preinstall.sh  preremove.sh

----------------------------------------

TITLE: Adding File System Plugin Dependencies in Cargo.toml
DESCRIPTION: Adds the File System plugin as a dependency in Cargo.toml for a Rust Tauri application.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-fs = "2"

----------------------------------------

TITLE: Using Window Plugin in Rust
DESCRIPTION: This snippet shows how to use the window plugin in Rust to manipulate window properties. It demonstrates getting a reference to a window by name and setting its title.

LANGUAGE: rust
CODE:
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_title("Tauri")?;
            Ok(())
        })
}

----------------------------------------

TITLE: Adding Write Permission to capabilities/default.json
DESCRIPTION: JSON modification to add file system write permission to the default capabilities configuration. This change adds the 'fs:allow-write-text-file' permission to enable writing to text files.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": [
    "main"
  ],
  "permissions": [
    "path:default",
    "event:default",
    "window:default",
    "app:default",
    "image:default",
    "resources:default",
    "menu:default",
    "tray:default",
    "shell:allow-open",
    "fs:default",
    "fs:allow-write-text-file",
  ]
}

----------------------------------------

TITLE: Installing snap on Debian
DESCRIPTION: Installs the snapd package manager on Debian-based distributions using apt.

LANGUAGE: shell
CODE:
sudo apt install snapd

----------------------------------------

TITLE: Configuring Deep Link in Tauri Config
DESCRIPTION: JSON configuration for defining deep link domains and schemes in tauri.conf.json.

LANGUAGE: json
CODE:
{
  "plugins": {
    "deep-link": {
      "mobile": [
        { "host": "your.website.com", "pathPrefix": ["/open"] },
        { "host": "another.site.br" }
      ],
      "desktop": {
        "schemes": ["something", "my-tauri-app"]
      }
    }
  }
}

----------------------------------------

TITLE: Advanced Error Handling with Custom Serialization in Tauri
DESCRIPTION: Implements a more advanced error handling approach with custom serialization for different error types. This provides better control over how errors are presented to the frontend.

LANGUAGE: rust
CODE:
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error("failed to parse as string: {0}")]
  Utf8(#[from] std::str::Utf8Error),
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
  Io(String),
  Utf8(String),
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    let error_message = self.to_string();
    let error_kind = match self {
      Self::Io(_) => ErrorKind::Io(error_message),
      Self::Utf8(_) => ErrorKind::Utf8(error_message),
    };
    error_kind.serialize(serializer)
  }
}

#[tauri::command]
fn read() -> Result<Vec<u8>, Error> {
  let data = std::fs::read("/path/to/file")?;
	Ok(data)
}

----------------------------------------

TITLE: Configuring Webview Console Log Target with JavaScript Attachment
DESCRIPTION: Configuration to direct logs to the webview console, combined with the JavaScript code to attach the console to the log stream.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .target(tauri_plugin_log::Target::new(
    tauri_plugin_log::TargetKind::Webview,
  ))
  .build()

----------------------------------------

TITLE: Implementing Basic Node.js Sidecar Logic in JavaScript
DESCRIPTION: A simple Node.js application that processes command line arguments and responds accordingly. It handles a 'ping' command and returns a customized 'pong' message with the provided input.

LANGUAGE: javascript
CODE:
const command = process.argv[2];

switch (command) {
  case 'ping':
    const message = process.argv[3];
    console.log(`pong, ${message}`);
    break;
  default:
    console.error(`unknown command ${command}`);
    process.exit(1);
}

----------------------------------------

TITLE: Adding NFC Capability in iOS Entitlements
DESCRIPTION: XML configuration for iOS entitlements file to enable NFC tag reading capability, required for NFC functionality on iOS devices.

LANGUAGE: xml
CODE:
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>com.apple.developer.nfc.readersession.formats</key>
	<array>
		<string>TAG</string>
	</array>
</dict>
</plist>

----------------------------------------

TITLE: Publishing an AUR package with Git commands
DESCRIPTION: Git commands for publishing a package to the Arch User Repository. These commands add files to the staging area, commit them with a message, and push the changes to the AUR.

LANGUAGE: sh
CODE:
git add .

git commit -m "Initial Commit"

git push

----------------------------------------

TITLE: Using Notification Plugin in Rust
DESCRIPTION: Shows how to send notifications with permission handling using the Notification plugin in a Rust-based Tauri application.

LANGUAGE: rust
CODE:
use tauri_plugin_notification::NotificationExt;
use tauri::plugin::PermissionState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            if app.notification().permission_state()? == PermissionState::Unknown {
                app.notification().request_permission()?;
            }
            if app.notification().permission_state()? == PermissionState::Granted {
                app.notification()
                    .builder()
                    .body("Tauri is awesome!")
                    .show()?;
            }
            Ok(())
        })
}

----------------------------------------

TITLE: Declaring Window Plugin in package.json
DESCRIPTION: This shows how to add the Tauri window plugin as a dependency in your package.json file for JavaScript projects. This enables frontend code to interact with window management functionality.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-window": "^2.0.0"
  }
}

----------------------------------------

TITLE: Initializing the Shell Plugin in Rust
DESCRIPTION: Modification of the lib.rs file to initialize the shell plugin in a Tauri application.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring Resource Files in tauri.conf.json with Array Notation
DESCRIPTION: Basic configuration example for adding resources to a Tauri application using an array of file paths. This allows including absolute paths, relative paths, and glob patterns for bundling files with the application.

LANGUAGE: json
CODE:
{
  "bundle": {
    "resources": [
      "/absolute/path/to/textfile.txt",
      "relative/path/to/jsonfile.json",
      "resources/**/*"
    ]
  }
}

----------------------------------------

TITLE: Installing snap on Fedora
DESCRIPTION: Installs the snapd package manager on Fedora using dnf and enables classic snap support by creating a symbolic link. Requires system reboot after installation.

LANGUAGE: shell
CODE:
sudo dnf install snapd
# Enable classic snap support
sudo ln -s /var/lib/snapd/snap /snap

----------------------------------------

TITLE: Adding Windows 7 Notification Support in Tauri
DESCRIPTION: Cargo.toml configuration to enable Windows 7 compatibility for notifications in Tauri applications.

LANGUAGE: toml
CODE:
[dependencies]
tauri-plugin-notification = { version = "2.0.0", features = [ "windows7-compat" ] }

----------------------------------------

TITLE: Extracting Target Triple with PowerShell on Windows
DESCRIPTION: PowerShell command to extract the target triple from rustc output on Windows systems.

LANGUAGE: powershell
CODE:
rustc -Vv | Select-String "host:" | ForEach-Object {$_.Line.split(" ")[1]}

----------------------------------------

TITLE: Implementing File Write Functionality in TypeScript
DESCRIPTION: TypeScript code that implements file writing functionality using the Tauri fs plugin. This code writes the input from a form to a text file in the user's home directory.

LANGUAGE: typescript
CODE:
import { writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

let greetInputEl: HTMLInputElement | null;

async function write(message: string) {
    await writeTextFile('test.txt', message, { baseDir: BaseDirectory.Home });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    if (!greetInputEl )
      return;

    write(greetInputEl.value == "" ? "No input provided": greetInputEl.value);

  });
});

----------------------------------------

TITLE: Configuring iOS Info.plist for NFC
DESCRIPTION: XML configuration for the iOS Info.plist file to add NFC reader usage description, which is required to scan or write to NFC tags on iOS devices.

LANGUAGE: xml
CODE:
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
	<dict>
		<key>NFCReaderUsageDescription</key>
		<string>Read and write various NFC tags</string>
	</dict>
</plist>

----------------------------------------

TITLE: Configuring Core Plugin Permissions in Tauri 2.0 Beta (Old Format)
DESCRIPTION: JSON configuration showing how core plugin permissions were specified in Tauri 2.0 beta version. This format lists individual permissions like 'path:default', 'window:default', etc., which will be deprecated in the release candidate.

LANGUAGE: json
CODE:
...\n\"permissions\": [\n    \"path:default\",\n    \"event:default\",\n    \"window:default\",\n    \"app:default\",\n    \"image:default\",\n    \"resources:default\",\n    \"menu:default\",\n    \"tray:default\"\n]\n...

----------------------------------------

TITLE: Updating Tauri Dependencies for Version 1.7.0
DESCRIPTION: Commands to update both NPM and Cargo dependencies to the latest Tauri 1.7.0 release. Shows options for different package managers including npm, yarn, pnpm, and cargo.

LANGUAGE: bash
CODE:
npm install @tauri-apps/cli@latest @tauri-apps/api@latest

LANGUAGE: bash
CODE:
yarn upgrade @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
pnpm update @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
cargo update

----------------------------------------

TITLE: Configuring Custom Path Scope for fs Plugin
DESCRIPTION: JSON configuration to add a custom path scope for the file system write permission. This allows write access to a specific file ('test.txt') in the user's home directory.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": [
    "main"
  ],
  "permissions": [
    "path:default",
    "event:default",
    "window:default",
    "app:default",
    "image:default",
    "resources:default",
    "menu:default",
    "tray:default",
    "shell:allow-open",
    "fs:allow-write-text-file",
    {
      "identifier": "fs:allow-write-text-file",
      "allow": [{ "path": "$HOME/test.txt" }]
    },
  ]
}

----------------------------------------

TITLE: Configuring Extended tasks.json for Windows Tauri Debugging
DESCRIPTION: This enhanced tasks.json file groups build and UI development tasks together for Windows debugging. It adds a cargo build task and a task group to ensure proper compilation before launching the application.

LANGUAGE: json
CODE:
{
  // See https://go.microsoft.com/fwlink/?LinkId=733558
  // for the documentation about the tasks.json format
  "version": "2.0.0",
  "tasks": [
    {
      "label": "build:debug",
      "type": "cargo",
      "command": "build"
    },
    {
      "label": "ui:dev",
      "type": "shell",
      // `dev` keeps running in the background
      // ideally you should also configure a `problemMatcher`
      // see https://code.visualstudio.com/docs/editor/tasks#_can-a-background-task-be-used-as-a-prelaunchtask-in-launchjson
      "isBackground": true,
      // change this to your `beforeDevCommand`:
      "command": "yarn",
      "args": ["dev"]
    },
    {
      "label": "dev",
      "dependsOn": ["build:debug", "ui:dev"],
      "group": {
        "kind": "build"
      }
    }
  ]
}

----------------------------------------

TITLE: Testing a Snap Package
DESCRIPTION: Command to run and test a snap package locally before publishing.

LANGUAGE: shell
CODE:
snap run your-app

----------------------------------------

TITLE: Updating Cargo.toml Dependencies for Tauri
DESCRIPTION: Example of how to update the tauri and tauri-build dependencies in the src-tauri/Cargo.toml file. Replace %version% with the desired version number for both dependencies.

LANGUAGE: toml
CODE:
[build-dependencies]
tauri-build = "%version%"

[dependencies]
tauri = { version = "%version%" }

----------------------------------------

TITLE: Updated Vite Configuration for Development Server in Tauri 2.0 RC
DESCRIPTION: JavaScript configuration for Vite development server in Tauri 2.0 RC, using the new TAURI_DEV_HOST environment variable instead of platform detection and internal-ip package.

LANGUAGE: js
CODE:
import { defineConfig } from 'vite';\nimport Unocss from 'unocss/vite';\nimport { svelte } from '@sveltejs/vite-plugin-svelte';\n\nconst host = process.env.TAURI_DEV_HOST;\n\nexport default defineConfig({\n  plugins: [svelte()],\n  clearScreen: false,\n  server: {\n    host: host || false,\n    port: 1420,\n    strictPort: true,\n    hmr: host\n      ? {\n          protocol: 'ws',\n          host: host,\n          port: 1430,\n        }\n      : undefined,\n  },\n});

----------------------------------------

TITLE: Default Permissions Configuration for fs Plugin
DESCRIPTION: The default permissions configuration for the file system plugin, defined in TOML format. This shows the default permissions granted to the plugin, including read access and scope limitations.

LANGUAGE: toml
CODE:
"$schema" = "schemas/schema.json"

[default]
description = """
# Tauri `fs` default permissions

This configuration file defines the default permissions granted
to the filesystem.

### Granted Permissions

This default permission set enables all read-related commands and
allows access to the `$APP` folder and sub directories created in it.
The location of the `$APP` folder depends on the operating system,
where the application is run.

In general the `$APP` folder needs to be manually created
by the application at runtime, before accessing files or folders
in it is possible.

### Denied Permissions

This default permission set prevents access to critical components
of the Tauri application by default.
On Windows the webview data folder access is denied.

"""
permissions = ["read-all", "scope-app-recursive", "deny-default"]


----------------------------------------

TITLE: Registering Notification Action Types in JavaScript
DESCRIPTION: Code to register notification action types with interactive elements like reply buttons and mark-as-read actions.

LANGUAGE: javascript
CODE:
import { registerActionTypes } from '@tauri-apps/plugin-notification';

await registerActionTypes([
  {
    id: 'messages',
    actions: [
      {
        id: 'reply',
        title: 'Reply',
        input: true,
        inputButtonTitle: 'Send',
        inputPlaceholder: 'Type your reply...',
      },
      {
        id: 'mark-read',
        title: 'Mark as Read',
        foreground: false,
      },
    ],
  },
]);

----------------------------------------

TITLE: Updating Tauri Dependencies with Package Managers
DESCRIPTION: Command line instructions for upgrading Tauri CLI and API dependencies to the latest version (1.6.0) using different package managers and updating Cargo dependencies.

LANGUAGE: bash
CODE:
npm install @tauri-apps/cli@latest @tauri-apps/api@latest

LANGUAGE: bash
CODE:
yarn upgrade @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
pnpm update @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
cargo update

----------------------------------------

TITLE: Customizing Breakpoint Appearance in Neovim
DESCRIPTION: Customization of how breakpoints and execution points are displayed in the editor using emoji icons.

LANGUAGE: lua
CODE:
vim.fn.sign_define('DapBreakpoint',{ text ='🟥', texthl ='', linehl ='', numhl =''})
vim.fn.sign_define('DapStopped',{ text ='▶️', texthl ='', linehl ='', numhl =''})


----------------------------------------

TITLE: Initializing Notification Plugin in Rust for Tauri
DESCRIPTION: Initializes the notification plugin in a Tauri Rust application builder.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
}

----------------------------------------

TITLE: Cloning the Flathub Repository for App Submission
DESCRIPTION: Command to clone a forked Flathub repository for submitting a new application. Uses the new-pr branch which is the proper branch for new application submissions.

LANGUAGE: shell
CODE:
git clone --branch=new-pr git@github.com:your_github_username/flathub.git

----------------------------------------

TITLE: Updated Vite Configuration for Development Server in Tauri 2.0 RC
DESCRIPTION: JavaScript configuration for Vite development server in Tauri 2.0 RC, using the new TAURI_DEV_HOST environment variable instead of platform detection and internal-ip package.

LANGUAGE: js
CODE:
import { defineConfig } from 'vite';\nimport Unocss from 'unocss/vite';\nimport { svelte } from '@sveltejs/vite-plugin-svelte';\n\nconst host = process.env.TAURI_DEV_HOST;\n\nexport default defineConfig({\n  plugins: [svelte()],\n  clearScreen: false,\n  server: {\n    host: host || false,\n    port: 1420,\n    strictPort: true,\n    hmr: host\n      ? {\n          protocol: 'ws',\n          host: host,\n          port: 1430,\n        }\n      : undefined,\n  },\n});

----------------------------------------

TITLE: Configuring External API Access in Tauri 1.3.0
DESCRIPTION: JSON configuration for the new dangerousRemoteUrlIpcAccess security feature in Tauri 1.3.0. This allows specific trusted domains to access Tauri IPC layer with fine-grained controls for windows, plugins, and API access.

LANGUAGE: json
CODE:
"security": {
  "dangerousRemoteUrlIpcAccess": [
    {
      "windows": ["main", "settings"],
      "domain": "trusted.example",
      "plugins": ["trusted-plugin"],
      "enableTauriAPI": false
    },
  ],
}

----------------------------------------

TITLE: Configurazione Vite per sviluppo mobile con Tauri
DESCRIPTION: Configurazione di Vite.js per lo sviluppo mobile che utilizza la variabile d'ambiente TAURI_DEV_HOST per permettere al server di essere accessibile dai dispositivi iOS durante lo sviluppo.

LANGUAGE: js
CODE:
import { defineConfig } from 'vite';

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig({
  clearScreen: false,
  server: {
    host: host || false,
    port: 1420,
    strictPort: true,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
  },
});

----------------------------------------

TITLE: Creating a Desktop Entry Point for Tauri 2.0
DESCRIPTION: Creates a new main.rs file that calls the shared run function from the library, maintaining compatibility with desktop platforms while supporting the new mobile architecture.

LANGUAGE: rust
CODE:
// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  app_lib::run();
}

----------------------------------------

TITLE: Displaying Community Guides and Tutorials
DESCRIPTION: This code uses the AwesomeTauri component to display community-contributed guides and tutorials, filtering for non-official, non-video content from the Awesome Tauri collection.

LANGUAGE: markdown
CODE:
<AwesomeTauri section="guides-no-official-no-video" />

----------------------------------------

TITLE: Configuring Custom Log Targets
DESCRIPTION: Example of using the clear_targets method to remove default targets and configure custom ones.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
.clear_targets()
.build()

----------------------------------------

TITLE: Updating Tauri Dependencies with Package Managers
DESCRIPTION: Commands to update Tauri CLI and API dependencies to the latest version using different package managers (npm, yarn, pnpm) and Cargo for Rust dependencies.

LANGUAGE: bash
CODE:
npm install @tauri-apps/cli@latest @tauri-apps/api@latest

LANGUAGE: bash
CODE:
yarn upgrade @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
pnpm update @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
cargo update

----------------------------------------

TITLE: CLI Plugin Configuration in package.json
DESCRIPTION: Defines the CLI plugin dependency in package.json for a JavaScript Tauri application.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-cli": "^2.0.0"
  }
}

----------------------------------------

TITLE: Creating a Desktop Entry Point for Tauri 2.0
DESCRIPTION: Creates a new main.rs file that calls the shared run function from the library, maintaining compatibility with desktop platforms while supporting the new mobile architecture.

LANGUAGE: rust
CODE:
// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  app_lib::run();
}

----------------------------------------

TITLE: Installing the File System Plugin via Command Line
DESCRIPTION: Command to install the file system plugin in a Tauri project using cargo.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-fs

----------------------------------------

TITLE: Installing the Shell Plugin with Shell Command
DESCRIPTION: Command to add the shell plugin to a Tauri project's dependencies in Cargo.toml by running it in the src-tauri folder.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-shell

----------------------------------------

TITLE: JSON Configuration Format Example in Tauri
DESCRIPTION: Example of Tauri configuration using the JSON format. This shows basic configuration properties for the build system including devPath and distDir settings.

LANGUAGE: json
CODE:
{
  "build": {
    "devPath": "http://localhost:8000",
    "distDir": "../dist"
  }
}

----------------------------------------

TITLE: Selecting UI Template for Rust Frontend
DESCRIPTION: Command prompt showing the UI template options available for Rust-based frontends in a Tauri project, including Vanilla, Yew, Leptos, and Sycamore.

LANGUAGE: bash
CODE:
? Choose your UI template ›
Vanilla
Yew
Leptos
Sycamore

----------------------------------------

TITLE: Configuring Vite for Tauri Mobile Apps (2.0.0-rc)
DESCRIPTION: Updated Vite configuration for Tauri mobile apps in version 2.0.0-rc. This setup uses the TAURI_DEV_HOST environment variable provided by Tauri CLI instead of manually determining the network IP.

LANGUAGE: javascript
CODE:
import { defineConfig } from 'vite';
import Unocss from 'unocss/vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    host: host || false,
    port: 1420,
    strictPort: true,
    hmr: host
      ? {
          protocol: 'ws',
          host: host,
          port: 1430,
        }
      : undefined,
  },
});

----------------------------------------

TITLE: Querying Specific RPM Package Information
DESCRIPTION: Command to query specific information from an RPM package using a custom format string.

LANGUAGE: bash
CODE:
rpm  -qp --queryformat '[%{NAME} %{VERSION} %{RELEASE} %{ARCH} %{SIZE}\n]' package_name.rpm

----------------------------------------

TITLE: Configuring iOS Privacy Settings for File System Access
DESCRIPTION: XML configuration for iOS privacy settings required for file system access, specifying the NSPrivacyAccessedAPICategoryFileTimestamp key.

LANGUAGE: xml
CODE:
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>NSPrivacyAccessedAPITypes</key>
    <array>
      <dict>
        <key>NSPrivacyAccessedAPIType</key>
        <string>NSPrivacyAccessedAPICategoryFileTimestamp</string>
        <key>NSPrivacyAccessedAPITypeReasons</key>
        <array>
          <string>C617.1</string>
        </array>
      </dict>
    </array>
  </dict>
</plist>

----------------------------------------

TITLE: Defining a LoginLayout struct for egui
DESCRIPTION: Create a struct to represent a login layout in egui with fields for username, password, and other UI elements. It includes a constructor that creates a channel for communicating with the main application.

LANGUAGE: rust
CODE:
use std::sync::mpsc::{channel, Receiver, Sender};
use tauri_egui::{eframe, egui};

pub struct LoginLayout {
  heading: String,
  users: Vec<String>,
  user: String,
  password: String,
  password_checker: Box<dyn Fn(&str) -> bool + Send + 'static>,
  tx: Sender<String>,
  texture: Option<egui::TextureHandle>,
}

impl LoginLayout {
  pub fn new(
    password_checker: Box<dyn Fn(&str) -> bool + Send + 'static>,
    users: Vec<String>,
  ) -> (Self, Receiver<String>) {
    let (tx, rx) = channel();
    let initial_user = users.iter().next().cloned().unwrap_or_else(String::new);
    (
      Self {
        heading: "Sign in".into(),
        users,
        user: initial_user,
        password: "".into(),
        password_checker,
        tx,
        texture: None,
      },
      rx,
    )
  }
}

----------------------------------------

TITLE: Initializing Tauri Application in Rust
DESCRIPTION: The main Rust entry point for the Tauri application, which creates and runs a default Tauri builder with the generated context. This minimal implementation is all that's needed to launch the Tauri window.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("unable to run Tauri application");
}

----------------------------------------

TITLE: Using MenuBuilder in Tauri 2.0
DESCRIPTION: Shows how to create a menu using the new MenuBuilder API in Tauri 2.0, which replaces the previous Menu API.

LANGUAGE: rust
CODE:
use tauri::menu::MenuBuilder;

tauri::Builder::default()
    .setup(|app| {
        let menu = MenuBuilder::new(app)
            .copy()
            .paste()
            .separator()
            .undo()
            .redo()
            .text("open-url", "Open URL")
            .check("toggle", "Toggle")
            .icon("show-app", "Show App", app.default_window_icon().cloned().unwrap())
            .build()?;
        Ok(())
    })

----------------------------------------

TITLE: Adding Android Targets to Rust Toolchain
DESCRIPTION: Command to add Android targets to the Rust toolchain using rustup, allowing compilation for different Android architectures when developing mobile apps with Tauri.

LANGUAGE: sh
CODE:
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

----------------------------------------

TITLE: Initializing the HTTP Plugin in Rust
DESCRIPTION: Code to initialize the HTTP plugin in your Tauri application by modifying the lib.rs file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Extracting WebView2 Fixed Version Runtime in PowerShell
DESCRIPTION: PowerShell command to extract a downloaded WebView2 fixed version runtime CAB file to the Tauri source folder. This is part of the process for bundling a fixed WebView2 version with your application.

LANGUAGE: powershell
CODE:
Expand .\Microsoft.WebView2.FixedVersionRuntime.128.0.2739.42.x64.cab -F:* ./src-tauri

----------------------------------------

TITLE: Setting Up Pre-Exit Hooks for Windows Updates in Tauri
DESCRIPTION: Shows how to register a callback function that will be executed before the application exits during a Windows update installation. This is useful for cleanup operations or saving application state.

LANGUAGE: rust
CODE:
use tauri_plugin_updater::UpdaterExt;

let update = app
  .updater_builder()
  .on_before_exit(|| {
    println!("app is about to exit on Windows!");
  })
  .build()?
  .check()
  .await?;

----------------------------------------

TITLE: RPM Post-Installation Script Template
DESCRIPTION: Sample shell script for the post-installation hook that shows how to access installation parameters.

LANGUAGE: bash
CODE:
echo "-------------"
echo "This is post"
echo "Install Value: $1"
echo "Upgrade Value: $1"
echo "Uninstall Value: $1"
echo "-------------"

----------------------------------------

TITLE: Installing Deep Link Plugin via Command Line
DESCRIPTION: Command to add the deep-link plugin to the project's dependencies in Cargo.toml.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-deep-link@2.0.0

----------------------------------------

TITLE: Creating a Tauri App with Deno
DESCRIPTION: Command to create a new Tauri application using Deno runtime. This runs the create-tauri-app package from npm using Deno with all permissions enabled.

LANGUAGE: sh
CODE:
deno run -A npm:create-tauri-app

----------------------------------------

TITLE: Running Shell Command to Add Persisted Scope Plugin
DESCRIPTION: Command to add the tauri-plugin-persisted-scope dependency to the project's Cargo.toml file. This is the first step in the manual installation process.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-persisted-scope

----------------------------------------

TITLE: Configuring Downloaded Bootstrapper in Tauri
DESCRIPTION: Configuration for using a downloaded WebView2 bootstrapper in a Tauri Windows application. This is the default setting that downloads the bootstrapper and runs it, requiring an internet connection but resulting in a smaller installer size.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "webviewInstallMode": {
        "type": "downloadBootstrapper"
      }
    }
  }
}

----------------------------------------

TITLE: Adding schemars Dependency in Cargo.toml
DESCRIPTION: This snippet shows how to add the schemars dependency to Cargo.toml for both dependencies and build-dependencies, which is needed to generate JSON schemas for scope entries.

LANGUAGE: toml
CODE:
# we need to add schemars to both dependencies and build-dependencies because the scope.rs module is shared between the app code and build script
[dependencies]
schemars = "0.8"

[build-dependencies]
schemars = "0.8"

----------------------------------------

TITLE: Initializing Process Plugin in Rust
DESCRIPTION: Code snippet showing how to modify the Rust lib.rs file to initialize the process plugin in a Tauri application.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring Tauri with npm for SvelteKit Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using npm with SvelteKit, specifying build commands and frontend distribution folder location.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}

----------------------------------------

TITLE: Debugging RPM Package Scripts
DESCRIPTION: Command to display the pre/post install/remove scripts included in an RPM package for debugging.

LANGUAGE: bash
CODE:
rpm -qp --scripts package_name.rpm

----------------------------------------

TITLE: Auto-Generating Command Permissions in build.rs
DESCRIPTION: Configuration in build.rs to automatically generate permissions for the 'write_custom_file' command. This creates 'allow' and 'deny' permissions in the autogenerated folder.

LANGUAGE: rust
CODE:
const COMMANDS: &[&str] = &["ping", "write_custom_file"];

----------------------------------------

TITLE: Creating a Tauri App with npm
DESCRIPTION: Command to create a new Tauri application using npm package manager. This uses the create-tauri-app package to initialize a new Tauri project.

LANGUAGE: sh
CODE:
npm create tauri-app@latest

----------------------------------------

TITLE: Configuring Custom Folder Target for Log Storage
DESCRIPTION: Configuration to store logs in a custom directory with an optional custom filename.

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .target(tauri_plugin_log::Target::new(
    tauri_plugin_log::TargetKind::Folder {
      path: std::path::PathBuf::from("/path/to/logs"),
      file_name: None,
    },
  ))
  .build()

----------------------------------------

TITLE: Updating Tauri Dependencies with Package Managers
DESCRIPTION: Commands to update Tauri CLI and API dependencies to the latest version using various package managers and Cargo.

LANGUAGE: bash
CODE:
npm install @tauri-apps/cli@latest @tauri-apps/api@latest

LANGUAGE: bash
CODE:
yarn upgrade @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
pnpm update @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
cargo update

----------------------------------------

TITLE: Setting macOS Certificate for Code Signing in Tauri
DESCRIPTION: Command to convert a .p12 certificate to base64 encoding for use with the APPLE_CERTIFICATE environment variable in Tauri's code signing process.

LANGUAGE: bash
CODE:
openssl base64 -in MyCertificate.p12 -out MyCertificate-base64.txt

----------------------------------------

TITLE: Configuring Tauri with yarn for SvelteKit Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using yarn with SvelteKit, specifying build commands and frontend distribution folder location.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}

----------------------------------------

TITLE: Initializing Opener Plugin in Rust
DESCRIPTION: Code to initialize the opener plugin in the Tauri application's lib.rs file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Installing Cocoapods via Homebrew
DESCRIPTION: This command installs Cocoapods, a dependency manager for Swift and Objective-C Cocoa projects, which is required for iOS development with Tauri.

LANGUAGE: sh
CODE:
brew install cocoapods

----------------------------------------

TITLE: Configuring Tauri with yarn for Vite Integration
DESCRIPTION: Configures the Tauri build settings in tauri.conf.json to work with Vite when using yarn as the package manager. Sets up development URLs and build commands.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Building macOS Application Bundle with Tauri CLI
DESCRIPTION: Command examples for building a Tauri application as a macOS application bundle using different package managers. The --bundles app flag specifies that only the app bundle should be generated.

LANGUAGE: bash
CODE:
npm run tauri build -- --bundles app
yarn tauri build --bundles app
pnpm tauri build --bundles app
deno task tauri build --bundles app
bun tauri build --bundles app
cargo tauri build --bundles app

----------------------------------------

TITLE: Error Message for Missing Permission
DESCRIPTION: Example error message that appears when the necessary file system permissions are not properly configured. This shows the specific error for attempting to write to a file without the required permissions.

LANGUAGE: shell
CODE:
[Error] Unhandled Promise Rejection: fs.write_text_file not allowed. Permissions associated with this command: fs:allow-app-write, fs:allow-app-write-recursive, fs:allow-appcache-write, fs:allow-appcache-write-recursive, fs:allow-appconf...
(anonymous function) (main.ts:5)

----------------------------------------

TITLE: Installing Rust on Windows with winget
DESCRIPTION: PowerShell command to install Rust using the Windows Package Manager (winget). Ensures the MSVC toolchain is used, which is required for Tauri development.

LANGUAGE: powershell
CODE:
winget install --id Rustlang.Rustup

----------------------------------------

TITLE: Setting JAVA_HOME Environment Variable for Android Development on macOS
DESCRIPTION: Command to set the JAVA_HOME environment variable on macOS for Android development with Tauri, pointing to the Java installation from Android Studio.

LANGUAGE: sh
CODE:
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"

----------------------------------------

TITLE: Using the Simplified Core Permissions in Tauri 2.0 RC
DESCRIPTION: A simplified approach to enabling all default core plugin permissions at once using the new 'core:default' permission set. This replaces the need to list individual core plugin permissions separately.

LANGUAGE: json
CODE:
...\n\"permissions\": [\n    \"core:default\"\n]\n...

----------------------------------------

TITLE: Adding License to Cargo.toml
DESCRIPTION: TOML configuration for specifying the license in the Cargo.toml file for a Tauri application.

LANGUAGE: toml
CODE:
[package]
name = "tauri-app"
version = "0.0.0"
description = "A Tauri App"
authors = ["you"]
edition = "2021"
license = "MIT" # add the license here
# ...  rest of the file

----------------------------------------

TITLE: Adding MySQL Engine Support with Cargo
DESCRIPTION: Command to add MySQL support to the tauri-plugin-sql dependency.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-sql --features mysql

----------------------------------------

TITLE: Configuring Tauri with pnpm for Vite Integration
DESCRIPTION: Configures the Tauri build settings in tauri.conf.json to work with Vite when using pnpm as the package manager. Sets up development URLs and build commands.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Configuring Entitlements in tauri.conf.json
DESCRIPTION: JSON configuration snippet that tells Tauri to use a custom Entitlements.plist file during the build process for applying specific entitlements to the application.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "entitlements": "./Entitlements.plist"
    }
  }
}

----------------------------------------

TITLE: Initializing the fs Plugin in Rust
DESCRIPTION: Rust code modification to initialize the file system plugin in a Tauri application. This code adds the plugin to the Tauri builder in the lib.rs file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

----------------------------------------

TITLE: Installing System Dependencies for Gentoo Linux
DESCRIPTION: Command to install required system packages for Tauri development on Gentoo Linux using emerge package manager.

LANGUAGE: sh
CODE:
sudo emerge --ask \
  net-libs/webkit-gtk:4.1 \
  dev-libs/libappindicator \
  net-misc/curl \
  net-misc/wget \
  sys-apps/file

----------------------------------------

TITLE: Installing Tauri Updater Plugin via Cargo
DESCRIPTION: Command to add the Tauri updater plugin as a dependency to your project's Cargo.toml file. The target specification ensures it works on supported desktop platforms.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-updater --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

----------------------------------------

TITLE: Building a Universal macOS App with Tauri CLI
DESCRIPTION: Command to build a Tauri application as a universal macOS app bundle that supports both Apple Silicon and Intel processors. This creates an .app file that can be packaged for the App Store.

LANGUAGE: bash
CODE:
tauri build --bundles app --target universal-apple-darwin

----------------------------------------

TITLE: Tauri Application Directory Structure
DESCRIPTION: Shows the directory structure for a Tauri application, highlighting where permission and capability files should be located. Permissions can only be defined in TOML while capabilities can be in JSON, JSON5, or TOML.

LANGUAGE: sh
CODE:
tauri-app
├── index.html
├── package.json
├── src
├── src-tauri
│   ├── Cargo.toml
│   ├── permissions
│      └── <identifier>.toml
|   ├── capabilities
│      └── <identifier>.json/.toml
│   ├── src
│   ├── tauri.conf.json

----------------------------------------

TITLE: Installing Tauri Updater Plugin via Cargo
DESCRIPTION: Command to add the Tauri updater plugin as a dependency to your project's Cargo.toml file. The target specification ensures it works on supported desktop platforms.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-updater --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

----------------------------------------

TITLE: Configuring Cargo.toml for Mobile Support in Tauri 2.0
DESCRIPTION: Configuration needed in Cargo.toml to generate shared libraries required for mobile support in Tauri 2.0.

LANGUAGE: toml
CODE:
// src-tauri/Cargo.toml
[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

----------------------------------------

TITLE: Importing Astro and React Components in Tauri Documentation
DESCRIPTION: This code imports various components used to build the Features & Recipes documentation page, including LinkCard from Starlight, custom Astro components for displaying features, community resources, search functionality, and compatibility tables.

LANGUAGE: jsx
CODE:
import { LinkCard } from '@astrojs/starlight/components';
import FeaturesList from '@components/list/Features.astro';
import CommunityList from '@components/list/Community.astro';
import Search from '@components/CardGridSearch.astro';
import AwesomeTauri from '@components/AwesomeTauri.astro';
import TableCompatibility from '@components/plugins/TableCompatibility.astro';

----------------------------------------

TITLE: Installing LLVM on macOS via Homebrew
DESCRIPTION: Command to install the LLVM toolchain on macOS using Homebrew, necessary for cross-compiling Windows applications.

LANGUAGE: sh
CODE:
brew install llvm

----------------------------------------

TITLE: Implementing CLI Plugin in Rust
DESCRIPTION: Sets up the CLI plugin in a Rust Tauri application by initializing it in the application builder.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
}

----------------------------------------

TITLE: Installing Flatpak Runtime for GNOME
DESCRIPTION: Command to install the GNOME Platform and SDK runtime version 46 required for building Flatpaks.

LANGUAGE: shell
CODE:
flatpak install flathub org.gnome.Platform//46 org.gnome.Sdk//46

----------------------------------------

TITLE: Manually Adding the fs Plugin via Cargo
DESCRIPTION: Command to manually add the file system plugin to a Tauri application using Cargo. This is necessary when adding plugins from crates.io rather than using the Tauri CLI.

LANGUAGE: shell
CODE:
cargo add tauri-plugin-fs

----------------------------------------

TITLE: Clipboard Plugin Configuration in package.json
DESCRIPTION: Defines the Clipboard Manager plugin dependency in package.json for a JavaScript Tauri application.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-clipboard-manager": "^2.0.0"
  }
}

----------------------------------------

TITLE: Implementing CLI Plugin in Rust
DESCRIPTION: Sets up the CLI plugin in a Rust Tauri application by initializing it in the application builder.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
}

----------------------------------------

TITLE: Example HTTP Response Headers
DESCRIPTION: The resulting HTTP response headers for a Tauri helloworld application using the example configuration. Shows how different header types are formatted in the actual HTTP response.

LANGUAGE: http
CODE:
access-control-allow-origin:  http://tauri.localhost
access-control-expose-headers: Tauri-Custom-Header
content-security-policy: default-src 'self'; connect-src ipc: http://ipc.localhost; script-src 'self' 'sha256-Wjjrs6qinmnr+tOry8x8PPwI77eGpUFR3EEGZktjJNs='
content-type: text/html
cross-origin-embedder-policy: require-corp
cross-origin-opener-policy: same-origin
tauri-custom-header: key1 'value1' 'value2'; key2 'value3'
timing-allow-origin: https://developer.mozilla.org, https://example.com

----------------------------------------

TITLE: RPM Pre-Installation Script Template
DESCRIPTION: Sample shell script for the pre-installation hook that shows how to access installation parameters.

LANGUAGE: bash
CODE:
echo "-------------"
echo "This is pre"
echo "Install Value: $1"
echo "Upgrade Value: $1"
echo "Uninstall Value: $1"
echo "-------------"

----------------------------------------

TITLE: Configuring v1 Compatible Update Artifacts in tauri.conf.json
DESCRIPTION: JSON configuration for enabling v1-compatible update artifacts creation in Tauri. Used when migrating from older Tauri versions.

LANGUAGE: json
CODE:
{
  "bundle": {
    "createUpdaterArtifacts": "v1Compatible"
  }
}

----------------------------------------

TITLE: Installing Flatpak Tools on Gentoo
DESCRIPTION: Command to install the required flatpak and flatpak-builder tools on Gentoo Linux.

LANGUAGE: shell
CODE:
sudo emerge --ask \
sys-apps/flatpak \
dev-util/flatpak-builder

----------------------------------------

TITLE: Creating Platform-Specific Capabilities in Tauri
DESCRIPTION: JSON configuration showing how to make capabilities platform-dependent by limiting filesystem access to only Linux and Windows platforms using the 'platforms' field.

LANGUAGE: json
CODE:
{
  "identifier": "fs-read-home",
  "description": "Allow access file access to home directory",
  "local": true,
  "windows": ["first"],
  "permissions": [
    "fs:allow-home-read",
  ],
  "platforms": ["linux", "windows"]
}

----------------------------------------

TITLE: Adding Process Plugin to Cargo Dependencies
DESCRIPTION: Shows how to add the Process plugin to your Cargo.toml dependencies for a Tauri project.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-process = "2"

----------------------------------------

TITLE: Adding Process Plugin to Cargo Dependencies
DESCRIPTION: Shows how to add the Process plugin to your Cargo.toml dependencies for a Tauri project.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-process = "2"

----------------------------------------

TITLE: Configuring System Tray Feature in Cargo.toml
DESCRIPTION: Adds the tray-icon feature to the Tauri dependency in Cargo.toml to enable system tray functionality.

LANGUAGE: toml
CODE:
tauri = { version = "2.0.0", features = [ "tray-icon" ] }

----------------------------------------

TITLE: Installing Flatpak Tools on Fedora
DESCRIPTION: Command to install the required flatpak and flatpak-builder tools on Fedora.

LANGUAGE: shell
CODE:
sudo dnf install flatpak flatpak-builder

----------------------------------------

TITLE: Defining Default Plugin Permissions in TOML
DESCRIPTION: TOML configuration that defines the default permissions for the plugin, including the newly added 'allow-write-custom-file' permission to enable the command by default.

LANGUAGE: toml
CODE:
"$schema" = "schemas/schema.json"
[default]
description = "Default permissions for the plugin"
permissions = ["allow-ping", "allow-write-custom-file"]

----------------------------------------

TITLE: Installing Tauri CLI with Package Managers
DESCRIPTION: Shows how to install the Tauri CLI using different package managers including npm, yarn, pnpm, deno, and cargo. Each command installs the latest version of the Tauri CLI as a development dependency.

LANGUAGE: bash
CODE:
npm install --save-dev @tauri-apps/cli@latest

LANGUAGE: bash
CODE:
yarn add -D @tauri-apps/cli@latest

LANGUAGE: bash
CODE:
pnpm add -D @tauri-apps/cli@latest

LANGUAGE: bash
CODE:
deno add -D npm:@tauri-apps/cli@latest

LANGUAGE: bash
CODE:
cargo install tauri-cli --version "^2.0.0" --locked

----------------------------------------

TITLE: Install Script for Tauri AUR Package
DESCRIPTION: An installation script for Tauri applications that handles post-installation, upgrade, and removal tasks like updating icon caches and desktop database entries. This script is referenced in the PKGBUILD file.

LANGUAGE: ini
CODE:
post_install() {
	gtk-update-icon-cache -q -t -f usr/share/icons/hicolor
	update-desktop-database -q
}

post_upgrade() {
	post_install
}

post_remove() {
	gtk-update-icon-cache -q -t -f usr/share/icons/hicolor
	update-desktop-database -q
}


----------------------------------------

TITLE: Creating Dialog Capabilities for Multiple Windows in Tauri
DESCRIPTION: JSON configuration for a capability file that grants dialog permissions to both 'first' and 'second' windows, allowing them to create Yes/No dialog prompts.

LANGUAGE: json
CODE:
{
  "identifier": "dialog",
  "description": "Allow to open a dialog",
  "local": true,
  "windows": ["first", "second"],
  "permissions": ["dialog:allow-ask"]
}

----------------------------------------

TITLE: Initializing OS Plugin in Rust
DESCRIPTION: Demonstrates how to initialize the OS plugin in a Rust-based Tauri application.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
}

----------------------------------------

TITLE: Using Path API with Tauri Manager
DESCRIPTION: This code shows how to use the migrated path API functionality through the Tauri Manager trait. It demonstrates accessing the home directory and resolving paths relative to standard directories.

LANGUAGE: rust
CODE:
use tauri::{path::BaseDirectory, Manager};

tauri::Builder::default()
    .setup(|app| {
        let home_dir_path = app.path().home_dir().expect("failed to get home dir");

        let path = app.path().resolve("path/to/something", BaseDirectory::Config)?;

        Ok(())
  })

----------------------------------------

TITLE: Example i18n JSON Resource File
DESCRIPTION: A sample JSON file for internationalization containing translations. This represents the type of file that would be bundled as a resource in a Tauri application.

LANGUAGE: json
CODE:
{
  "hello": "Guten Tag!",
  "bye": "Auf Wiedersehen!"
}

----------------------------------------

TITLE: Configuring package.json Scripts for Next.js and Tauri
DESCRIPTION: JSON configuration for package.json that defines scripts for development, building, and running the Next.js application with Tauri integration.

LANGUAGE: json
CODE:
"scripts": {
  "dev": "next dev",
  "build": "next build",
  "start": "next start",
  "lint": "next lint",
  "tauri": "tauri"
}

----------------------------------------

TITLE: Configuring package.json Scripts for Next.js and Tauri
DESCRIPTION: JSON configuration for package.json that defines scripts for development, building, and running the Next.js application with Tauri integration.

LANGUAGE: json
CODE:
"scripts": {
  "dev": "next dev",
  "build": "next build",
  "start": "next start",
  "lint": "next lint",
  "tauri": "tauri"
}

----------------------------------------

TITLE: Passing Arguments to Sidecar in JavaScript
DESCRIPTION: JavaScript code demonstrating how to pass arguments to a sidecar command using the Command class from the Tauri shell plugin.

LANGUAGE: javascript
CODE:
import { Command } from '@tauri-apps/plugin-shell';
// notice that the args array matches EXACTLY what is specified in `capabilities/default.json`.
const command = Command.sidecar('binaries/my-sidecar', [
  'arg1',
  '-a',
  '--arg2',
  'any-string-that-matches-the-validator',
]);
const output = await command.execute();

----------------------------------------

TITLE: Defining Directory Creation Permission
DESCRIPTION: Example of a permission configuration that enables the mkdir command. This defines a simple command-based permission with a single allowed command.

LANGUAGE: toml
CODE:
[[permission]]
identifier = "allow-mkdir"
description = "This enables the mkdir command."
commands.allow = [
    "mkdir"
]

----------------------------------------

TITLE: Initializing Notification Plugin in Rust
DESCRIPTION: Demonstrates how to initialize the Notification plugin in a Rust-based Tauri application.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
}

----------------------------------------

TITLE: Configuring HTTP Headers in Next.js for Tauri Applications
DESCRIPTION: Adding HTTP headers to the Next.js configuration file (next.config.js) for Tauri applications. Next.js uses a different approach compared to Vite-based frameworks.

LANGUAGE: javascript
CODE:
module.exports = {
  //...
  async headers() {
    return [
      {
        source: '/*',
        headers: [
          {
            key: 'Cross-Origin-Opener-Policy',
            value: 'same-origin',
          },
          {
            key: 'Cross-Origin-Embedder-Policy',
            value: 'require-corp',
          },
          {
            key: 'Timing-Allow-Origin',
            value: 'https://developer.mozilla.org, https://example.com',
          },
          {
            key: 'Access-Control-Expose-Headers',
            value: 'Tauri-Custom-Header',
          },
          {
            key: 'Tauri-Custom-Header',
            value: "key1 'value1' 'value2'; key2 'value3'",
          },
        ],
      },
    ]
  },
}

----------------------------------------

TITLE: Configuring snapcraft.yaml for Tauri Applications
DESCRIPTION: Provides a template snapcraft.yaml configuration for packaging Tauri applications. Includes setup for dependencies, build process, and proper file organization within the snap package.

LANGUAGE: yaml
CODE:
name: appname
base: core22
version: '0.1.0'
summary: Your summary # 79 char long summary
description: |
  Your description

grade: stable
confinement: strict

layout:
  /usr/lib/$SNAPCRAFT_ARCH_TRIPLET/webkit2gtk-4.1:
    bind: $SNAP/usr/lib/$SNAPCRAFT_ARCH_TRIPLET/webkit2gtk-4.1

apps:
  appname:
    command: usr/bin/appname
    desktop: usr/share/applications/appname.desktop
    extensions: [gnome]
    #plugs:
    #  - network
    # Add whatever plugs you need here, see https://snapcraft.io/docs/snapcraft-interfaces for more info.
    # The gnome extension already includes [ desktop, desktop-legacy, gsettings, opengl, wayland, x11, mount-observe, calendar-service ]

package-repositories:
  - type: apt
    components: [main]
    suites: [noble]
    key-id: 78E1918602959B9C59103100F1831DDAFC42E99D
    url: http://ppa.launchpad.net/snappy-dev/snapcraft-daily/ubuntu

parts:
  build-app:
    plugin: dump
    build-snaps:
      - node/20/stable
      - rustup/latest/stable
    build-packages:
      - libwebkit2gtk-4.1-dev
      - build-essential
      - curl
      - wget
      - file
      - libxdo-dev
      - libssl-dev
      - libayatana-appindicator3-dev
      - librsvg2-dev
      - dpkg
    stage-packages:
      - libwebkit2gtk-4.1-0
      - libayatana-appindicator3-1
    source: .
    override-build: |
      set -eu
      npm install
      npm run tauri build -- --bundles deb
      dpkg -x src-tauri/target/release/bundle/deb/*.deb $SNAPCRAFT_PART_INSTALL/
      sed -i -e "s|Icon=appname|Icon=/usr/share/icons/hicolor/32x32/apps/appname.png|g" $SNAPCRAFT_PART_INSTALL/usr/share/applications/appname.desktop

----------------------------------------

TITLE: Setting JAVA_HOME Environment Variable for Android Development on Windows
DESCRIPTION: PowerShell command to set the JAVA_HOME environment variable on Windows for Android development with Tauri, pointing to the Java installation from Android Studio.

LANGUAGE: powershell
CODE:
[System.Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Android\Android Studio\jbr", "User")

----------------------------------------

TITLE: Configuring Tauri for Next.js Integration
DESCRIPTION: JSON configuration for the Tauri application to work with Next.js. Sets up build commands, development URL, and frontend distribution path.

LANGUAGE: json
CODE:
// src-tauri/tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../out"
  }
}

----------------------------------------

TITLE: Basic Tauri Project Cargo.toml
DESCRIPTION: Example of a minimal Cargo.toml file for a Tauri project, showing essential package metadata and dependencies required for Tauri application development.

LANGUAGE: toml
CODE:
[package]
name = "app"
version = "0.1.0"
description = "A Tauri App"
authors = ["you"]
license = ""
repository = ""
default-run = "app"
edition = "2021"
rust-version = "1.57"

[build-dependencies]
tauri-build = { version = "2.0.0" }

[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
tauri = { version = "2.0.0", features = [ ] }

----------------------------------------

TITLE: Configuring HTTP Headers in Nuxt for Tauri Applications
DESCRIPTION: Adding HTTP headers to the Nuxt configuration file (nuxt.config.ts) for Tauri applications. This configuration is needed to match development and production header settings.

LANGUAGE: typescript
CODE:
export default defineNuxtConfig({
  //...
  vite: {
    //...
    server: {
      //...
      headers:{
        'Cross-Origin-Opener-Policy': 'same-origin',
        'Cross-Origin-Embedder-Policy': 'require-corp',
        'Timing-Allow-Origin': 'https://developer.mozilla.org, https://example.com',
        'Access-Control-Expose-Headers': 'Tauri-Custom-Header',
        'Tauri-Custom-Header': "key1 'value1' 'value2'; key2 'value3'"
      }
    },
  },
});

----------------------------------------

TITLE: Adding HTTP Plugin to JavaScript Dependencies
DESCRIPTION: Shows how to add the HTTP plugin to package.json dependencies for a JavaScript-based Tauri project.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-http": "^2.0.0"
  }
}

----------------------------------------

TITLE: Setting MSVC Toolchain as Default for Rust on Windows
DESCRIPTION: Command to set the MSVC toolchain as the default for Rust on Windows, which is required for full compatibility with Tauri and other tools.

LANGUAGE: powershell
CODE:
rustup default stable-msvc

----------------------------------------

TITLE: Creating RPM Script Files
DESCRIPTION: Commands to create the four main RPM script files for pre/post installation and removal operations.

LANGUAGE: bash
CODE:
touch src-tauri/scripts/postinstall.sh \
touch src-tauri/scripts/preinstall.sh \
touch src-tauri/scripts/preremove.sh \
touch src-tauri/scripts/postremove.sh

----------------------------------------

TITLE: Generating GPG Key for RPM Signing
DESCRIPTION: Command to generate a GPG key for signing RPM packages, which is used in the Tauri build process.

LANGUAGE: bash
CODE:
gpg --gen-key

----------------------------------------

TITLE: Configuring NPM Scripts for Tauri and Next.js
DESCRIPTION: JSON configuration for package.json scripts that enable development, building, and Tauri command execution.

LANGUAGE: json
CODE:
"scripts": {
  "dev": "next dev",
  "build": "next build",
  "start": "next start",
  "lint": "next lint",
  "tauri": "tauri"
}

----------------------------------------

TITLE: Configuring Brownfield Pattern in Tauri
DESCRIPTION: JSON configuration for explicitly setting the brownfield pattern in the tauri.conf.json file. This pattern is the default and doesn't require additional configuration options.

LANGUAGE: json
CODE:
{
  "tauri": {
    "pattern": {
      "use": "brownfield"
    }
  }
}

----------------------------------------

TITLE: Updating Tauri Dependencies with Package Managers
DESCRIPTION: Commands for updating Tauri NPM and Cargo dependencies to the latest alpha release (2.0.0-alpha.4). Includes installation instructions for npm, yarn, pnpm, and cargo.

LANGUAGE: shell
CODE:
rm -r src-tauri/gen
tauri android init
tauri ios init

----------------------------------------

TITLE: Using Global Shortcut Plugin in Rust
DESCRIPTION: Demonstrates how to use the Global Shortcut plugin in Rust with a custom handler and shortcut registration.

LANGUAGE: rust
CODE:
use tauri_plugin_global_shortcut::GlobalShortcutExt;

tauri::Builder::default()
    .plugin(
        tauri_plugin_global_shortcut::Builder::new().with_handler(|app, shortcut| {
            println!("Shortcut triggered: {:?}", shortcut);
        })
        .build(),
    )
    .setup(|app| {
        // register a global shortcut
        // on macOS, the Cmd key is used
        // on Windows and Linux, the Ctrl key is used
        app.global_shortcut().register("CmdOrCtrl+Y")?;
        Ok(())
    })

----------------------------------------

TITLE: Uploading iOS App to App Store Using altool
DESCRIPTION: Command to upload a Tauri-built iOS app to the App Store using xcrun altool. This requires an App Store Connect API key for authentication, specified by the APPLE_API_KEY_ID and APPLE_API_ISSUER environment variables.

LANGUAGE: shell
CODE:
xcrun altool --upload-app --type ios --file "src-tauri/gen/apple/build/arm64/$APPNAME.ipa" --apiKey $APPLE_API_KEY_ID --apiIssuer $APPLE_API_ISSUER

----------------------------------------

TITLE: Using Process Plugin in Rust
DESCRIPTION: Shows how to exit or restart the application using the Process plugin in a Rust-based Tauri application.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // exit the app with a status code
            app.handle().exit(1);
            // restart the app
            app.handle().restart();
            Ok(())
        })
}

----------------------------------------

TITLE: Installing Global Shortcut Plugin via Command Line
DESCRIPTION: Command to add the global-shortcut plugin to a Tauri project's dependencies in Cargo.toml. This specifies platform targets for macOS, Windows, and Linux.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-global-shortcut --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

----------------------------------------

TITLE: Configuring Package.json Scripts for Tauri Development
DESCRIPTION: Script configuration in package.json for running development server, building the application, and executing Tauri commands.

LANGUAGE: json
CODE:
"scripts": {
  "dev": "next dev",
  "build": "next build",
  "start": "next start",
  "lint": "next lint",
  "tauri": "tauri"
}

----------------------------------------

TITLE: Initializing a Verso Browser Instance in Rust
DESCRIPTION: A minimal example of creating a Verso browser instance. This demonstrates Verso's simplified API compared to Servo's more complex approach. The code creates a browser window pointing to example.com with a panel enabled and maximized window state.

LANGUAGE: rust
CODE:
use std::env::current_exe;
use std::thread::sleep;
use std::time::Duration;
use url::Url;
use verso::VersoBuilder;

fn main() {
    let versoview_path = current_exe().unwrap().parent().unwrap().join("versoview");
    let controller = VersoBuilder::new()
        .with_panel(true)
        .maximized(true)
        .build(versoview_path, Url::parse("https://example.com").unwrap());
    loop {
        sleep(Duration::MAX);
    }
}

----------------------------------------

TITLE: Adding a submit button to egui layout
DESCRIPTION: Create a submit button that is enabled only when the password field is not empty. The button is positioned and sized appropriately in the layout.

LANGUAGE: rust
CODE:
let mut button = ui.add_enabled(!password.is_empty(), egui::Button::new("Unlock"));
button.rect.min.x = 100.;
button.rect.max.x = 100.;

----------------------------------------

TITLE: Global Shortcut Plugin Configuration in package.json
DESCRIPTION: Defines the Global Shortcut plugin dependency in package.json for a JavaScript Tauri application.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-global-shortcut": "^2.0.0"
  }
}

----------------------------------------

TITLE: Installing System Dependencies for openSUSE Linux
DESCRIPTION: Command to install required system packages for Tauri development on openSUSE Linux using zypper package manager.

LANGUAGE: sh
CODE:
sudo zypper up
sudo zypper in webkit2gtk3-devel \
  libopenssl-devel \
  curl \
  wget \
  file \
  libappindicator3-1 \
  librsvg-devel
sudo zypper in -t pattern devel_basis

----------------------------------------

TITLE: Implementing Drop Lifecycle Hook in Tauri Plugin
DESCRIPTION: Shows how to implement the on_drop lifecycle hook to execute cleanup code when the plugin is being destroyed.

LANGUAGE: rust
CODE:
use tauri::plugin::Builder;

Builder::new("<plugin-name>")
  .on_drop(|app| {
    // plugin has been destroyed...
  })

----------------------------------------

TITLE: Setting Minimum macOS System Version
DESCRIPTION: JSON configuration that specifies macOS 12.0 as the minimum system version required to run the application. This enforces the requirement in the app bundle.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "minimumSystemVersion": "12.0"
    }
  }
}

----------------------------------------

TITLE: Using Tauri CLI to Automate Migration from Beta to RC
DESCRIPTION: Command examples showing how to install the latest Tauri CLI and run the migrate command using different package managers (npm, yarn, pnpm) or cargo to automate the upgrade process.

LANGUAGE: shell
CODE:
npm install @tauri-apps/cli@latest
npm run tauri migrate

----------------------------------------

TITLE: Loading and displaying an image texture in egui
DESCRIPTION: Load a PNG image from included bytes, convert it to an egui texture, and display it in the UI. This requires the 'png' dependency.

LANGUAGE: rust
CODE:
let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
  let mut reader = png::Decoder::new(std::io::Cursor::new(include_bytes!("icons/32x32.png")))
  .read_info()
  .unwrap();
  let mut buffer = Vec::new();
  while let Ok(Some(row)) = reader.next_row() {
    buffer.extend(row.data());
  }
  let icon_size = [reader.info().width as usize, reader.info().height as usize];
  // Load the texture only once.
  ctx.load_texture(
    "icon",
    egui::ColorImage::from_rgba_unmultiplied(icon_size, &buffer),
    egui::TextureFilter::Linear,
  )
});
logo_and_heading(
  ui,
  egui::Image::new(texture, texture.size_vec2()),
  heading.as_str(),
);

----------------------------------------

TITLE: Configuring Vite for Tauri Mobile Apps (2.0.0-beta)
DESCRIPTION: Example Vite configuration for Tauri mobile apps in version 2.0.0-beta. This setup uses internal-ip to expose the development server on the network for mobile development with proper HMR configuration.

LANGUAGE: javascript
CODE:
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { internalIpV4Sync } from 'internal-ip';

const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    host: mobile ? '0.0.0.0' : false,
    port: 1420,
    strictPort: true,
    hmr: mobile
      ? {
          protocol: 'ws',
          host: internalIpV4Sync(),
          port: 1421,
        }
      : undefined,
  },
});

----------------------------------------

TITLE: Implementing Global Shortcut Plugin in Rust
DESCRIPTION: Sets up the Global Shortcut plugin in a Rust Tauri application with a default builder.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::default().build())
}

----------------------------------------

TITLE: Tauri CLI Commands for Microsoft Store Build
DESCRIPTION: Commands to build a Tauri app specifically for Microsoft Store distribution, using a separate configuration file for Microsoft Store-specific settings.

LANGUAGE: bash
CODE:
npm run tauri build -- --no-bundle
npm run tauri bundle -- --config src-tauri/tauri.microsoftstore.conf.json

----------------------------------------

TITLE: Defining a Database Migration in Rust
DESCRIPTION: Rust code that defines a database migration using the Migration struct from tauri-plugin-sql.

LANGUAGE: rust
CODE:
use tauri_plugin_sql::{Migration, MigrationKind};

let migration = Migration {
    version: 1,
    description: "create_initial_tables",
    sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
    kind: MigrationKind::Up,
};

----------------------------------------

TITLE: Verbose RPM Package Upgrade
DESCRIPTION: Command to upgrade an installed RPM package with detailed verbose output for debugging.

LANGUAGE: bash
CODE:
rpm -Uvvh package_name.rpm

----------------------------------------

TITLE: Migrating Core Plugin Permissions (After Change)
DESCRIPTION: Updated format for core plugin permissions in Tauri's capability configuration file with the required 'core:' prefix added to each permission.

LANGUAGE: json
CODE:
...
"permissions": [
    "core:path:default",
    "core:event:default",
    "core:window:default",
    "core:app:default",
    "core:image:default",
    "core:resources:default",
    "core:menu:default",
    "core:tray:default",
]
...

----------------------------------------

TITLE: Adding a user selection ComboBox in egui
DESCRIPTION: Create a dropdown ComboBox for user selection with monospace font styling. The ComboBox shows all available users and updates the selected user when changed.

LANGUAGE: rust
CODE:
ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
  control_label(ui, "User");
  egui::ComboBox::from_id_source("user")
    .width(ui.available_width() - 8.)
    .selected_text(egui::RichText::new(user.clone()).family(egui::FontFamily::Monospace))
    .show_ui(ui, move |ui| {
      for user_name in users {
        ui.selectable_value(user, user_name.clone(), user_name.clone());
      }
    })
    .response;
});

----------------------------------------

TITLE: Upgrading Tauri Dependencies with Package Managers
DESCRIPTION: Commands to update Tauri dependencies to the latest 1.5.0 release using different package managers and Cargo. These commands update both the CLI and API packages to ensure compatibility with the new version.

LANGUAGE: bash
CODE:
npm install @tauri-apps/cli@latest @tauri-apps/api@latest

LANGUAGE: bash
CODE:
yarn upgrade @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
pnpm update @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
cargo update

----------------------------------------

TITLE: Adding Global Shortcut Plugin Dependencies in Cargo.toml
DESCRIPTION: Adds the Global Shortcut plugin as a conditional dependency in Cargo.toml for desktop platforms.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
[target."cfg(not(any(target_os = \"android\", target_os = \"ios\")))".dependencies]
tauri-plugin-global-shortcut = "2"

----------------------------------------

TITLE: Updating Svelte Frontend to Test the Plugin Command
DESCRIPTION: Modified Svelte component that adds a button to invoke the new 'writeCustomFile' command and display the response. This allows testing the plugin functionality from the example application.

LANGUAGE: svelte
CODE:
<script>
  import Greet from './lib/Greet.svelte'
  import { ping, writeCustomFile } from 'tauri-plugin-test-api'

  let response = ''

  function updateResponse(returnValue) {
    response += `[${new Date().toLocaleTimeString()}]` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
  }

  function _writeCustomFile() {
    writeCustomFile("HELLO FROM TAURI PLUGIN").then(updateResponse).catch(updateResponse)
  }
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
  </div>

  <div>
    <button on:click="{_writeCustomFile}">Write</button>
    <div>{@html response}</div>
  </div>


</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>

----------------------------------------

TITLE: Creating Logs in Rust with the log Crate
DESCRIPTION: Example of creating log entries in Rust code using the log crate macros, which integrates with the Tauri log plugin.

LANGUAGE: rust
CODE:
log::error!("something bad happened!");
log::info!("Tauri is awesome!");

----------------------------------------

TITLE: Installing cargo-xwin for Cross-Compilation
DESCRIPTION: Command to install cargo-xwin, a tool that facilitates cross-compilation of Windows applications from non-Windows systems.

LANGUAGE: sh
CODE:
cargo install --locked cargo-xwin

----------------------------------------

TITLE: Configuring Tauri with deno for Nuxt Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using deno as package manager. Defines build commands, development URL, and frontend distribution path for Nuxt integration.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "deno task dev",
    "beforeBuildCommand": "deno task generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Creating a password input field in egui
DESCRIPTION: Add a password text field with masking (displaying characters as dots). The field spans the available width and includes a label.

LANGUAGE: rust
CODE:
ui.style_mut().spacing.item_spacing.y = 20.;

let textfield = ui
  .with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
    ui.style_mut().spacing.item_spacing.y = 0.;
    control_label(ui, "Password");
    ui.horizontal_wrapped(|ui| {
      let field = ui.add_sized(
        [ui.available_width(), 18.],
        egui::TextEdit::singleline(password).password(true),
      );
      field
    })
    .inner
  })
  .inner;

----------------------------------------

TITLE: Template Flavor Selection Prompt in create-tauri-app v3
DESCRIPTION: Second part of the new two-step template selection process in create-tauri-app version 3, showing specific options for the selected framework (Vue).

LANGUAGE: bash
CODE:
✔ Choose your package manager · pnpm
✔ Choose your UI template · Vue - (https://vuejs.org)
? Choose your UI flavor ›
❯ TypeScript
  JavaScript

----------------------------------------

TITLE: File System Plugin Configuration in package.json
DESCRIPTION: Defines the File System plugin dependency in package.json for a JavaScript Tauri application.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-fs": "^2.0.0"
  }
}

----------------------------------------

TITLE: Configuring Biometric Authentication for iOS
DESCRIPTION: XML configuration for the iOS Info.plist file to enable Face ID usage by adding the required NSFaceIDUsageDescription property.

LANGUAGE: xml
CODE:
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
	<dict>
		<key>NSFaceIDUsageDescription</key>
		<string>Authenticate with biometric</string>
	</dict>
</plist>

----------------------------------------

TITLE: Setting Android Environment Variables on macOS
DESCRIPTION: Commands to set ANDROID_HOME and NDK_HOME environment variables on macOS, which are required for Android development with Tauri.

LANGUAGE: sh
CODE:
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

----------------------------------------

TITLE: Selecting Package Manager for TypeScript/JavaScript Frontend
DESCRIPTION: Command prompt showing the package manager options available for TypeScript/JavaScript frontends in a Tauri project, including pnpm, yarn, npm, and bun.

LANGUAGE: bash
CODE:
? Choose your package manager ›
pnpm
yarn
npm
bun

----------------------------------------

TITLE: Configuring Tauri with NPM for Nuxt Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using NPM as the package manager. Sets up build commands, development URL, and frontend distribution path for a Nuxt project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Enabling tauri-egui plugin in Tauri application
DESCRIPTION: Initialize the tauri-egui plugin in a Tauri application's setup function.

LANGUAGE: rust
CODE:
fn main() {
  tauri::Builder::default()
    .setup(|app| {
      app.wry_plugin(tauri_egui::EguiPluginBuilder::new(app.handle()));
      Ok(())
    })
}

----------------------------------------

TITLE: Original Template Selection Prompt in create-tauri-app v2
DESCRIPTION: Shows the original single prompt for template selection in create-tauri-app version 2, demonstrating the large number of options that made selection difficult.

LANGUAGE: bash
CODE:
✔ Choose your package manager · pnpm
? Choose your UI template ›
  vanilla
  vanilla-ts
  vue
❯ vue-ts
  svelte
  svelte-ts
  react
  react-ts
  solid
  solid-ts
  next
  next-ts
  preact
  preact-ts
  angular
  clojurescript
  svelte-kit
  svelte-kit-ts

----------------------------------------

TITLE: Implementing File System Plugin in Rust
DESCRIPTION: Sets up the File System plugin in a Rust Tauri application.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
}

----------------------------------------

TITLE: Displaying Community Video Guides
DESCRIPTION: This code uses the AwesomeTauri component to display community-contributed video guides and tutorials, filtering for non-official video content from the Awesome Tauri collection.

LANGUAGE: markdown
CODE:
<AwesomeTauri section="guides-no-official-only-video" />

----------------------------------------

TITLE: Generating a GPG Key for AppImage Signing in Linux
DESCRIPTION: Command to generate a new GPG key that will be used for signing AppImage packages. This creates a full key pair that can be used for digital signatures.

LANGUAGE: shell
CODE:
gpg2 --full-gen-key

----------------------------------------

TITLE: Using CLI Plugin in Rust
DESCRIPTION: Shows how to use the CLI plugin in Rust to access command line arguments through the application setup.

LANGUAGE: rust
CODE:
fn main() {
    use tauri_plugin_cli::CliExt;
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            let cli_matches = app.cli().matches()?;
            Ok(())
        })
}

----------------------------------------

TITLE: Configuring Core Plugin Permissions in Tauri 2.0 RC (New Format)
DESCRIPTION: The new required format for specifying core plugin permissions in Tauri 2.0 release candidate. All core plugin permissions now use the 'core:' namespace prefix to clearly distinguish them from third-party plugins.

LANGUAGE: json
CODE:
...\n\"permissions\": [\n    \"core:path:default\",\n    \"core:event:default\",\n    \"core:window:default\",\n    \"core:app:default\",\n    \"core:image:default\",\n    \"core:resources:default\",\n    \"core:menu:default\",\n    \"core:tray:default\"\n]\n...

----------------------------------------

TITLE: Adding Shell Plugin to Cargo Dependencies
DESCRIPTION: Shows how to add the Shell plugin to your Cargo.toml dependencies for a Tauri project.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-shell = "2"

----------------------------------------

TITLE: Installing Barcode Scanner Plugin Manually via Cargo
DESCRIPTION: Command to add the barcode scanner plugin to the project's dependencies in Cargo.toml, targeting only Android and iOS platforms.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-barcode-scanner --target 'cfg(any(target_os = "android", target_os = "ios"))'

----------------------------------------

TITLE: Defining UI helper functions for egui layout
DESCRIPTION: Create helper functions for reusable UI elements like logo with heading and control labels, managing spacing between UI elements.

LANGUAGE: rust
CODE:
fn logo_and_heading(ui: &mut egui::Ui, logo: egui::Image, heading: &str) {
  let original_item_spacing_y = ui.style().spacing.item_spacing.y;
  ui.style_mut().spacing.item_spacing.y = 8.;
  ui.add(logo);
  ui.style_mut().spacing.item_spacing.y = 16.;
  ui.heading(egui::RichText::new(heading));
  ui.style_mut().spacing.item_spacing.y = original_item_spacing_y;
}

fn control_label(ui: &mut egui::Ui, label: &str) {
  let original_item_spacing_y = ui.style().spacing.item_spacing.y;
  ui.style_mut().spacing.item_spacing.y = 8.;
  ui.label(label);
  ui.style_mut().spacing.item_spacing.y = original_item_spacing_y;
}

----------------------------------------

TITLE: Installing OS Information Plugin with Package Manager
DESCRIPTION: Command to install the OS Information plugin using your project's package manager. This is the automatic setup method.

LANGUAGE: sh
CODE:
npm run tauri add os

LANGUAGE: sh
CODE:
yarn run tauri add os

LANGUAGE: sh
CODE:
pnpm tauri add os

LANGUAGE: sh
CODE:
deno task tauri add os

LANGUAGE: sh
CODE:
bun tauri add os

LANGUAGE: sh
CODE:
cargo tauri add os

----------------------------------------

TITLE: Updating Tauri Dependencies for 1.3.0
DESCRIPTION: Commands for updating Tauri CLI and API packages to version 1.3.0 using different package managers and Cargo.

LANGUAGE: bash
CODE:
npm install @tauri-apps/cli@latest @tauri-apps/api@latest

LANGUAGE: bash
CODE:
yarn upgrade @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
pnpm update @tauri-apps/cli @tauri-apps/api --latest

LANGUAGE: bash
CODE:
cargo update

----------------------------------------

TITLE: Adding OS Plugin to JavaScript Dependencies
DESCRIPTION: Shows how to add the OS plugin to package.json dependencies for a JavaScript-based Tauri project.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-os": "^2.0.0"
  }
}

----------------------------------------

TITLE: Tauri Configuration for NPM with Vite
DESCRIPTION: Configuration in tauri.conf.json for a Tauri project using npm with Vite, specifying build commands, development URL, and frontend distribution folder.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Rendering LinkCard Components for RSS Feed Options in JSX
DESCRIPTION: This code snippet displays three LinkCard components that provide links to different RSS feeds for the Tauri project. Each card includes a title, description, and href attribute pointing to the specific feed URL.

LANGUAGE: jsx
CODE:
<LinkCard
  title="All updates"
  description="Get notified about any updates across the entire site."
  href="/feed.xml"
/>

<LinkCard
  title="Blog updates"
  description="Stay up-to-date with the latest blog posts and articles."
  href="/blog/rss.xml"
/>

<LinkCard
  title="Pages updates"
  description="Receive updates for the main website pages."
  href="/pages.xml"
/>

----------------------------------------

TITLE: Configuring Tauri with npm for Nuxt Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using npm as package manager. Defines build commands, development URL, and frontend distribution path for Nuxt integration.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Creating a Tauri Application with create-tauri-app
DESCRIPTION: Command to create a new Tauri application using the create-tauri-app utility. This initializes a project with TypeScript and the Vanilla template using pnpm as the package manager.

LANGUAGE: shell
CODE:
pnpm create tauri-app

----------------------------------------

TITLE: Setting Minimum System Version for Apple Silicon Support
DESCRIPTION: JSON configuration that sets the minimum macOS version requirement to 12.0, which is needed when supporting only Apple Silicon processors instead of creating a universal binary.

LANGUAGE: json
CODE:
{
  "bundle": {
    "macOS": {
      "minimumSystemVersion": "12.0"
    }
  }
}

----------------------------------------

TITLE: Installing Previous Versions of create-tauri-app
DESCRIPTION: Commands to install and use version 2 of create-tauri-app using various package managers and installation methods for users who need access to the removed templates.

LANGUAGE: bash
CODE:
# pnpm
pnpm create tauri-app@2

# yarn
yarn create tauri-app@2

# npm
npm create tauri-app@2

# Cargo
cargo install create-tauri-app --version 2.8.0 --locked
cargo create-tauri-app

# Bash
sh <(curl https://create.tauri.app/v/2.8.0/sh)

# Powershell
iwr -useb https://create.tauri.app/v/2.8.0/ps | iex

----------------------------------------

TITLE: Using Notification Plugin in JavaScript
DESCRIPTION: Demonstrates how to send notifications using the Notification plugin in a JavaScript-based Tauri application.

LANGUAGE: javascript
CODE:
import { sendNotification } from '@tauri-apps/plugin-notification';
sendNotification('Tauri is awesome!');

----------------------------------------

TITLE: Configuring Tauri with npm for SvelteKit Integration
DESCRIPTION: JSON configuration for Tauri when using npm as the package manager. Sets up build commands and specifies the frontend distribution directory for a SvelteKit project.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}

----------------------------------------

TITLE: Configuring Single Language WiX Installer in Tauri
DESCRIPTION: Configuration for creating a WiX installer targeting a specific language (French in this example). This sets the language Tauri should build the installer against.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "wix": {
        "language": "fr-FR"
      }
    }
  }
}

----------------------------------------

TITLE: Tauri Application Directory Structure
DESCRIPTION: This shell snippet shows a simplified example of a Tauri application directory structure, highlighting the location of capability files within the src-tauri/capabilities directory and the tauri.conf.json file.

LANGUAGE: sh
CODE:
tauri-app
├── index.html
├── package.json
├── src
├── src-tauri
│   ├── Cargo.toml
│   ├── capabilities
│      └── <identifier>.json/toml
│   ├── src
│   ├── tauri.conf.json

----------------------------------------

TITLE: Enabling Detailed Backtraces for Rust Errors in Tauri (Linux/macOS)
DESCRIPTION: Command for enabling detailed Rust backtraces when debugging Tauri applications on Linux and macOS platforms, providing more information about errors.

LANGUAGE: shell
CODE:
RUST_BACKTRACE=1 tauri dev

----------------------------------------

TITLE: Enabling JSON5 or TOML Configuration in Cargo.toml
DESCRIPTION: Code snippet showing how to enable JSON5 or TOML configuration formats by adding feature flags to the tauri and tauri-build dependencies in Cargo.toml.

LANGUAGE: toml
CODE:
[build-dependencies]
tauri-build = { version = "2.0.0", features = [ "config-json5" ] }

[dependencies]
tauri = { version = "2.0.0", features = [  "config-json5" ] }

----------------------------------------

TITLE: Improved UI Template Selection Prompt in create-tauri-app v3
DESCRIPTION: First part of the new two-step template selection process in create-tauri-app version 3, showing the simplified framework choices.

LANGUAGE: bash
CODE:
✔ Choose your package manager · pnpm
? Choose your UI template ›
  Vanilla
❯ Vue
  Svelte
  React
  Solid
  Angular
  Next
  SvelteKit
  ClojureScript
  Preact

----------------------------------------

TITLE: Using Global Shortcut Plugin in JavaScript
DESCRIPTION: Shows how to use the Global Shortcut plugin in JavaScript to register keyboard shortcuts.

LANGUAGE: javascript
CODE:
import { register } from '@tauri-apps/plugin-global-shortcut';
await register('CommandOrControl+Shift+C', () => {
  console.log('Shortcut triggered');
});

----------------------------------------

TITLE: Configuring Tauri with Qwik using npm
DESCRIPTION: Tauri configuration for a Qwik project using npm as the package manager. It specifies the development URL, frontend distribution directory, and commands to run before development and build processes.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "devUrl": "http://localhost:5173"
    "frontendDist": "../dist",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  }
}

----------------------------------------

TITLE: Enabling NSIS Language Selector in Tauri
DESCRIPTION: Configuration to enable a language selector in the NSIS installer. This allows users to choose their preferred language at the start of the installation process, before the installer content is rendered.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "nsis": {
        "displayLanguageSelector": true
      }
    }
  }
}

----------------------------------------

TITLE: Managing Flatpak Applications Locally
DESCRIPTION: Commands for installing, running, and updating a Flatpak application from a local repository for testing purposes.

LANGUAGE: shell
CODE:
# Install the flatpak
flatpak -y --user install <local repo name> <your flatpak id>

# Run it
flatpak run <your flatpak id>

# Update it
flatpak -y --user update <your flatpak id>

----------------------------------------

TITLE: Linux-specific Tauri Configuration
DESCRIPTION: Example of a platform-specific configuration for Linux that will be merged with the base configuration, overriding some values and adding new ones.

LANGUAGE: json
CODE:
{
  "productName": "my-app",
  "bundle": {
    "resources": ["./linux-assets"]
  },
  "plugins": {
    "cli": {
      "description": "My app",
      "subcommands": {
        "update": {}
      }
    },
    "deep-link": {}
  }
}

----------------------------------------

TITLE: Improved Frontend Language Selection Prompt in create-tauri-app
DESCRIPTION: Example of the improved language selection prompt in create-tauri-app version 3, showing how users can choose between Rust and TypeScript/JavaScript for their frontend.

LANGUAGE: bash
CODE:
? Choose which language to use for your frontend ›
  Rust
❯ TypeScript / JavaScript  (pnpm, yarn, npm)

----------------------------------------

TITLE: Using Dialog Plugin in Rust
DESCRIPTION: Demonstrates how to use the Dialog plugin in Rust to pick files and show message dialogs.

LANGUAGE: rust
CODE:
use tauri_plugin_dialog::DialogExt;
tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| {
        app.dialog().file().pick_file(|file_path| {
            // do something with the optional file path here
            // the file path is `None` if the user closed the dialog
        });

        app.dialog().message("Tauri is Awesome!").show();
        Ok(())
     })

----------------------------------------

TITLE: Selecting UI Template for .NET Frontend
DESCRIPTION: Command prompt showing Blazor as the available UI template option for .NET-based frontends in a Tauri project.

LANGUAGE: bash
CODE:
? Choose your UI template ›
Blazor  (https://dotnet.microsoft.com/en-us/apps/aspnet/web-apps/blazor/)

----------------------------------------

TITLE: Configuring Tauri with Deno for SvelteKit Integration
DESCRIPTION: JSON configuration for tauri.conf.json when using Deno with SvelteKit, specifying build commands and frontend distribution folder location.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "deno task dev",
    "beforeBuildCommand": "deno task build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}

----------------------------------------

TITLE: Adding HTTP Plugin to Cargo Dependencies
DESCRIPTION: Shows how to add the HTTP plugin to your Cargo.toml dependencies for a Tauri project.

LANGUAGE: toml
CODE:
# Cargo.toml
[dependencies]
tauri-plugin-http = "2"

----------------------------------------

TITLE: Installing Dialog Plugin via Package Manager
DESCRIPTION: Shows how to install the Tauri Dialog plugin using various package managers through the Tauri CLI.

LANGUAGE: sh
CODE:
cargo tauri add dialog

----------------------------------------

TITLE: Configuring Development Environment in Tauri
DESCRIPTION: JSON configuration for setting up development URL and command. This snippet shows how to configure Tauri to work with a JavaScript framework's development server by specifying the development URL and the command to start the dev server.

LANGUAGE: json
CODE:
{
  "build": {
    "devUrl": "http://localhost:3000",
    "beforeDevCommand": "npm run dev"
  }
}

----------------------------------------

TITLE: Creating a Tauri App with Cargo
DESCRIPTION: Commands to create a new Tauri application using Rust's Cargo package manager. This first installs the create-tauri-app package with the --locked flag, then runs it to initialize a new Tauri project.

LANGUAGE: sh
CODE:
cargo install create-tauri-app --locked
cargo create-tauri-app

----------------------------------------

TITLE: Configuring SvelteKit with Static Adapter
DESCRIPTION: JavaScript configuration for SvelteKit that imports and implements the static adapter. This enables static site generation which is required for Tauri integration.

LANGUAGE: js
CODE:
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter(),
  },
};

export default config;

----------------------------------------

TITLE: Selecting Frontend Language for Tauri Project
DESCRIPTION: Command prompt showing the available language options for the frontend of a Tauri application, including Rust, TypeScript/JavaScript, and .NET.

LANGUAGE: bash
CODE:
? Choose which language to use for your frontend ›
Rust  (cargo)
TypeScript / JavaScript  (pnpm, yarn, npm, bun)
.NET  (dotnet)

----------------------------------------

TITLE: Configuring Multi-Language WiX Installers in Tauri
DESCRIPTION: Configuration for creating WiX installers targeting multiple languages. This approach generates a specific installer for each language, with the language key as a suffix.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "wix": {
        "language": ["en-US", "pt-BR", "fr-FR"]
      }
    }
  }
}

----------------------------------------

TITLE: Configurando tauri.conf.json con deno para Nuxt
DESCRIPTION: Configuración del archivo tauri.conf.json utilizando deno como entorno de ejecución. Define los comandos para desarrollo y construcción, la URL de desarrollo y la ubicación de la distribución frontend.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "deno task dev",
    "beforeBuildCommand": "deno task generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Setting Environment Variables for Update Signing on Mac/Linux
DESCRIPTION: Shell commands to set environment variables for the Tauri signing process on Mac/Linux. These variables define the private key path or content and optional password.

LANGUAGE: sh
CODE:
export TAURI_SIGNING_PRIVATE_KEY="Path or content of your private key"
# optionally also add a password
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""

----------------------------------------

TITLE: Setting Environment Variables for Update Signing on Mac/Linux
DESCRIPTION: Shell commands to set environment variables for the Tauri signing process on Mac/Linux. These variables define the private key path or content and optional password.

LANGUAGE: sh
CODE:
export TAURI_SIGNING_PRIVATE_KEY="Path or content of your private key"
# optionally also add a password
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""

----------------------------------------

TITLE: Configuring Vite for Mobile Development with Tauri
DESCRIPTION: JavaScript configuration for Vite to work with Tauri mobile development. This setup ensures the development server listens on the correct host address to be accessible by iOS devices during development.

LANGUAGE: js
CODE:
import { defineConfig } from 'vite';

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig({
  clearScreen: false,
  server: {
    host: host || false,
    port: 1420,
    strictPort: true,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
  },
});

----------------------------------------

TITLE: Creating a Tauri App with Yarn
DESCRIPTION: Command to create a new Tauri application using Yarn package manager. This uses the create-tauri-app package to initialize a new Tauri project.

LANGUAGE: sh
CODE:
yarn create tauri-app

----------------------------------------

TITLE: Configuring Tauri with Qwik using yarn
DESCRIPTION: Tauri configuration for a Qwik project using yarn as the package manager. It defines the development URL, frontend distribution directory, and commands to run before development and build processes.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "devUrl": "http://localhost:5173"
    "frontendDist": "../dist",
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn build"
  }
}

----------------------------------------

TITLE: Combining Allow and Deny Scopes in a Permission Set
DESCRIPTION: Example of creating a permission set that provides reasonable access to application data while maintaining security by combining both allow and deny scopes.

LANGUAGE: toml
CODE:
[[set]]
identifier = "scope-applocaldata-reasonable"
description = '''
This scope set allows access to the `APPLOCALDATA` folder and
subfolders except for linux,
while it denies access to dangerous Tauri relevant files and
folders by default on windows.
'''
permissions = ["scope-applocaldata-recursive", "deny-default"]

----------------------------------------

TITLE: Installing LLVM and LLD Linker on Ubuntu
DESCRIPTION: Commands to install the LLVM toolchain and LLD linker on Ubuntu, required for cross-compiling Windows applications.

LANGUAGE: sh
CODE:
sudo apt install lld llvm

----------------------------------------

TITLE: Setting Application Icon as Tray Icon in JavaScript
DESCRIPTION: Demonstrates how to use the application's default window icon as the tray icon in JavaScript. This uses the defaultWindowIcon function from the app API.

LANGUAGE: javascript
CODE:
import { TrayIcon } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';

const options = {
  icon: await defaultWindowIcon(),
};

const tray = await TrayIcon.new(options);

----------------------------------------

TITLE: Installing WebSocket Plugin with Package Manager (Shell)
DESCRIPTION: Command to add the WebSocket plugin to a Tauri project using the project's package manager.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-websocket

----------------------------------------

TITLE: Creating a .taurignore File for Tauri File Watching
DESCRIPTION: Example of a .taurignore file used to restrict which files Tauri watches for changes. This file follows the same syntax as .gitignore files, allowing you to ignore specific folders and files from triggering automatic rebuilds.

LANGUAGE: plaintext
CODE:
build/
src/generated/*.rs
deny.toml

----------------------------------------

TITLE: Dialog Plugin Configuration in package.json
DESCRIPTION: Defines the Dialog plugin dependency in package.json for a JavaScript Tauri application.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-dialog": "^2.0.0"
  }
}

----------------------------------------

TITLE: Configuring Notification Plugin in package.json
DESCRIPTION: Adds the @tauri-apps/plugin-notification dependency to package.json for JavaScript notifications.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-notification": "^2.0.0"
  }
}

----------------------------------------

TITLE: Configurazione frontendDist per sviluppo senza framework
DESCRIPTION: Configurazione nel file tauri.conf.json per indicare a Tauri dove si trova il codice frontend quando non si utilizza alcun framework UI o bundler. Tauri avvierà un server di sviluppo predefinito.

LANGUAGE: json
CODE:
{
  "build": {
    "frontendDist": "./src"
  }
}

----------------------------------------

TITLE: Configuring HTTP Headers in Angular for Tauri Applications
DESCRIPTION: Adding HTTP headers to the Angular configuration file (angular.json) for Tauri applications. Required for development environments to match production header settings.

LANGUAGE: json
CODE:
{
  //...
  "projects":{
    //...
    "insert-project-name":{
      //...
      "architect":{
        //...
        "serve":{
          //...
          "options":{
            //...
            "headers":{
              "Cross-Origin-Opener-Policy": "same-origin",
              "Cross-Origin-Embedder-Policy": "require-corp",
              "Timing-Allow-Origin": "https://developer.mozilla.org, https://example.com",
              "Access-Control-Expose-Headers": "Tauri-Custom-Header",
              "Tauri-Custom-Header": "key1 'value1' 'value2'; key2 'value3'"
            }
          }
        }
      }
    }
  }
}

----------------------------------------

TITLE: Downloading Files with JavaScript
DESCRIPTION: Example of using the Upload plugin's JavaScript API to download a file from a remote server, including progress tracking and custom headers.

LANGUAGE: javascript
CODE:
import { download } from '@tauri-apps/plugin-upload';
// when using `"withGlobalTauri": true`, you may use
// const { download } = window.__TAURI__.upload;

download(
  'https://example.com/file-download-link',
  './path/to/save/my/file.txt',
  ({ progress, total }) =>
    console.log(`Downloaded ${progress} of ${total} bytes`), // a callback that will be called with the download progress
  { 'Content-Type': 'text/plain' } // optional headers to send with the request
);

----------------------------------------

TITLE: Dialog Plugin Configuration in package.json
DESCRIPTION: Defines the Dialog plugin dependency in package.json for a JavaScript Tauri application.

LANGUAGE: json
CODE:
// package.json
{
  "dependencies": {
    "@tauri-apps/plugin-dialog": "^2.0.0"
  }
}

----------------------------------------

TITLE: JSON5 Configuration Format Example in Tauri
DESCRIPTION: Example of Tauri configuration using the JSON5 format, which is available with the config-json5 Cargo feature. JSON5 allows comments and more flexible syntax than standard JSON.

LANGUAGE: json5
CODE:
{
  build: {
    // devServer URL (comments are allowed!)
    devPath: 'http://localhost:8000',
    distDir: '../dist',
  },
}

----------------------------------------

TITLE: Passing Arguments to Sidecar in Rust
DESCRIPTION: Rust function showing how to pass arguments to a sidecar command using the tauri_plugin_shell::ShellExt trait.

LANGUAGE: rust
CODE:
use tauri_plugin_shell::ShellExt;
#[tauri::command]
async fn call_my_sidecar(app: tauri::AppHandle) {
  let sidecar_command = app
    .shell()
    .sidecar("my-sidecar")
    .unwrap()
    .args(["arg1", "-a", "--arg2", "any-string-that-matches-the-validator"]);
  let (mut _rx, mut _child) = sidecar_command.spawn().unwrap();
}

----------------------------------------

TITLE: Using Notification Plugin in Rust with Tauri
DESCRIPTION: Example of checking notification permissions and sending a notification in Rust.

LANGUAGE: rust
CODE:
use tauri_plugin_notification::NotificationExt;
use tauri::plugin::PermissionState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            if app.notification().permission_state()? == PermissionState::Unknown {
                app.notification().request_permission()?;
            }
            if app.notification().permission_state()? == PermissionState::Granted {
                app.notification()
                    .builder()
                    .body("Tauri is awesome!")
                    .show()?;
            }
            Ok(())
        })
}

----------------------------------------

TITLE: Configurazione di devUrl e beforeDevCommand in Tauri
DESCRIPTION: Esempio di configurazione nel file tauri.conf.json per specificare l'URL di sviluppo e il comando da eseguire prima dell'avvio dell'ambiente di sviluppo, utile quando si utilizza un framework UI o un bundler JavaScript.

LANGUAGE: json
CODE:
{
  "build": {
    "devUrl": "http://localhost:3000",
    "beforeDevCommand": "npm run dev"
  }
}

----------------------------------------

TITLE: Registering Command Permissions in build.rs
DESCRIPTION: This Rust snippet shows how to register commands in the build.rs file using AppManifest. By default, all registered commands can be used by all windows, but this allows for more specific control.

LANGUAGE: rust
CODE:
fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(&["your_command"])),
    )
    .unwrap();
}

----------------------------------------

TITLE: Adding the fs Plugin with Tauri CLI
DESCRIPTION: Command to add the file system plugin to a Tauri application using the Tauri CLI. This is the automated approach for adding official Tauri plugins.

LANGUAGE: shell
CODE:
pnpm tauri add fs

----------------------------------------

TITLE: Creating a Menu with MenuBuilder in Tauri 2.0
DESCRIPTION: Example of using the new tauri::menu::MenuBuilder API to create an application menu with various items.

LANGUAGE: rust
CODE:
use tauri::menu::MenuBuilder;

tauri::Builder::default()
    .setup(|app| {
        let menu = MenuBuilder::new(app)
            .copy()
            .paste()
            .separator()
            .undo()
            .redo()
            .text("open-url", "Open URL")
            .check("toggle", "Toggle")
            .icon("show-app", "Show App", app.default_window_icon().cloned().unwrap())
            .build()?;
        Ok(())
    })

----------------------------------------

TITLE: Adding tauri-egui dependency in Cargo.toml
DESCRIPTION: Add the tauri-egui package to the project dependencies in Cargo.toml.

LANGUAGE: toml
CODE:
[dependencies]
tauri-egui = "0.1"

----------------------------------------

TITLE: Platform-Specific Desktop Capability Configuration
DESCRIPTION: This JSON snippet defines a capability specifically for desktop platforms (Linux, macOS, Windows). It enables permissions on plugins that are only available on desktop, such as global shortcut registration.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "desktop-capability",
  "windows": ["main"],
  "platforms": ["linux", "macOS", "windows"],
  "permissions": ["global-shortcut:allow-register"]
}

----------------------------------------

TITLE: Initializing Global Shortcut Plugin in Rust for Tauri
DESCRIPTION: Initializes the global shortcut plugin in a Tauri Rust application using the default builder.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::default().build())
}

----------------------------------------

TITLE: Configuring Tauri for Trunk Integration
DESCRIPTION: JSON configuration for Tauri to work with Trunk bundler. Sets up build commands, development path, distribution directory, and enables the global Tauri object for WASM bindings.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "trunk serve",
    "beforeBuildCommand": "trunk build",
    "devPath": "http://localhost:8080",
    "distDir": "../dist"
  },
  "app": {
    "withGlobalTauri": true
  }
}

----------------------------------------

TITLE: Initializing WebSocket Plugin in Rust
DESCRIPTION: Modifies the Rust entry point to initialize the WebSocket plugin in a Tauri application.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Configuring Resources for Info.plist Localization
DESCRIPTION: JSON configuration snippet for tauri.conf.json that specifies how to include localized InfoPlist.strings files in the application bundle.

LANGUAGE: json
CODE:
{
  "bundle": {
    "resources": {
      "infoplist/**": "./"
    }
  }
}

----------------------------------------

TITLE: Configuring Core Plugin Permissions in Tauri 2.0 RC with Prefixes
DESCRIPTION: Updated JSON configuration for Tauri 2.0 RC that prepends 'core:' to all permission identifiers as required by the new capabilities system.

LANGUAGE: json
CODE:
...\n"permissions": [\n    "core:path:default",\n    "core:event:default",\n    "core:window:default",\n    "core:app:default",\n    "core:image:default",\n    "core:resources:default",\n    "core:menu:default",\n    "core:tray:default",\n]\n...

----------------------------------------

TITLE: Configurando tauri.conf.json con yarn para Nuxt
DESCRIPTION: Configuración del archivo tauri.conf.json utilizando yarn como gestor de paquetes. Define los comandos para desarrollo y construcción, la URL de desarrollo y la ubicación de la distribución frontend.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "yarn dev",
    "beforeBuildCommand": "yarn generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Configuring Core Plugin Permissions in Tauri 2.0 RC with Prefixes
DESCRIPTION: Updated JSON configuration for Tauri 2.0 RC that prepends 'core:' to all permission identifiers as required by the new capabilities system.

LANGUAGE: json
CODE:
...\n"permissions": [\n    "core:path:default",\n    "core:event:default",\n    "core:window:default",\n    "core:app:default",\n    "core:image:default",\n    "core:resources:default",\n    "core:menu:default",\n    "core:tray:default",\n]\n...

----------------------------------------

TITLE: Tauri Plugin Directory Structure
DESCRIPTION: Shows the directory structure for a Tauri plugin, highlighting where permission files should be located. The permissions directory contains identifier-specific permission files and a default permission file.

LANGUAGE: sh
CODE:
tauri-plugin
├── README.md
├── src
│  └── lib.rs
├── build.rs
├── Cargo.toml
├── permissions
│  └── <identifier>.json/toml
│  └── default.json/toml

----------------------------------------

TITLE: Initializing File System Plugin in Rust for Tauri
DESCRIPTION: Initializes the file system plugin in a Tauri Rust application by adding it to the Builder.

LANGUAGE: rust
CODE:
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
}

----------------------------------------

TITLE: Using Window Plugin in JavaScript
DESCRIPTION: This code demonstrates how to use the window plugin in JavaScript to manipulate window properties, specifically setting the window title in this example.

LANGUAGE: javascript
CODE:
import { appWindow } from '@tauri-apps/plugin-window';
await appWindow.setTitle('Tauri');

----------------------------------------

TITLE: Installing Store Plugin with Cargo in Terminal
DESCRIPTION: Command to add the Tauri Store plugin to the project's dependencies in Cargo.toml using the cargo command line tool.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-store

----------------------------------------

TITLE: Configuring Opener Plugin Permissions
DESCRIPTION: JSON configuration to set permissions for the opener plugin in a Tauri application's capabilities file.

LANGUAGE: json
CODE:
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "opener:allow-open-path",
      "allow": [
        {
          "path": "/path/to/file"
        }
      ]
    }
  ]
}

----------------------------------------

TITLE: Handling form submission in egui
DESCRIPTION: Handle the submit action when the button is clicked or Enter key is pressed after entering the password. Validate the password and either send it to the main thread or show an error message.

LANGUAGE: rust
CODE:
if (textfield.lost_focus() && ui.input().key_pressed(egui::Key::Enter)) || button.clicked()
{
  if password_checker(&password) {
    let _ = tx.send(password.clone());
    password.clear();
    frame.close();
  } else {
    *heading = "Invalid password".into();
    textfield.request_focus();
  }
}

----------------------------------------

TITLE: Creating a Minimal HTML Frontend for Tauri Application
DESCRIPTION: An HTML file that serves as the frontend for the Tauri application. It includes a simple styled page with a centered "Hello, Tauri!" heading, using a dark color scheme with centered content.

LANGUAGE: html
CODE:
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>Hello Tauri!</title>
    <style>
      body {
        /* Add a nice colorscheme */
        background-color: #222831;
        color: #ececec;

        /* Make the body the exact size of the window */
        margin: 0;
        height: 100vh;
        width: 100vw;

        /* Vertically and horizontally center children of the body tag */
        display: flex;
        justify-content: center;
        align-items: center;
      }
    </style>
  </head>
  <body>
    <h1>Hello, Tauri!</h1>
  </body>
</html>

----------------------------------------

TITLE: Adding Updater Plugin Dependency in Cargo.toml
DESCRIPTION: This snippet shows how to add the Tauri updater plugin as a dependency in your Cargo.toml file. This is required when migrating from the deprecated tauri::updater API to the new plugin-based approach.

LANGUAGE: toml
CODE:
[dependencies]
tauri-plugin-updater = "2"

----------------------------------------

TITLE: Configurando nuxt.config.ts para integración con Tauri
DESCRIPTION: Configuración de Nuxt para trabajar con Tauri. Habilita SSG, configura el servidor de desarrollo para ser accesible desde dispositivos físicos iOS, y configura Vite para una mejor compatibilidad con Tauri CLI.

LANGUAGE: ts
CODE:
export default defineNuxtConfig({
  // (opcional) Habilita las herramientas de desarrollo de Nuxt
  devtools: { enabled: true },
  // Habilita SSG
  ssr: false,
  // Permite que el servidor de desarrollo sea detectable por otros dispositivos al ejecutarse en dispositivos físicos con iOS
  devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },
  vite: {
    // Mejor soporte para la salida de Tauri CLI
    clearScreen: false,
    // Habilita las variables de entorno
    // Las variables de entorno adicionales se pueden encontrar en
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      // Tauri requiere un puerto consistente
      strictPort: true,
    },
  },
});

----------------------------------------

TITLE: Initializing Positioner Plugin in Rust
DESCRIPTION: Code to initialize the positioner plugin in the Tauri application's lib.rs file.

LANGUAGE: rust
CODE:
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
		tauri::Builder::default()
				.setup(|app| {
						#[cfg(desktop)]
						app.handle().plugin(tauri_plugin_positioner::init());
						Ok(())
				})
				.run(tauri::generate_context!())
				.expect("error while running tauri application");
}

----------------------------------------

TITLE: Converting Main Function to Mobile-Compatible Entry Point in Tauri 2.0
DESCRIPTION: Modifies the entry point function to be compatible with both desktop and mobile builds using the mobile_entry_point macro.

LANGUAGE: rust
CODE:
// src-tauri/src/lib.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 在这里编写你的代码
}

----------------------------------------

TITLE: Creating Window-Specific Filesystem Capabilities in Tauri
DESCRIPTION: JSON configuration for creating a capability file that grants filesystem read access to the home directory, but only for the 'first' window.

LANGUAGE: json
CODE:
{
  "identifier": "fs-read-home",
  "description": "Allow access file access to home directory",
  "local": true,
  "windows": ["first"],
  "permissions": [
    "fs:allow-home-read",
  ]
}

----------------------------------------

TITLE: Accessing Managed State in Tauri
DESCRIPTION: Shows how to access previously managed state using the state method from any type that implements the Manager trait, such as the App instance.

LANGUAGE: rust
CODE:
let data = app.state::<AppData>();

----------------------------------------

TITLE: Displaying AppImage Embedded Signature
DESCRIPTION: Command to display the signature embedded within a Tauri AppImage package. This requires replacing $APPNAME and $VERSION with the correct values based on your configuration.

LANGUAGE: shell
CODE:
./src-tauri/target/release/bundle/appimage/$APPNAME_$VERSION_amd64.AppImage --appimage-signature

----------------------------------------

TITLE: Configurando tauri.conf.json con pnpm para Nuxt
DESCRIPTION: Configuración del archivo tauri.conf.json utilizando pnpm como gestor de paquetes. Define los comandos para desarrollo y construcción, la URL de desarrollo y la ubicación de la distribución frontend.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Filtering NFC Tags in JavaScript
DESCRIPTION: JavaScript code to scan NFC tags with specific filters for mime type, URI format, and NFC technologies using the scan function from the NFC plugin.

LANGUAGE: javascript
CODE:
import { scan, TechKind } from '@tauri-apps/plugin-nfc';

const techLists = [
  // capture anything using NfcF
  [TechKind.NfcF],
  // capture all MIFARE Classics with NDEF payloads
  [TechKind.NfcA, TechKind.MifareClassic, TechKind.Ndef],
];

const tag = await scan({
  type: 'ndef', // or 'tag'
  mimeType: 'text/plain',
  uri: {
    scheme: 'https',
    host: 'my.domain.com',
    pathPrefix: '/app',
  },
  techLists,
});

----------------------------------------

TITLE: Configuring Tauri with npm for Vite Integration
DESCRIPTION: Configures the Tauri build settings in tauri.conf.json to work with Vite when using npm as the package manager. Sets up development URLs and build commands.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Displaying AppImage Embedded Signature
DESCRIPTION: Command to display the signature embedded within a Tauri AppImage package. This requires replacing $APPNAME and $VERSION with the correct values based on your configuration.

LANGUAGE: shell
CODE:
./src-tauri/target/release/bundle/appimage/$APPNAME_$VERSION_amd64.AppImage --appimage-signature

----------------------------------------

TITLE: Configurando tauri.conf.json con npm para Nuxt
DESCRIPTION: Configuración del archivo tauri.conf.json utilizando npm como gestor de paquetes. Define los comandos para desarrollo y construcción, la URL de desarrollo y la ubicación de la distribución frontend.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run generate",
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Configuring Stdout Log Target
DESCRIPTION: Configuration to direct logs to the standard output stream (terminal).

LANGUAGE: rust
CODE:
tauri_plugin_log::Builder::new()
  .target(tauri_plugin_log::Target::new(
    tauri_plugin_log::TargetKind::Stdout,
  ))
  .build()

----------------------------------------

TITLE: Configuring Plugin in Tauri Application JSON
DESCRIPTION: Example of how to configure a plugin in a Tauri application's tauri.conf.json file, showing how to set plugin-specific options like timeout.

LANGUAGE: json
CODE:
{
  "build": { ... },
  "tauri": { ... },
  "plugins": {
    "plugin-name": {
      "timeout": 30
    }
  }
}

----------------------------------------

TITLE: Setting Minimum Webview2 Version in Tauri
DESCRIPTION: Configuration to specify a minimum required Webview2 version for the application. The installer will verify the current Webview2 version and run the Webview2 bootstrapper if needed to ensure compatibility with features like custom URI schemes.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "nsis": {
        "minimumWebview2Version": "110.0.1531.0"
      }
    }
  }
}

----------------------------------------

TITLE: Setting Minimum Webview2 Version in Tauri
DESCRIPTION: Configuration to specify a minimum required Webview2 version for the application. The installer will verify the current Webview2 version and run the Webview2 bootstrapper if needed to ensure compatibility with features like custom URI schemes.

LANGUAGE: json
CODE:
{
  "bundle": {
    "windows": {
      "nsis": {
        "minimumWebview2Version": "110.0.1531.0"
      }
    }
  }
}

----------------------------------------

TITLE: Converting iOS Provisioning Profile to Base64 in macOS Terminal
DESCRIPTION: Command to convert an iOS provisioning profile (.mobileprovision file) to base64 format and copy it to the clipboard. This base64 string is used as the IOS_MOBILE_PROVISION environment variable for manual code signing.

LANGUAGE: bash
CODE:
base64 -i <path-to-profile.mobileprovision> | pbcopy

----------------------------------------

TITLE: Finding Target Triple with rustc on Linux/Unix
DESCRIPTION: Command to check the current platform's target triple using rustc, which is necessary for properly naming sidecar binaries for different architectures.

LANGUAGE: sh
CODE:
rustc -Vv

----------------------------------------

TITLE: Initializing Global Shortcut Plugin in Rust
DESCRIPTION: Code modification for the lib.rs file to initialize the global-shortcut plugin in a Tauri application. This adds the plugin to the Tauri builder setup function.

LANGUAGE: rust
CODE:
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_global_shortcut::Builder::new().build());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

----------------------------------------

TITLE: Installing Localhost Plugin Manually with Cargo
DESCRIPTION: Command to add the localhost plugin to a Tauri project's dependencies in Cargo.toml using cargo.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-localhost

----------------------------------------

TITLE: Implementing Shared Logic for Desktop and Mobile in Tauri Plugin
DESCRIPTION: Shows how to create shared functionality between desktop and mobile implementations in a Tauri plugin by defining common methods in the lib.rs file.

LANGUAGE: rust
CODE:
use tauri::Runtime;

impl<R: Runtime> <plugin-name><R> {
  pub fn do_something(&self) {
    // do something that is a shared implementation between desktop and mobile
  }
}

----------------------------------------

TITLE: Creating a Tauri App with Fish Shell
DESCRIPTION: Command to create a new Tauri application using Fish shell. This uses curl with process substitution to fetch and execute the Tauri app creation script.

LANGUAGE: sh
CODE:
sh (curl -sSL https://create.tauri.app/sh | psub)

----------------------------------------

TITLE: Checking if Files Exist with Tauri FS Plugin
DESCRIPTION: Checks if a file exists at the specified path. This function returns a boolean and is useful for verifying file presence before performing operations that require the file to exist.

LANGUAGE: javascript
CODE:
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';
const tokenExists = await exists('token', {
  baseDir: BaseDirectory.AppLocalData,
});

----------------------------------------

TITLE: Checking if Files Exist with Tauri FS Plugin
DESCRIPTION: Checks if a file exists at the specified path. This function returns a boolean and is useful for verifying file presence before performing operations that require the file to exist.

LANGUAGE: javascript
CODE:
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';
const tokenExists = await exists('token', {
  baseDir: BaseDirectory.AppLocalData,
});

----------------------------------------

TITLE: Tauri Configuration for Deno with Vite
DESCRIPTION: Configuration in tauri.conf.json for a Tauri project using Deno with Vite, specifying build commands, development URL, and frontend distribution folder.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "deno task dev",
    "beforeBuildCommand": "deno task build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Emitting Events from Android Tauri Plugin
DESCRIPTION: Shows how to emit events from an Android Tauri plugin. The example demonstrates triggering events during plugin lifecycle, handling intents, and in response to command execution.

LANGUAGE: kotlin
CODE:
@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    override fun load(webView: WebView) {
      trigger("load", JSObject())
    }

    override fun onNewIntent(intent: Intent) {
      // handle new intent event
      if (intent.action == Intent.ACTION_VIEW) {
        val data = intent.data.toString()
        val event = JSObject()
        event.put("data", data)
        trigger("newIntent", event)
      }
    }

    @Command
    fun openCamera(invoke: Invoke) {
      val payload = JSObject()
      payload.put("open", true)
      trigger("camera", payload)
    }
}

----------------------------------------

TITLE: Configuring iOS Universal Links JSON
DESCRIPTION: JSON configuration for iOS universal links to associate URLs with your application.

LANGUAGE: json
CODE:
{
  "applinks": {
    "details": [
      {
        "appIDs": ["$DEVELOPMENT_TEAM_ID.$APP_BUNDLE_ID"],
        "components": [
          {
            "/": "/open/*",
            "comment": "Matches any URL whose path starts with /open/"
          }
        ]
      }
    ]
  }
}

----------------------------------------

TITLE: Tauri Configuration for Deno with Vite
DESCRIPTION: Configuration in tauri.conf.json for a Tauri project using Deno with Vite, specifying build commands, development URL, and frontend distribution folder.

LANGUAGE: json
CODE:
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "deno task dev",
    "beforeBuildCommand": "deno task build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}

----------------------------------------

TITLE: Implementing a Command in iOS Plugin
DESCRIPTION: Shows how to implement a command in an iOS Tauri plugin that can be called from Rust code, using Swift to resolve with a return value.

LANGUAGE: swift
CODE:
class ExamplePlugin: Plugin {
	@objc public func openCamera(_ invoke: Invoke) throws {
    invoke.resolve(["path": "/path/to/photo.jpg"])
	}
}

----------------------------------------

TITLE: Creating a Tauri App with PowerShell
DESCRIPTION: Command to create a new Tauri application using PowerShell. This uses Invoke-RestMethod (irm) to fetch and execute the Tauri app creation script.

LANGUAGE: sh
CODE:
irm https://create.tauri.app/ps | iex

----------------------------------------

TITLE: HTML Structure for Custom Titlebar in Tauri
DESCRIPTION: HTML markup for a custom titlebar with minimize, maximize, and close buttons. The data-tauri-drag-region attribute enables window dragging.

LANGUAGE: html
CODE:
<div data-tauri-drag-region class="titlebar">
  <div class="titlebar-button" id="titlebar-minimize">
    <img
      src="https://api.iconify.design/mdi:window-minimize.svg"
      alt="minimize"
    />
  </div>
  <div class="titlebar-button" id="titlebar-maximize">
    <img
      src="https://api.iconify.design/mdi:window-maximize.svg"
      alt="maximize"
    />
  </div>
  <div class="titlebar-button" id="titlebar-close">
    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>
</div>

----------------------------------------

TITLE: Installing webkit2gtk-4.1 on different Linux distributions
DESCRIPTION: This bash snippet shows how to install the WebKit2GTK-4.1 package on different Linux distributions including Arch Linux/Manjaro, Debian/Ubuntu, and Fedora using their respective package managers.

LANGUAGE: bash
CODE:
# On Arch Linux / Manjaro:
sudo pacman -S webkit2gtk-4.1
# On Debian / Ubuntu:
sudo apt install libwebkit2gtk-4.1-dev
# On Fedora:
sudo dnf install webkit2gtk4.1-devel

----------------------------------------

TITLE: Example Tauri package.json Scripts Configuration
DESCRIPTION: Basic package.json script configuration for a Tauri project using Vite, including scripts for development, building, preview, and Tauri CLI commands.

LANGUAGE: json
CODE:
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  }
}

----------------------------------------

TITLE: Example Tauri package.json Scripts Configuration
DESCRIPTION: Basic package.json script configuration for a Tauri project using Vite, including scripts for development, building, preview, and Tauri CLI commands.

LANGUAGE: json
CODE:
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  }
}

----------------------------------------

TITLE: Defining and Parsing Command Arguments in Android Plugin
DESCRIPTION: Shows how to define and parse command arguments in an Android Tauri plugin using classes annotated with @InvokeArg, including required, optional, and default values.

LANGUAGE: kotlin
CODE:
import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin

@InvokeArg
internal class OpenAppArgs {
  lateinit var name: String
  var timeout: Int? = null
}

@InvokeArg
internal class OpenArgs {
  lateinit var requiredArg: String
  var allowEdit: Boolean = false
  var quality: Int = 100
  var app: OpenAppArgs? = null
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
  @Command
  fun openCamera(invoke: Invoke) {
    val args = invoke.parseArgs(OpenArgs::class.java)
  }
}

----------------------------------------

TITLE: Using CLI Plugin in JavaScript
DESCRIPTION: Demonstrates how to use the CLI plugin in JavaScript to retrieve command line arguments.

LANGUAGE: javascript
CODE:
import { getMatches } from '@tauri-apps/plugin-cli';
const matches = await getMatches();

----------------------------------------

TITLE: Configuring Window Decoration in Tauri Config
DESCRIPTION: JSON configuration in tauri.conf.json to disable window decorations, which is a prerequisite for implementing custom titlebars.

LANGUAGE: json
CODE:
"tauri": {
	"windows": [
		{
			"decorations": false
		}
	]
}

----------------------------------------

TITLE: Adding Tokio Dependency for Asynchronous Operations
DESCRIPTION: Command-line instructions for adding the Tokio crate to the Rust backend, which provides asynchronous runtime capabilities for simulating heavy setup tasks.

LANGUAGE: sh
CODE:
# Run this command where the `Cargo.toml` file is
cd src-tauri
# Add the Tokio crate
cargo add tokio
# Optionally go back to the top folder to keep developing
# `tauri dev` can figure out where to run automatically
cd ..

----------------------------------------

TITLE: Using CLI Plugin in JavaScript
DESCRIPTION: Demonstrates how to use the CLI plugin in JavaScript to retrieve command line arguments.

LANGUAGE: javascript
CODE:
import { getMatches } from '@tauri-apps/plugin-cli';
const matches = await getMatches();

----------------------------------------

TITLE: Installing Single Instance Plugin via Command Line
DESCRIPTION: Command to manually add the Tauri Single Instance plugin as a dependency to your project's Cargo.toml file with platform-specific targeting.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-single-instance --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

----------------------------------------

TITLE: Implementing Permission Handlers for iOS Tauri Plugin
DESCRIPTION: Demonstrates how to override the checkPermissions and requestPermissions functions in an iOS Tauri plugin. This implementation allows the plugin to check and request notification permissions.

LANGUAGE: swift
CODE:
class ExamplePlugin: Plugin {
  @objc open func checkPermissions(_ invoke: Invoke) {
    invoke.resolve(["postNotification": "prompt"])
  }

  @objc public override func requestPermissions(_ invoke: Invoke) {
    // request permissions here
    // then resolve the request
    invoke.resolve(["postNotification": "granted"])
  }
}

----------------------------------------

TITLE: Creating a Contribution Link Card
DESCRIPTION: This code creates a link card encouraging users to contribute their own learning resources to the Awesome Tauri repository, directing them to the pull requests page.

LANGUAGE: markdown
CODE:
<LinkCard
  title="Have something to share?"
  description="Open a pull request to show us your amazing resource."
  href="https://github.com/tauri-apps/awesome-tauri/pulls"
/>

----------------------------------------

TITLE: Creating Link Cards for Security Learning Resources
DESCRIPTION: This snippet shows how to create a grid of link cards for security-related learning resources in Tauri, providing navigation to tutorials about plugin permissions and capabilities.

LANGUAGE: markdown
CODE:
<CardGrid>
  <LinkCard
    title="Using Plugin Permissions"
    href="/learn/security/using-plugin-permissions/"
  />
  <LinkCard
    title="Capabilities for Different Windows and Platforms"
    href="/learn/security/capabilities-for-windows-and-platforms/"
  />
  <LinkCard
    title="Writing Plugin Permissions"
    href="/learn/security/writing-plugin-permissions/"
  />
</CardGrid>

----------------------------------------

TITLE: Running the Tauri Application in Development Mode
DESCRIPTION: Command to run the Tauri application in development mode using pnpm. This launches the application for testing the implemented file system permissions.

LANGUAGE: shell
CODE:
pnpm run tauri dev

----------------------------------------

TITLE: Installing Opener Plugin with Package Manager
DESCRIPTION: Command to add the opener plugin to a Tauri project using package managers like npm, yarn, pnpm, deno, bun, or cargo.

LANGUAGE: sh
CODE:
cargo add tauri-plugin-opener

----------------------------------------

TITLE: Creating a Tauri App with pnpm
DESCRIPTION: Command to create a new Tauri application using pnpm package manager. This uses the create-tauri-app package to initialize a new Tauri project.

LANGUAGE: sh
CODE:
pnpm create tauri-app

----------------------------------------

TITLE: Using Astro Components for Layout in Markdown
DESCRIPTION: This snippet demonstrates the import and usage of various Astro components for creating a structured layout in a Markdown document, including Card, CardGrid, LinkCard, and custom components for displaying Tauri resources.

LANGUAGE: markdown
CODE:
import { Card, CardGrid, LinkCard } from '@astrojs/starlight/components';
import AwesomeTauri from '@components/AwesomeTauri.astro';
import BookItem from '@components/BookItem.astro';
import RoseRustBook from '@assets/learn/community/HTML_CSS_JavaScript_and_Rust_for_Beginners_A_Guide_to_Application_Development_with_Tauri.png';

----------------------------------------

TITLE: Implementing Event Loop Lifecycle Hook in Tauri Plugin
DESCRIPTION: Demonstrates how to implement the on_event lifecycle hook to handle core events such as window events, menu events, and application exit requests.

LANGUAGE: rust
CODE:
use std::{collections::HashMap, fs::write, sync::Mutex};
use tauri::{plugin::Builder, Manager, RunEvent};

struct DummyStore(Mutex<HashMap<String, String>>);

Builder::new("<plugin-name>")
  .setup(|app, _api| {
    app.manage(DummyStore(Default::default()));
    Ok(())
  })
  .on_event(|app, event| {
    match event {
      RunEvent::ExitRequested { api, .. } => {
        // user requested a window to be closed and there's no windows left

        // we can prevent the app from exiting:
        api.prevent_exit();
      }
      RunEvent::Exit => {
        // app is going to exit, you can cleanup here

        let store = app.state::<DummyStore>();
        write(
          app.path().app_local_data_dir().unwrap().join("store.json"),
          serde_json::to_string(&*store.0.lock().unwrap()).unwrap(),
        )
        .unwrap();
      }
      _ => {}
    }
  })

----------------------------------------

TITLE: Windows Certificate Import Step for GitHub Actions
DESCRIPTION: A GitHub Actions workflow step that imports a Windows code signing certificate on the Windows runner. This step decodes a base64-encoded certificate and imports it into the certificate store.

LANGUAGE: yml
CODE:
- name: import windows certificate
  if: matrix.platform == 'windows-latest'
  env:
    WINDOWS_CERTIFICATE: ${{ secrets.WINDOWS_CERTIFICATE }}
    WINDOWS_CERTIFICATE_PASSWORD: ${{ secrets.WINDOWS_CERTIFICATE_PASSWORD }}
  run: |
    New-Item -ItemType directory -Path certificate
    Set-Content -Path certificate/tempCert.txt -Value $env:WINDOWS_CERTIFICATE
    certutil -decode certificate/tempCert.txt certificate/certificate.pfx
    Remove-Item -path certificate -include tempCert.txt
    Import-PfxCertificate -FilePath certificate/certificate.pfx -CertStoreLocation Cert:\CurrentUser\My -Password (ConvertTo-SecureString -String $env:WINDOWS_CERTIFICATE_PASSWORD -Force -AsPlainText)

----------------------------------------

TITLE: Using Process Plugin in JavaScript
DESCRIPTION: Demonstrates how to exit or relaunch the application using the Process plugin in a JavaScript-based Tauri application.

LANGUAGE: javascript
CODE:
import { exit, relaunch } from '@tauri-apps/plugin-process';
await exit(0);
await relaunch();