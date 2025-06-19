use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc, Weekday, Local};
use std::collections::HashMap;
use rrule::{Frequency, RRule, RRuleSet};
use crate::models::{Task, TaskFrequency};

#[derive(Debug, Clone, serde::Serialize)]
pub struct CalendarDay {
    pub date: NaiveDate,
    pub tasks: Vec<Task>,
    pub is_today: bool,
    pub is_current_month: bool,
    pub is_current_week: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CalendarWeek {
    pub days: Vec<CalendarDay>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CalendarMonth {
    pub year: i32,
    pub month: u32,
    pub weeks: Vec<CalendarWeek>,
}

pub struct CalendarService;

impl CalendarService {
    pub fn generate_month_view(
        year: i32,
        month: u32,
        tasks: Vec<Task>,
    ) -> CalendarMonth {
        let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let last_day = Self::last_day_of_month(year, month);

        // Group tasks by date
        let mut tasks_by_date: HashMap<NaiveDate, Vec<Task>> = HashMap::new();

        for task in tasks {
            // Add original task
            if let Some(due_date) = task.due_date {
                let date = due_date.date_naive();
                tasks_by_date.entry(date).or_insert_with(Vec::new).push(task.clone());
            }

            // Generate recurring instances
        }

        // Generate calendar grid
        let mut weeks = Vec::new();
        let mut current_date = Self::start_of_calendar_grid(first_day);
        let today = Local::now().naive_local().date();

        // Calculate current week boundaries
        let current_week_start = Self::start_of_week(today);
        let current_week_end = current_week_start + Duration::days(6);

        while current_date <= Self::end_of_calendar_grid(last_day) {
            let mut week = Vec::new();

            for _ in 0..7 {
                let tasks = tasks_by_date.get(&current_date).cloned().unwrap_or_default();

                week.push(CalendarDay {
                    date: current_date,
                    tasks,
                    is_today: current_date == today,
                    is_current_month: current_date.month() == month,
                    is_current_week: current_date >= current_week_start && current_date <= current_week_end,
                });

                current_date += Duration::days(1);
            }

            weeks.push(CalendarWeek { days: week });
        }

        CalendarMonth { year, month, weeks }
    }
    
    fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
        if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() - Duration::days(1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() - Duration::days(1)
        }
    }

    fn start_of_week(date: NaiveDate) -> NaiveDate {
        let days_from_sunday = date.weekday().num_days_from_sunday();
        date - Duration::days(days_from_sunday as i64)
    }

    fn start_of_calendar_grid(first_day: NaiveDate) -> NaiveDate {
        let days_from_sunday = first_day.weekday().num_days_from_sunday();
        first_day - Duration::days(days_from_sunday as i64)
    }

    fn end_of_calendar_grid(last_day: NaiveDate) -> NaiveDate {
        let days_until_saturday = 6 - last_day.weekday().num_days_from_sunday();
        last_day + Duration::days(days_until_saturday as i64)
    }
}