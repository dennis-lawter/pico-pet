use crate::game::color::Rgb332;
use crate::game::display::render;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
// use crate::game::nvm::settings::SettingType;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

pub struct StatScene {
    next_scene: Option<SceneType>,
}

impl Default for StatScene {
    fn default() -> Self {
        Self { next_scene: None }
    }
}

impl SceneBehavior for StatScene {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();

        if input.get_state(&KeyNames::Back).just_released {
            self.next_scene = Some(SceneType::Main);
        }
    }

    fn tick(&mut self) {}

    fn sound(&mut self) {}

    fn draw(&mut self) {
        let nvm = crate::game::globals::get_nvm();
        // let hardware = crate::game::globals::get_hardware();

        text_writer::full_dialog_box("Stats", "");

        let curr_hp = nvm.pet.get_health();
        let max_hp = nvm.pet.get_max_health();

        let mut height_offset = 24;

        {
            let hp_text = fixedstr::str_format!(fixedstr::str12, "HP: {} / {}", curr_hp, max_hp);
            text_writer::draw_text(8, height_offset, FontStyle::Small, Rgb332::BLACK, &hp_text);
        }

        height_offset += 8;

        {
            let full_hp_bar_width = LCD_WIDTH - 16;
            let hp_bar_width = curr_hp as usize * full_hp_bar_width / max_hp as usize;
            render::solid_line_rect(8, height_offset, hp_bar_width, 8, Rgb332::BLACK);
            render::fill_rect(8, height_offset, hp_bar_width, 8, Rgb332::RED);
        }

        height_offset += 8;

        {
            let header = "Last fed";
            text_writer::draw_text(8, height_offset, FontStyle::Small, Rgb332::BLACK, header);
        }

        height_offset += 8;

        {
            let last_fed = nvm.pet.get_last_fed_date();
            let feed_deadline = nvm.settings.get_feeding_deadline();
            let data = fixedstr::str_format!(
                fixedstr::str24,
                "{}-{}-{} {}:{:02}:{:02}",
                last_fed.year_since_2k as u16 + 2000,
                last_fed.month,
                last_fed.day_of_month,
                feed_deadline.0,
                feed_deadline.1,
                0
            );
            text_writer::draw_text(8, height_offset, FontStyle::Small, Rgb332::BLACK, &data);
        }

        height_offset += 8;

        {
            let header = "Next feeding";
            text_writer::draw_text(8, height_offset, FontStyle::Small, Rgb332::BLACK, header);
        }

        height_offset += 8;

        {
            let last_fed = nvm.pet.get_feeding_deadline();
            let data = fixedstr::str_format!(
                fixedstr::str24,
                "{}-{}-{} {}:{:02}:{:02}",
                last_fed.date.year_since_2k as u16 + 2000,
                last_fed.date.month,
                last_fed.date.day_of_month,
                last_fed.time.hr,
                last_fed.time.min,
                last_fed.time.sec,
            );
            text_writer::draw_text(8, height_offset, FontStyle::Small, Rgb332::BLACK, &data);
        }

        height_offset += 8;

        {
            let header = "Now";
            text_writer::draw_text(8, height_offset, FontStyle::Small, Rgb332::BLACK, header);
        }

        height_offset += 8;

        {
            let now = crate::game::globals::get_hardware().get_date_time();
            let data = fixedstr::str_format!(
                fixedstr::str24,
                "{}-{}-{} {}:{:02}:{:02}",
                now.date.year_since_2k as u16 + 2000,
                now.date.month,
                now.date.day_of_month,
                now.time.hr,
                now.time.min,
                now.time.sec,
            );
            text_writer::draw_text(8, height_offset, FontStyle::Small, Rgb332::BLACK, &data);
        }

        height_offset += 8;

        {
            let header = "Time until next feeding";
            text_writer::draw_text(8, height_offset, FontStyle::Small, Rgb332::BLACK, header);
        }

        height_offset += 8;

        {
            let now = crate::game::globals::get_hardware().get_date_time();
            let feeding_deadline = nvm.pet.get_feeding_deadline();
            let diff_secs = feeding_deadline - now;

            let data = fixedstr::str_format!(
                fixedstr::str12,
                "{}:{:02}:{:02}",
                diff_secs / (60 * 60),
                (diff_secs / 60) % 60,
                diff_secs % 60
            );
            text_writer::draw_text(8, height_offset, FontStyle::Small, Rgb332::BLACK, &data);
        }

        // height_offset += 8;

        // {
        //     let time = hardware.get_time();
        //     let curr_hr = time.hr;
        //     let curr_min = time.min;
        //     let feeding_deadline_hr = nvm
        //         .settings
        //         .get_setting(SettingType::FeedingDeadlineHour)
        //         .get_value();
        //     let feeding_deadline_min = nvm
        //         .settings
        //         .get_setting(SettingType::FeedingDeadlineMinute)
        //         .get_value();
        //     let feeding_deadline_is_after_midnight = true; // TODO: testing

        //     // create hours_to_feeding_deadline and minutes_to_feeding_deadline
        //     if feeding_deadline_is_after_midnight {
        //         let day_in_minutes = 24 * 60;
        //         let minutes_from_midnight_to_feeding_deadline =
        //             feeding_deadline_hr as i16 * 60 + feeding_deadline_min as i16;
        //         let minutes_from_now_to_midnight =
        //             day_in_minutes - (curr_hr as i16 * 60 + curr_min as i16);
        //     }
        // }
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
