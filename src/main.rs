#![windows_subsystem = "windows"]

mod typingspeed;
use std::time::{Duration};

use gamesense::{
    client::GameSenseClient,
    handler::screen::{self, ScreenHandler},
};
use serde_json::json;
use trayicon::{MenuBuilder, TrayIcon, TrayIconBuilder};
use typingspeed::TypingSpeedTracker;
use winit::{event::{ElementState, Event, KeyboardInput}, event_loop::{ControlFlow, EventLoop, EventLoopProxy}};

const GAME_NAME: &str = "TYPING_SPEED";
const GAME_DISPLAY_NAME: &str = "Typing Speed Display";
const DEVELOPER: &str = "Anton Ekström";
const EVENT_NAME_SPEED: &str = "EVENT";
const OLED_TIMEOUT_MILLIS: isize = 7000;
const TYPING_SPEED_TIMEFRAME_SECONDS: u64 = 15;

#[derive(Clone, Eq, PartialEq, Debug)]
enum TrayIconEvent {
    Exit
}

fn main() {
    let mut tracker = TypingSpeedTracker::new(Duration::from_millis(OLED_TIMEOUT_MILLIS as u64));
    let (mut client, screen_handler) = init_gamesense();

    let event_loop = EventLoop::<TrayIconEvent>::with_user_event();
    let proxy = event_loop.create_proxy();

    let tray_icon = build_tray_icon(proxy);

    start_client(&mut client, screen_handler);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Move the tray_icon to the main loop (even if you don't use it)
        //
        // Tray icon uses normal message pump from winit, for orderly closure
        // and removal of the tray icon when you exit it must be moved inside.
        let _ = tray_icon;

        match event {
            Event::DeviceEvent { event, .. } => {
                if let winit::event::DeviceEvent::Key(KeyboardInput { state, .. }) = event {
                    if state == ElementState::Pressed {
                        tracker.press_key();
                        
                        let timespan = Duration::from_secs(TYPING_SPEED_TIMEFRAME_SECONDS);
                        send_wpm(&mut client, tracker.typing_speed_within_timespan(timespan).wpm() as isize);
                    }
                }
            }
            Event::UserEvent(e) => match e {
                TrayIconEvent::Exit => {
                    *control_flow = ControlFlow::Exit;
                    stop_client(&mut client);
                }
            },
            _ => (),
        }
    });
}

fn build_tray_icon(proxy: EventLoopProxy<TrayIconEvent>) -> TrayIcon<TrayIconEvent> {
    return TrayIconBuilder::new()
        .sender_winit(proxy)
        .icon_from_buffer(include_bytes!("../goblin.ico"))
        .tooltip(GAME_DISPLAY_NAME)
        .menu(MenuBuilder::new().item("Stop", TrayIconEvent::Exit))
        .build()
        .unwrap();
}

fn send_wpm(client: &mut GameSenseClient, wpm: isize) {
    client
        .trigger_event_frame(EVENT_NAME_SPEED, wpm, json!({ "wpm": wpm }))
        .unwrap();
}

fn start_client(client: &mut GameSenseClient, handler: ScreenHandler) {
    client
        .bind_event(EVENT_NAME_SPEED, None, None, None, None, vec![handler])
        .unwrap();
    client.start_heartbeat();
}

fn stop_client(client: &mut GameSenseClient) {
    client.stop_heartbeat().unwrap();
}

fn init_gamesense() -> (GameSenseClient, ScreenHandler) {
    let client = GameSenseClient::new(GAME_NAME, GAME_DISPLAY_NAME, DEVELOPER, None).unwrap();

    let handler = screen::ScreenHandler::new(
        "screened",
        "one",
        screen::ScreenDataDefinition::StaticScreenDataDefinition(
            screen::StaticScreenDataDefinition(vec![screen::ScreenFrameData::MultiLineFrameData(
                screen::MultiLineFrameData {
                    frame_modifiers_data: Some(screen::FrameModifiersData {
                        length_millis: Some(OLED_TIMEOUT_MILLIS),
                        icon_id: Some(screen::Icon::None),
                        repeats: None,
                    }),
                    lines: vec![screen::LineData {
                        type_options: screen::LineDataType::TextModifiersData(
                            screen::TextModifiersData {
                                has_text: true,
                                prefix: None,
                                suffix: Some(String::from(" WPM")),
                                bold: None,
                                wrap: None,
                            },
                        ),
                        data_accessor_data: Some(screen::DataAccessorData {
                            arg: None,
                            context_frame_key: Some(String::from("wpm")),
                        }),
                    }],
                },
            )]),
        ),
    );

    return (client, handler);
}
