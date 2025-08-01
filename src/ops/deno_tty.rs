// Copyright 2018-2025 the Deno authors. MIT license.

#[cfg(unix)]
use std::cell::RefCell;
#[cfg(unix)]
use std::collections::HashMap;
use std::io::Error;
#[cfg(windows)]
use std::sync::Arc;

use deno_core::OpState;
#[cfg(unix)]
use deno_core::ResourceId;
use deno_core::op2;
#[cfg(windows)]
use deno_core::parking_lot::Mutex;
use deno_error::JsErrorBox;
use deno_error::JsErrorClass;
use deno_error::builtin_classes::GENERIC_ERROR;
#[cfg(windows)]
use deno_io::WinTtyState;
#[cfg(unix)]
use nix::sys::termios;
use rustyline::Cmd;
use rustyline::Editor;
use rustyline::KeyCode;
use rustyline::KeyEvent;
use rustyline::Modifiers;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;

#[cfg(unix)]
#[derive(Default, Clone)]
struct TtyModeStore(std::rc::Rc<RefCell<HashMap<ResourceId, termios::Termios>>>);

#[cfg(unix)]
impl TtyModeStore {
    pub fn get(&self, id: ResourceId) -> Option<termios::Termios> {
        self.0.borrow().get(&id).map(ToOwned::to_owned)
    }

    pub fn take(&self, id: ResourceId) -> Option<termios::Termios> {
        self.0.borrow_mut().remove(&id)
    }

    pub fn set(&self, id: ResourceId, mode: termios::Termios) {
        self.0.borrow_mut().insert(id, mode);
    }
}

#[cfg(unix)]
use deno_process::JsNixError;
#[cfg(windows)]
use winapi::shared::minwindef::DWORD;
#[cfg(windows)]
use winapi::um::wincon;

deno_core::extension!(
    deno_tty,
    ops = [op_set_raw, op_console_size, op_read_line_prompt],
    state = |state| {
        #[cfg(unix)]
        state.put(TtyModeStore::default());
        #[cfg(not(unix))]
        let _ = state;
    },
);

