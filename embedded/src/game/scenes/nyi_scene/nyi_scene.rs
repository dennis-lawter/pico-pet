use crate::game::color::Rgb332;
use crate::game::display::text_writer;
use crate::game::display::text_writer::draw_text;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::input::KeyNames;
use crate::game::hardware::rtc::RealDate;
use crate::game::hardware::rtc::RealDateTime;
use crate::game::hardware::rtc::RealTime;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

pub struct NyiScene {
    next_scene: Option<SceneType>,
}

impl SceneBehavior for NyiScene {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.next_scene = Some(SceneType::Main);
        }
    }

    fn tick(&mut self) {
        ()
    }

    fn sound(&mut self) {
        let hardware = crate::game::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        text_writer::full_dialog_box("NOT IMPL", "");

        let input = crate::game::globals::get_input();
        if !input.get_state(&KeyNames::Clock).just_released {
            // limit checks to 1hz
            // return;
        }
        let nvm = crate::game::globals::get_nvm();
        nvm.pet.is_hungry = false;
        let now = crate::game::globals::get_hardware().get_date_time();
        let time_hr = now.time.hr;
        let time_min = now.time.min;
        let (feeding_deadline_hr, feeding_deadline_min) = nvm.settings.get_feeding_deadline();
        let (fed_day, fed_mon, fed_yr) = nvm.pet.get_last_fed_date();
        let next_feed_day = fed_day + 1; // TODO: wrap around.....
        let next_feed_mon = fed_mon;
        let next_feed_yr = fed_yr;

        let last_fed = RealDateTime {
            date: RealDate {
                year_since_2k: fed_yr,
                month: fed_mon,
                day_of_month: fed_day,
                day_of_week: 1, // doesn't matter
            },
            time: RealTime {
                hr: feeding_deadline_hr,
                min: feeding_deadline_min,
                sec: 0,
            },
        };
        let mut next_feed = last_fed.clone();
        next_feed.date.add_day();

        // if now.date.year_since_2k > next_feed_yr {
        //     nvm.pet.is_hungry = true;
        // } else if now.date.month > next_feed_mon {
        //     nvm.pet.is_hungry = true;
        // } else if now.date.day_of_month > next_feed_day {
        //     nvm.pet.is_hungry = true;
        // } else if now.time.hr > feeding_deadline_hr {
        //     nvm.pet.is_hungry = true;
        // } else if now.time.min > feeding_deadline_min {
        //     nvm.pet.is_hungry = true;
        // } else if now.date.year_since_2k == next_feed_yr {
        //     if now.date.month == next_feed_mon {
        //         if now.date.day_of_month == next_feed_day {
        //             if now.time.hr == feeding_deadline_hr {
        //                 if now.time.min == feeding_deadline_min {
        //                     // It's time to feed
        //                     // TODO: this is a test to confirm the logic
        //                     nvm.pet.is_hungry = true;
        //                 }
        //             }
        //         }
        //     }
        // }

        nvm.pet.is_hungry = now > next_feed;

        let mut y = 16;
        draw_text(8, y, FontStyle::Small, Rgb332::BLACK, "NOW:");
        y += 8;
        draw_text(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(
                fixedstr::str32,
                "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                now.date.year_since_2k as u16 + 2000,
                now.date.month,
                now.date.day_of_month,
                now.time.hr,
                now.time.min,
                now.time.sec
            ),
        );
        y += 8;
        draw_text(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(fixedstr::str32, "{}", now.to_y2k_epoch()),
        );

        // y += 8;
        // draw_text(8, y, FontStyle::Small, Rgb332::BLACK, "LAST FED:");
        // y += 8;
        // draw_text(
        //     8,
        //     y,
        //     FontStyle::Small,
        //     Rgb332::BLACK,
        //     &fixedstr::str_format!(
        //         fixedstr::str32,
        //         "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        //         fed_yr as u16 + 2000,
        //         fed_mon,
        //         fed_day,
        //         feeding_deadline_hr,
        //         feeding_deadline_min,
        //         0
        //     ),
        // );
        // y += 8;
        // draw_text(
        //     8,
        //     y,
        //     FontStyle::Small,
        //     Rgb332::BLACK,
        //     &fixedstr::str_format!(fixedstr::str32, "{}", last_fed.to_y2k_epoch()),
        // );

        y += 8;
        draw_text(8, y, FontStyle::Small, Rgb332::BLACK, "NEXT FEED DEADLINE:");
        y += 8;
        draw_text(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(
                fixedstr::str32,
                "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                next_feed_yr as u16 + 2000,
                next_feed_mon,
                next_feed_day,
                feeding_deadline_hr,
                feeding_deadline_min,
                0
            ),
        );
        y += 8;
        draw_text(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(fixedstr::str32, "{}", next_feed.to_y2k_epoch()),
        );

        y += 8;
        draw_text(8, y, FontStyle::Small, Rgb332::BLACK, "HUNGRY?:");
        y += 8;
        draw_text(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(
                fixedstr::str32,
                "{}",
                if nvm.pet.is_hungry { "YES" } else { "NO" }
            ),
        )
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
impl Default for NyiScene {
    fn default() -> Self {
        Self { next_scene: None }
    }
}
