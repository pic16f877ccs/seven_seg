#[cfg(not(target_os = "linux"))]
compile_error!("only linux is supported");
use crate::Color::*;
use async_std::task::sleep;
use cattocol::by_lines;
use seven_seg::sevseg_four;
use std::time::Duration;
use txtframe::{Color, FrameVar, TextFrame};

#[async_std::main]
async fn main() {
    print!("\x1B[2J\x1B[1;1H\n\x1b[s");
    let main_frame = TextFrame::new()
        .frame_var(&FrameVar::Heavy)
        .expand(2)
        .fill('░')
        .color_fill(Yellow)
        .color_fra(Blue);

    let freq_frame = TextFrame::new()
        .frame_var(&FrameVar::Heavy)
        .color_fra(Blue)
        .color_txt(Red);

    let mut label_frame = TextFrame::new().color_fra(Cyan).color_txt(Green);

    loop {
        let mut cores_freq = String::new();
        let mut frame_core_freq = String::new();
        let mut col_lines = 0;

        for cur_cpu_freq in cpu_freq::get().iter().enumerate().take(12) {
            let core_freq = sevseg_four(&(cur_cpu_freq.1.cur.unwrap() as u16).to_string()).unwrap();
            let freq = freq_frame.frame_iterln(&core_freq);
            let mut tmp_cores_freq = String::new();
            label_frame.set_width(25);

            if cur_cpu_freq.0 > 2 {
                col_lines += 1;

                frame_core_freq.push_str(
                    &freq
                        .chain(
                            label_frame
                                .frame_iterln(&format!("Cpu core: {} frequency", cur_cpu_freq.0)),
                        )
                        .collect::<String>(),
                );

                if col_lines == 3 {
                    tmp_cores_freq
                        .push_str(&by_lines(&cores_freq, &frame_core_freq).collect::<String>());
                    frame_core_freq.clear();
                    col_lines = 0;
                    cores_freq.clear();
                    cores_freq.push_str(&tmp_cores_freq);
                }
            } else {
                cores_freq.push_str(
                    &freq
                        .chain(
                            label_frame
                                .frame_iterln(&format!("Cpu core: {} frequency", cur_cpu_freq.0)),
                        )
                        .collect::<String>(),
                );
            }
        }

        print!("{}\x1b[u", main_frame.frame_iter_esc(&cores_freq).collect::<String>());

        sleep(Duration::from_millis(500)).await;
    }
}