#[derive(Debug, thiserror::Error, deno_error::JsError)]
pub enum TtyError {
    #[class(inherit)]
    #[error(transparent)]
    Resource(
        #[from]
        #[inherit]
        deno_core::error::ResourceError,
    ),
    #[class(inherit)]
    #[error("{0}")]
    Io(
        #[from]
        #[inherit]
        Error,
    ),
    #[cfg(unix)]
    #[class(inherit)]
    #[error(transparent)]
    Nix(#[inherit] JsNixError),
    #[class(inherit)]
    #[error(transparent)]
    Other(#[inherit] JsErrorBox),
}

// ref: <https://learn.microsoft.com/en-us/windows/console/setconsolemode>
#[cfg(windows)]
const COOKED_MODE: DWORD =
    // enable line-by-line input (returns input only after CR is read)
    wincon::ENABLE_LINE_INPUT
  // enables real-time character echo to console display (requires ENABLE_LINE_INPUT)
  | wincon::ENABLE_ECHO_INPUT
  // system handles CTRL-C (with ENABLE_LINE_INPUT, also handles BS, CR, and LF) and other control keys (when using `ReadFile` or `ReadConsole`)
  | wincon::ENABLE_PROCESSED_INPUT;

#[cfg(windows)]
fn mode_raw_input_on(original_mode: DWORD) -> DWORD {
    original_mode & !COOKED_MODE | wincon::ENABLE_VIRTUAL_TERMINAL_INPUT
}

#[cfg(windows)]
fn mode_raw_input_off(original_mode: DWORD) -> DWORD {
    original_mode & !wincon::ENABLE_VIRTUAL_TERMINAL_INPUT | COOKED_MODE
}

#[op2(fast)]
fn op_set_raw(state: &mut OpState, rid: u32, is_raw: bool, cbreak: bool) -> Result<(), TtyError> {
    let handle_or_fd = state.resource_table.get_fd(rid)?;

    // From https://github.com/kkawakam/rustyline/blob/master/src/tty/windows.rs
    // and https://github.com/kkawakam/rustyline/blob/master/src/tty/unix.rs
    // and https://github.com/crossterm-rs/crossterm/blob/e35d4d2c1cc4c919e36d242e014af75f6127ab50/src/terminal/sys/windows.rs
    // Copyright (c) 2015 Katsu Kawakami & Rustyline authors. MIT license.
    // Copyright (c) 2019 Timon. MIT license.
    #[cfg(windows)]
    {
        use deno_error::JsErrorBox;
        use winapi::shared::minwindef::FALSE;
        use winapi::um::consoleapi;

        let handle = handle_or_fd;

        if cbreak {
            return Err(TtyError::Other(JsErrorBox::not_supported()));
        }

        let mut original_mode: DWORD = 0;
        // SAFETY: winapi call
        if unsafe { consoleapi::GetConsoleMode(handle, &mut original_mode) } == FALSE {
            return Err(TtyError::Io(Error::last_os_error()));
        }

        let new_mode = if is_raw {
            mode_raw_input_on(original_mode)
        } else {
            mode_raw_input_off(original_mode)
        };

        let stdin_state = state.borrow::<Arc<Mutex<WinTtyState>>>();
        let mut stdin_state = stdin_state.lock();

        if stdin_state.reading {
            let cvar = stdin_state.cvar.clone();

            /* Trick to unblock an ongoing line-buffered read operation if not already pending.
            See https://github.com/libuv/libuv/pull/866 for prior art */
            if original_mode & COOKED_MODE != 0 && !stdin_state.cancelled {
                // SAFETY: Write enter key event to force the console wait to return.
                let record = unsafe {
                    let mut record: wincon::INPUT_RECORD = std::mem::zeroed();
                    record.EventType = wincon::KEY_EVENT;
                    record.Event.KeyEvent_mut().wVirtualKeyCode =
                        winapi::um::winuser::VK_RETURN as u16;
                    record.Event.KeyEvent_mut().bKeyDown = 1;
                    record.Event.KeyEvent_mut().wRepeatCount = 1;
                    *record.Event.KeyEvent_mut().uChar.UnicodeChar_mut() = '\r' as u16;
                    record.Event.KeyEvent_mut().dwControlKeyState = 0;
                    record.Event.KeyEvent_mut().wVirtualScanCode =
                        winapi::um::winuser::MapVirtualKeyW(
                            winapi::um::winuser::VK_RETURN as u32,
                            winapi::um::winuser::MAPVK_VK_TO_VSC,
                        ) as u16;
                    record
                };
                stdin_state.cancelled = true;

                // SAFETY: winapi call to open conout$ and save screen state.
                let active_screen_buffer = unsafe {
                    /* Save screen state before sending the VK_RETURN event */
                    let handle = winapi::um::fileapi::CreateFileW(
                        "conout$"
                            .encode_utf16()
                            .chain(Some(0))
                            .collect::<Vec<_>>()
                            .as_ptr(),
                        winapi::um::winnt::GENERIC_READ | winapi::um::winnt::GENERIC_WRITE,
                        winapi::um::winnt::FILE_SHARE_READ | winapi::um::winnt::FILE_SHARE_WRITE,
                        std::ptr::null_mut(),
                        winapi::um::fileapi::OPEN_EXISTING,
                        0,
                        std::ptr::null_mut(),
                    );

                    let mut active_screen_buffer = std::mem::zeroed();
                    winapi::um::wincon::GetConsoleScreenBufferInfo(
                        handle,
                        &mut active_screen_buffer,
                    );
                    winapi::um::handleapi::CloseHandle(handle);
                    active_screen_buffer
                };
                stdin_state.screen_buffer_info = Some(active_screen_buffer);

                // SAFETY: winapi call to write the VK_RETURN event.
                if unsafe { winapi::um::wincon::WriteConsoleInputW(handle, &record, 1, &mut 0) }
                    == FALSE
                {
                    return Err(TtyError::Io(Error::last_os_error()));
                }

                /* Wait for read thread to acknowledge the cancellation to ensure that nothing
                interferes with the screen state.
                NOTE: `wait_while` automatically unlocks stdin_state */
                cvar.wait_while(&mut stdin_state, |state: &mut WinTtyState| state.cancelled);
            }
        }

        // SAFETY: winapi call
        if unsafe { consoleapi::SetConsoleMode(handle, new_mode) } == FALSE {
            return Err(TtyError::Io(Error::last_os_error()));
        }

        Ok(())
    }
    #[cfg(unix)]
    {
        fn prepare_stdio() {
            // SAFETY: Save current state of stdio and restore it when we exit.
            unsafe {
                use libc::atexit;
                use libc::tcgetattr;
                use libc::tcsetattr;
                use libc::termios;
                use once_cell::sync::OnceCell;

                // Only save original state once.
                static ORIG_TERMIOS: OnceCell<Option<termios>> = OnceCell::new();
                ORIG_TERMIOS.get_or_init(|| {
                    let mut termios = std::mem::zeroed::<termios>();
                    if tcgetattr(libc::STDIN_FILENO, &mut termios) == 0 {
                        extern "C" fn reset_stdio() {
                            // SAFETY: Reset the stdio state.
                            unsafe {
                                tcsetattr(
                                    libc::STDIN_FILENO,
                                    0,
                                    &ORIG_TERMIOS.get().unwrap().unwrap(),
                                )
                            };
                        }

                        atexit(reset_stdio);
                        return Some(termios);
                    }

                    None
                });
            }
        }

        prepare_stdio();
        let tty_mode_store = state.borrow::<TtyModeStore>().clone();
        let previous_mode = tty_mode_store.get(rid);

        // SAFETY: Nix crate requires value to implement the AsFd trait
        let raw_fd = unsafe { std::os::fd::BorrowedFd::borrow_raw(handle_or_fd) };

        if is_raw {
            let mut raw = match previous_mode {
                Some(mode) => mode,
                None => {
                    // Save original mode.
                    let original_mode =
                        termios::tcgetattr(raw_fd).map_err(|e| TtyError::Nix(JsNixError(e)))?;
                    tty_mode_store.set(rid, original_mode.clone());
                    original_mode
                }
            };

            raw.input_flags &= !(termios::InputFlags::BRKINT
                | termios::InputFlags::ICRNL
                | termios::InputFlags::INPCK
                | termios::InputFlags::ISTRIP
                | termios::InputFlags::IXON);

            raw.control_flags |= termios::ControlFlags::CS8;

            raw.local_flags &= !(termios::LocalFlags::ECHO
                | termios::LocalFlags::ICANON
                | termios::LocalFlags::IEXTEN);
            if !cbreak {
                raw.local_flags &= !(termios::LocalFlags::ISIG);
            }
            raw.control_chars[termios::SpecialCharacterIndices::VMIN as usize] = 1;
            raw.control_chars[termios::SpecialCharacterIndices::VTIME as usize] = 0;
            termios::tcsetattr(raw_fd, termios::SetArg::TCSADRAIN, &raw)
                .map_err(|e| TtyError::Nix(JsNixError(e)))?;
        } else {
            // Try restore saved mode.
            if let Some(mode) = tty_mode_store.take(rid) {
                termios::tcsetattr(raw_fd, termios::SetArg::TCSADRAIN, &mode)
                    .map_err(|e| TtyError::Nix(JsNixError(e)))?;
            }
        }

        Ok(())
    }
}

#[op2(fast)]
fn op_console_size(state: &mut OpState, #[buffer] result: &mut [u32]) -> Result<(), TtyError> {
    fn check_console_size(
        state: &mut OpState,
        result: &mut [u32],
        rid: u32,
    ) -> Result<(), TtyError> {
        let fd = state.resource_table.get_fd(rid)?;
        let size = console_size_from_fd(fd)?;
        result[0] = size.cols;
        result[1] = size.rows;
        Ok(())
    }

    let mut last_result = Ok(());
    // Since stdio might be piped we try to get the size of the console for all
    // of them and return the first one that succeeds.
    for rid in [0, 1, 2] {
        last_result = check_console_size(state, result, rid);
        if last_result.is_ok() {
            return last_result;
        }
    }

    last_result
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConsoleSize {
    pub cols: u32,
    pub rows: u32,
}

pub fn console_size(std_file: &std::fs::File) -> Result<ConsoleSize, std::io::Error> {
    #[cfg(windows)]
    {
        use std::os::windows::io::AsRawHandle;
        let handle = std_file.as_raw_handle();
        console_size_from_fd(handle)
    }
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let fd = std_file.as_raw_fd();
        console_size_from_fd(fd)
    }
}

#[cfg(windows)]
fn console_size_from_fd(
    handle: std::os::windows::io::RawHandle,
) -> Result<ConsoleSize, std::io::Error> {
    // SAFETY: winapi calls
    unsafe {
        let mut bufinfo: winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();

        if winapi::um::wincon::GetConsoleScreenBufferInfo(handle, &mut bufinfo) == 0 {
            return Err(Error::last_os_error());
        }

        // calculate the size of the visible window
        // * use over/under-flow protections b/c MSDN docs only imply that srWindow components are all non-negative
        // * ref: <https://docs.microsoft.com/en-us/windows/console/console-screen-buffer-info-str> @@ <https://archive.is/sfjnm>
        let cols = std::cmp::max(
            bufinfo.srWindow.Right as i32 - bufinfo.srWindow.Left as i32 + 1,
            0,
        ) as u32;
        let rows = std::cmp::max(
            bufinfo.srWindow.Bottom as i32 - bufinfo.srWindow.Top as i32 + 1,
            0,
        ) as u32;

        Ok(ConsoleSize { cols, rows })
    }
}

#[cfg(not(windows))]
fn console_size_from_fd(fd: std::os::unix::prelude::RawFd) -> Result<ConsoleSize, std::io::Error> {
    // SAFETY: libc calls
    unsafe {
        let mut size: libc::winsize = std::mem::zeroed();
        if libc::ioctl(fd, libc::TIOCGWINSZ, &mut size as *mut _) != 0 {
            return Err(Error::last_os_error());
        }
        Ok(ConsoleSize {
            cols: size.ws_col as u32,
            rows: size.ws_row as u32,
        })
    }
}

#[cfg(all(test, windows))]
mod tests {
    #[test]
    fn test_winos_raw_mode_transitions() {
        use crate::ops::tty::mode_raw_input_off;
        use crate::ops::tty::mode_raw_input_on;

        let known_off_modes = [0xf7 /* Win10/CMD */, 0x1f7 /* Win10/WinTerm */];
        let known_on_modes = [0x2f0 /* Win10/CMD */, 0x3f0 /* Win10/WinTerm */];

        // assert known transitions
        assert_eq!(known_on_modes[0], mode_raw_input_on(known_off_modes[0]));
        assert_eq!(known_on_modes[1], mode_raw_input_on(known_off_modes[1]));

        // assert ON-OFF round-trip is neutral
        assert_eq!(
            known_off_modes[0],
            mode_raw_input_off(mode_raw_input_on(known_off_modes[0]))
        );
        assert_eq!(
            known_off_modes[1],
            mode_raw_input_off(mode_raw_input_on(known_off_modes[1]))
        );
    }
}

deno_error::js_error_wrapper!(ReadlineError, JsReadlineError, |err| {
    match err {
        ReadlineError::Io(e) => e.get_class(),
        ReadlineError::Eof => GENERIC_ERROR.into(),
        ReadlineError::Interrupted => GENERIC_ERROR.into(),
        #[cfg(windows)]
        ReadlineError::Decode(_) => GENERIC_ERROR.into(),
        #[cfg(windows)]
        ReadlineError::SystemError(_) => GENERIC_ERROR.into(),
        _ => GENERIC_ERROR.into(),
    }
});

#[op2]
#[string]
pub fn op_read_line_prompt(
    #[string] prompt_text: &str,
    #[string] default_value: &str,
) -> Result<Option<String>, JsReadlineError> {
    let mut editor =
        Editor::<(), rustyline::history::DefaultHistory>::new().expect("Failed to create editor.");

    editor.set_keyseq_timeout(Some(1));
    editor.bind_sequence(KeyEvent(KeyCode::Esc, Modifiers::empty()), Cmd::Interrupt);

    let read_result = editor.readline_with_initial(prompt_text, (default_value, ""));
    match read_result {
        Ok(line) => Ok(Some(line)),
        Err(ReadlineError::Interrupted) => {
            // SAFETY: Disable raw mode and raise SIGINT.
            unsafe {
                libc::raise(libc::SIGINT);
            }
            Ok(None)
        }
        Err(ReadlineError::Eof) => Ok(None),
        Err(err) => Err(JsReadlineError(err)),
    }
}
