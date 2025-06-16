use chrono::{Local, SecondsFormat};
use log::{error, info, warn, LevelFilter};
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::ffi::OsStr;
use std::io::Write;
use std::sync::{LazyLock, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    fn GetProcessForPID(pid: i32, psn: *mut ProcessSerialNumber) -> i32;
    fn _CGSDefaultConnection() -> i32;
    fn CGSEventIsAppUnresponsive(conn: i32, psn: *const ProcessSerialNumber) -> bool;
}

static SYSTEM: LazyLock<Mutex<System>> = LazyLock::new(|| {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()),
    );
    Mutex::new(system)
});

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct ProcessSerialNumber {
    highLongOfPSN: u32,
    lowLongOfPSN: u32,
}

fn get_pid() -> Option<u32> {
    let mut system = SYSTEM.lock().unwrap();
    system.refresh_processes(ProcessesToUpdate::All, true);
    let process_name = OsStr::new("CursorUIViewService");
    let mut processes = system.processes_by_exact_name(process_name);
    processes.next().map(|p| p.pid().as_u32())
}

fn is_process_unresponsive(pid: u32) -> bool {
    let mut psn = ProcessSerialNumber {
        highLongOfPSN: 0,
        lowLongOfPSN: 0,
    };
    let result = unsafe { GetProcessForPID(pid as i32, &mut psn) };
    if result != 0 {
        error!("获取进程 ID 时出错");
        return false;
    }
    info!("{:?}", psn);
    unsafe { CGSEventIsAppUnresponsive(_CGSDefaultConnection(), &psn) }
}

fn kill_process(pid: u32) {
    let _ = signal::kill(Pid::from_raw(pid as i32), Signal::SIGKILL);
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            let render = level_style.render();
            let reset = level_style.render_reset();
            writeln!(
                buf,
                "{render}[{} {} line:{}] {}{reset}",
                Local::now().to_rfc3339_opts(SecondsFormat::Millis, false),
                record.level(),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    loop {
        match get_pid() {
            Some(pid) => {
                if is_process_unresponsive(pid) {
                    warn!("检测到未响应进程 {}", pid);
                    kill_process(pid);
                } else {
                    info!("检测到响应进程 {}", pid);
                }
            }
            _ => error!("获取进程 ID 时出错2"),
        }
        thread::sleep(Duration::from_secs(60));
    }
}
